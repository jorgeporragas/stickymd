#!/usr/bin/env python3
"""flow_check — machine checks for FLOW's documentation rules.

Specification: SPEC.md beside this file. Standard library only; Python 3.11+.
Exit codes: 0 no error-severity finding, 1 at least one, 2 the tool could not run.
"""
from __future__ import annotations

import argparse
import fnmatch
import hashlib
import json
import re
import subprocess
import sys
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

TOOL_VERSION = "0.3.0"

# ---------------------------------------------------------------------------
# Defaults. A project overrides any of these in .flow/config.toml (D-005).
# ---------------------------------------------------------------------------

DEFAULTS: dict = {
    "files": {
        "components_dir": "docs/components",
        # One file per ADR (D-020). The ID is read from the file's first heading.
        "adr_dir": "docs/adr",
    },
    "severity": {
        "C3": "error", "C5": "error", "C6": "error",
        "C7": "error", "C8": "error", "C10": "error",
    },
    "component_files": {"max_words": 400},
    "entries": [],
    "decisions": {"entry": r"^#{1,2} ([A-Z]+-\d+)"},
    "references": {
        "names": {
            "FLOW": ".claude/rules/flow.md", "PRD": "docs/PRD.md", "CLAUDE": "CLAUDE.md",
            "COMPLETED": "docs/COMPLETED.md", "DESIGN": "docs/DESIGN.md",
            "SCHEMA": "docs/SCHEMA.md", "FIXES": "docs/FIXES.md", "SETUP": ".flow/SETUP.md",
        },
        "markdown_globs": ["*.md", "docs/**/*.md", ".claude/**/*.md"],
        # ADRs describe the past and are immutable: a reference there may name a section that
        # has since gone, and that is accurate history. Globs.
        "exempt": ["docs/adr/*.md"],
        "code_globs": [],
    },
    "inventory": [],
    "co_change": [],
    "vendor": {"manifest": ".flow/manifest.json"},
}

SIZE_CHECKS = {"C3"}
CHECK_NAMES = {
    "C3": "entry-size", "C5": "references", "C6": "supersession",
    "C7": "inventory", "C8": "commit-checks", "C10": "vendored-copy",
}
# Retired checks keep their IDs, so an old config or command names them clearly instead of
# meaning something new.
RETIRED = {
    "C1": "item notes live in issues since FLOW 3.0 (D-016)",
    "C2": "COMPLETED is no longer written since FLOW 3.0 (D-016)",
    "C4": "truth files carry no Last verified stamp since FLOW 3.0 (D-017)",
    "C9": "item state lives in issues since FLOW 3.0 (D-016)",
}


class ToolError(Exception):
    """The tool cannot run. Exit 2, never a pass."""


# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

def merge(base, over):
    if isinstance(base, dict) and isinstance(over, dict):
        out = dict(base)
        for k, v in over.items():
            out[k] = merge(base[k], v) if k in base else v
        return out
    return over


def load_config(root: Path, path: str | None):
    cfg_path = root / (path or ".flow/config.toml")
    user: dict = {}
    if cfg_path.exists():
        try:
            user = tomllib.loads(cfg_path.read_text(encoding="utf-8"))
        except tomllib.TOMLDecodeError as e:
            raise ToolError(f"{cfg_path}: {e}") from e
    elif path:
        raise ToolError(f"config file not found: {cfg_path}")
    for k in user.get("severity", {}):
        if k in RETIRED:
            raise ToolError(f"severity.{k}: {k} is retired: {RETIRED[k]}. Remove it from the "
                            "config")
    cfg = merge(DEFAULTS, user)
    for k, v in cfg["severity"].items():
        if v not in ("error", "warn", "off"):
            raise ToolError(f"severity.{k} must be error, warn or off, not {v!r}")
    return cfg, (cfg_path if cfg_path.exists() else None)


# ---------------------------------------------------------------------------
# Git
# ---------------------------------------------------------------------------

def git(root: Path, *args: str, check=True) -> str:
    try:
        r = subprocess.run(["git", "-C", str(root), *args], capture_output=True,
                           text=True, encoding="utf-8", errors="replace")
    except FileNotFoundError as e:
        raise ToolError("git is not installed") from e
    if check and r.returncode != 0:
        raise ToolError(f"git {' '.join(args)}: {r.stderr.strip()}")
    return r.stdout


@dataclass
class Changeset:
    """A range's combined diff, or the staged index (the hook's view)."""
    label: str
    changed: dict[str, str] = field(default_factory=dict)  # path -> status letter


def changeset_for_range(root: Path, rev_range: str) -> Changeset:
    """The range as ONE change: what a squash merge puts on the default branch (D-020). A pull
    request that adds a migration in one commit and SCHEMA in the next owes nothing."""
    base, head = (rev_range.split("..", 1) if ".." in rev_range
                  else (f"{rev_range}^", rev_range))
    head = head.lstrip(".") or "HEAD"
    cs = Changeset(label=rev_range)
    for line in git(root, "diff", "--name-status", f"{base}...{head}").splitlines():
        parts = line.split("\t")
        cs.changed[parts[-1]] = parts[0][0]
    return cs


def changeset_staged(root: Path) -> Changeset:
    cs = Changeset(label="staged")
    for line in git(root, "diff", "--cached", "--name-status").splitlines():
        parts = line.split("\t")
        cs.changed[parts[-1]] = parts[0][0]
    return cs


def _show(root: Path, spec: str):
    r = subprocess.run(["git", "-C", str(root), "show", spec], capture_output=True,
                       text=True, encoding="utf-8", errors="replace")
    return r.stdout if r.returncode == 0 else None


def touched_lines(root: Path, rev_range: str | None, path: str) -> set[int] | None:
    """Line numbers changed in the working copy of `path` over the range.
    None means nothing is touched (no range given)."""
    if not rev_range:
        return None
    base = range_base(rev_range)
    diff = git(root, "diff", "-U0", base, "--", path, check=False)
    lines: set[int] = set()
    for m in re.finditer(r"^@@ -\S+ \+(\d+)(?:,(\d+))? @@", diff, re.M):
        start, count = int(m.group(1)), int(m.group(2) or "1")
        lines.update(range(start, start + max(count, 1)))
    return lines


def range_base(rev_range: str) -> str:
    return rev_range.split("..")[0] if ".." in rev_range else f"{rev_range}^"


LINK_TARGET = re.compile(r"\]\([^)]*\)")


def same_text(text: str) -> str:
    """Text compared for a move: whitespace collapsed and link targets dropped, because moving
    an entry into another folder re-wraps it and rewrites its relative links (#24)."""
    return " ".join(LINK_TARGET.sub("]", text).split())


# ---------------------------------------------------------------------------
# Markdown helpers
# ---------------------------------------------------------------------------

def words(text: str) -> int:
    """The spec's definition: str.split(), the same count as `wc -w` in a UTF-8 locale."""
    return len(text.split())


def read(root: Path, rel: str) -> str | None:
    p = root / rel
    return p.read_text(encoding="utf-8", errors="replace") if p.is_file() else None


HEADING = re.compile(r"^(#{1,6})\s+(.*?)\s*#*\s*$")


def strip_fences(lines: list[str]) -> list[str]:
    """Blank out fenced code blocks so headings and links inside them are ignored."""
    out, fence = [], None
    for line in lines:
        m = re.match(r"^\s*(```+|~~~+)", line)
        if m:
            tok = m.group(1)[0] * 3
            if fence is None:
                fence = tok
            elif tok == fence:
                fence = None
            out.append("")
            continue
        out.append("" if fence else line)
    return out


@dataclass
class Entry:
    level: int
    title: str
    start: int  # 1-based line of the heading
    end: int    # 1-based last line
    body: str


def entries(text: str) -> list[Entry]:
    lines = text.splitlines()
    clean = strip_fences(lines)
    heads = []
    for i, line in enumerate(clean):
        m = HEADING.match(line)
        if m:
            heads.append((i, len(m.group(1)), m.group(2)))
    out = []
    for n, (i, lvl, title) in enumerate(heads):
        j = len(lines)
        for k, l2, _ in heads[n + 1:]:
            if l2 <= lvl:
                j = k
                break
        out.append(Entry(lvl, title, i + 1, j, "\n".join(lines[i:j])))
    return out


def norm(text: str) -> str:
    text = re.sub(r"[*_`]", "", text)
    text = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", text)
    return re.sub(r"\s+", " ", text).strip().casefold()


def slug(title: str) -> str:
    """GitHub's heading anchor rule, approximately: lowercase, drop punctuation except
    hyphens and spaces, spaces to hyphens."""
    t = norm(title)
    t = re.sub(r"[^\w\- ]", "", t)
    return t.replace(" ", "-")


# ---------------------------------------------------------------------------
# Report
# ---------------------------------------------------------------------------

@dataclass
class Finding:
    check: str
    file: str
    line: int
    message: str
    measured: object = None
    limit: object = None
    severity: str = "error"

    def as_dict(self):
        d = {"file": self.file, "line": self.line, "message": self.message,
             "severity": self.severity}
        if self.measured is not None:
            d["measured"] = self.measured
        if self.limit is not None:
            d["limit"] = self.limit
        return d


class Ctx:
    def __init__(self, root, cfg, rev_range, staged):
        self.root, self.cfg, self.range, self.staged = root, cfg, rev_range, staged
        self.findings: list[Finding] = []
        self._touched: dict[str, set[int] | None] = {}
        self._base_docs: str | None = None
        self._new: dict[str, bool] = {}

    def moved_unchanged(self, path: str, text: str | None) -> bool:
        """True when `path` did not exist at the range's base and `text` did, word for word, in
        some Markdown file there: an entry moved into its own file, not edited (#24, D-007)."""
        if not self.range or not text or not text.strip():
            return False
        base = range_base(self.range)
        if path not in self._new:
            r = subprocess.run(["git", "-C", str(self.root), "cat-file", "-e", f"{base}:{path}"],
                               capture_output=True)
            self._new[path] = r.returncode != 0
        if not self._new[path]:
            return False
        if self._base_docs is None:
            names = git(self.root, "ls-tree", "-r", "--name-only", base, check=False).split("\n")
            docs = (_show(self.root, f"{base}:{n}") for n in names if n.endswith(".md"))
            self._base_docs = "\0".join(same_text(d) for d in docs if d)
        return same_text(text) in self._base_docs

    def touched(self, path: str, start: int, end: int) -> bool:
        if path not in self._touched:
            self._touched[path] = touched_lines(self.root, self.range, path)
        t = self._touched[path]
        return bool(t) and any(start <= n <= end for n in t)

    def add(self, check, file, line, message, measured=None, limit=None,
            span=None, text=None):
        sev = self.cfg["severity"].get(check, "error")
        if sev == "off":
            return
        if check in SIZE_CHECKS and sev == "error":
            # D-007: only entries the range touched may fail; old debt is a warning.
            # A moved entry is not an edited one: `text` is what the check measured.
            s, e = span or (line, line)
            if not self.touched(file, s, e) or self.moved_unchanged(file, text):
                sev = "warn"
        self.findings.append(Finding(check, file, line, message, measured, limit, sev))


# ---------------------------------------------------------------------------
# Checks
# ---------------------------------------------------------------------------

def c3_entries(ctx: Ctx):
    specs = list(ctx.cfg["entries"])
    # v2.6: one file per component. In separate files an oversized entry looks normal, so each
    # file is an entry with its own limit (D-008).
    cdir = ctx.root / ctx.cfg["files"]["components_dir"]
    climit = ctx.cfg["component_files"].get("max_words")
    if climit and cdir.is_dir():
        for p in sorted(cdir.glob("*.md")):
            rel = p.relative_to(ctx.root).as_posix()
            text = p.read_text(encoding="utf-8", errors="replace")
            body = "\n".join(text.splitlines()[1:]) if text.startswith("#") else text
            n = words(body)
            if n > climit:
                ctx.add("C3", rel, 1, f"component file is {n} words; the limit is {climit}",
                        measured=n, limit=climit, span=(1, len(text.splitlines()) or 1),
                        text=body)
    for spec in specs:
        rel = spec["file"]
        text = read(ctx.root, rel)
        if text is None:
            continue
        es = entries(text)
        under = spec.get("under")
        window = None
        if under:
            parents = [e for e in es if norm(e.title) == norm(under)]
            if not parents:
                ctx.add("C3", rel, 1, f"configured section {under!r} not found")
                continue
            window = (parents[0].start, parents[0].end)
        for e in es:
            if e.level != spec.get("heading_level", 2):
                continue
            if window and not (window[0] < e.start <= window[1]):
                continue
            if spec.get("match") and norm(e.title) != norm(spec["match"]):
                continue
            body = "\n".join(e.body.splitlines()[1:])
            n = words(body)
            if n > spec["max_words"]:
                ctx.add("C3", rel, e.start,
                        f"entry {e.title!r} is {n} words; the limit is {spec['max_words']}",
                        measured=n, limit=spec["max_words"], span=(e.start, e.end))


MD_LINK = re.compile(r"(?<!!)\[[^\]]*\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")


def section_ref_regex(names) -> re.Pattern:
    alt = "|".join(sorted(map(re.escape, names), key=len, reverse=True))
    file_part = rf"(?:\[`?(?P<a>{alt})(?:\.md)?`?\]\([^)]*\)|`?\b(?P<b>{alt})(?:\.md)?`?)"
    return re.compile(file_part + r"(?P<chain>(?:\s*›\s*\*[^*\n]+\*)+)")


def heading_index(root: Path, rel: str, cache: dict):
    if rel not in cache:
        text = read(root, rel)
        if text is None:
            cache[rel] = None
        else:
            es = entries(text)
            titles = [norm(e.title) for e in es]
            # FLOW files name rules in bold at the start of a list item or paragraph, and
            # cross-references point at them as they point at headings.
            titles += [norm(m.group(1)) for m in re.finditer(
                r"^\s*(?:[-*+]\s+|\d+\.\s+)?\*\*(.+?)\*\*", text, re.M)]
            slugs, seen = set(), {}
            for e in es:
                s = slug(e.title)
                k = seen.get(s, 0)
                slugs.add(s if k == 0 else f"{s}-{k}")
                seen[s] = k + 1
            cache[rel] = (titles, slugs)
    return cache[rel]


def heading_exists(titles, wanted: str) -> bool:
    w = norm(wanted)
    for t in titles:
        if t == w:
            return True
        # A reference may omit a trailing qualifier: "Theme-dependent ... colour" for
        # "Theme-dependent ... colour (the sweep's rule)".
        if t.startswith(w) and t[len(w):len(w) + 1] in (" ", "(", ":", "—", "-", ","):
            return True
    return False


def c5_references(ctx: Ctx):
    rc = ctx.cfg["references"]
    names = rc["names"]
    pat = section_ref_regex(names)
    cache: dict = {}
    md_files = sorted({p for g in rc["markdown_globs"] for p in ctx.root.glob(g)
                       if p.is_file() and ".git" not in p.parts})
    code_files = sorted({p for g in rc["code_globs"] for p in ctx.root.glob(g) if p.is_file()})

    def check_sections(rel, lines):
        for i, line in enumerate(lines, 1):
            for m in pat.finditer(line):
                target = names[m.group("a") or m.group("b")]
                idx = heading_index(ctx.root, target, cache)
                if idx is None:
                    ctx.add("C5", rel, i, f"section reference to missing file {target}")
                    continue
                for h in re.findall(r"\*([^*\n]+)\*", m.group("chain")):
                    if not heading_exists(idx[0], h):
                        ctx.add("C5", rel, i, f"{target} has no heading {h!r}")

    for p in md_files:
        rel = p.relative_to(ctx.root).as_posix()
        if any(fnmatch.fnmatch(rel, g) for g in rc["exempt"]):
            continue
        lines = strip_fences(p.read_text(encoding="utf-8", errors="replace").splitlines())
        check_sections(rel, lines)
        # Links written inside inline code are examples, not links: blank them out, except a
        # code span that is itself a link's text, as in [`FIXES.md`](FIXES.md).
        masked = [re.sub(r"`[^`\n]*`(?!\]\()", lambda m: " " * len(m.group(0)), l)
                  for l in lines]
        for i, line in enumerate(masked, 1):
            for m in MD_LINK.finditer(line):
                target = m.group(1)
                if re.match(r"^[a-z][a-z0-9+.-]*:", target, re.I):
                    continue  # http:, mailto:, ...
                path, _, anchor = target.partition("#")
                if path:
                    base = ctx.root if path.startswith("/") else p.parent
                    dest = (base / path.lstrip("/")).resolve()
                    if not dest.exists():
                        ctx.add("C5", rel, i, f"link to missing path {target}")
                        continue
                else:
                    dest = p
                if anchor and dest.suffix == ".md" and dest.is_file():
                    try:
                        drel = dest.relative_to(ctx.root.resolve()).as_posix()
                    except ValueError:
                        continue
                    idx = heading_index(ctx.root, drel, cache)
                    if idx and anchor.lower() not in idx[1]:
                        ctx.add("C5", rel, i, f"link anchor #{anchor} not found in {drel}")
    for p in code_files:
        rel = p.relative_to(ctx.root).as_posix()
        check_sections(rel, p.read_text(encoding="utf-8", errors="replace").splitlines())


def c6_supersession(ctx: Ctx):
    """Supersession pointers agree across the ADR folder, one file per decision (D-020)."""
    d = ctx.root / ctx.cfg["files"]["adr_dir"]
    if not d.is_dir():
        return
    entry_re = re.compile(ctx.cfg["decisions"]["entry"], re.M)
    recs = {}  # id -> (file, line, status, supersedes)
    for p in sorted(d.glob("*.md")):
        text = p.read_text(encoding="utf-8", errors="replace")
        m = entry_re.search(text)
        if not m:
            continue  # a README or template, not a decision
        rel = p.relative_to(ctx.root).as_posix()
        status = re.search(r"^\**Status:\**\s*(.*)$", text, re.M)
        sup = re.search(r"^\**Supersedes:\**\s*(.*)$", text, re.M)
        if m.group(1) in recs:
            ctx.add("C6", rel, 1, f"{m.group(1)} is also {recs[m.group(1)][0]}")
            continue
        recs[m.group(1)] = (rel, text.count("\n", 0, m.start()) + 1,
                            status.group(1) if status else "", sup.group(1).strip() if sup else "")
    idre = re.compile(r"[A-Z]+-\d+")

    def superseded_list(s):
        return [] if s.startswith("—") else idre.findall(s)

    for eid, (rel, line, status, sup) in recs.items():
        for by in re.findall(r"Superseded by ([A-Z]+-\d+)", status):
            if by not in recs:
                ctx.add("C6", rel, line, f"{eid} is superseded by {by}, which does not exist")
            elif eid not in superseded_list(recs[by][3]):
                ctx.add("C6", rel, line, f"{eid} says Superseded by {by}, but {by} has no "
                        f"'Supersedes: {eid}'")
        for old in superseded_list(sup):
            if old not in recs:
                ctx.add("C6", rel, line, f"{eid} supersedes {old}, which does not exist")
            elif f"Superseded by {eid}" not in recs[old][2]:
                ctx.add("C6", rel, line, f"{eid} supersedes {old}, but {old}'s Status does not "
                        f"say 'Superseded by {eid}'")


def c7_inventory(ctx: Ctx):
    for spec in ctx.cfg["inventory"]:
        dp = ctx.root / spec["doc"]
        if dp.is_dir():
            # v2.6: one file per component. A component is named by its file's name or text.
            doc = "\n".join(f"{q.name}\n{q.read_text(encoding='utf-8', errors='replace')}"
                            for q in sorted(dp.glob("*.md")))
        else:
            doc = read(ctx.root, spec["doc"])
        if doc is None:
            raise ToolError(f"inventory doc not found: {spec['doc']}")
        d = ctx.root / spec["dir"]
        if not d.is_dir():
            raise ToolError(f"inventory directory not found: {spec['dir']}")
        files = sorted(d.glob(spec.get("glob", "*")))
        if not files:
            ctx.add("C7", spec["dir"], 1, "inventory directory matches no files: "
                    "a check that scans nothing passes forever")
        for f in files:
            # Accepted spellings of a file's name: the file name, its stem, the stem in
            # PascalCase (the class it usually declares), and the stem as words (a heading).
            parts = [w for w in re.split(r"[_\-]", f.stem) if w]
            pascal = "".join(w[:1].upper() + w[1:] for w in parts)
            spaced = r"\s+".join(map(re.escape, parts))
            named = f.name in doc or re.search(
                rf"\b({re.escape(f.stem)}|{re.escape(pascal)})\b", doc) or \
                re.search(rf"\b{spaced}\b", doc, re.I)
            if f.is_file() and not named:
                ctx.add("C7", spec["doc"], 1,
                        f"{f.relative_to(ctx.root).as_posix()} is not named in the inventory")


def c8_commits(ctx: Ctx):
    if ctx.staged:
        sets = [changeset_staged(ctx.root)]
    elif ctx.range:
        sets = [changeset_for_range(ctx.root, ctx.range)]
    else:
        return
    for cs in sets:
        paths = cs.changed
        for rule in ctx.cfg["co_change"]:
            hits = [p for p, st in paths.items() if fnmatch.fnmatch(p, rule["when"])
                    and (rule.get("on", "any") == "any" or st == "A")]
            if hits:
                req = rule["require"] if isinstance(rule["require"], list) else [rule["require"]]
                if not any(r in paths for r in req):
                    ctx.add("C8", hits[0], 1, f"{cs.label}: {', '.join(hits)} changed without "
                            f"{' or '.join(req)} in the same change")


def c10_vendor(ctx: Ctx):
    rel = ctx.cfg["vendor"]["manifest"]
    text = read(ctx.root, rel)
    if text is None:
        return  # not a vendored project; nothing to verify
    try:
        manifest = json.loads(text)
    except json.JSONDecodeError as e:
        raise ToolError(f"{rel}: {e}") from e
    for path, digest in sorted(manifest.get("files", {}).items()):
        p = ctx.root / path
        if not p.is_file():
            ctx.add("C10", path, 1, f"vendored FLOW file missing (FLOW {manifest.get('version')})")
        elif file_digest(p) != digest:
            ctx.add("C10", path, 1, f"vendored FLOW file was modified; FLOW files change only "
                    f"by adopting a release (pinned: {manifest.get('version')})")


def file_digest(p: Path) -> str:
    # Line endings are normalised so a Windows checkout hashes like the release.
    return hashlib.sha256(p.read_bytes().replace(b"\r\n", b"\n")).hexdigest()


# ---------------------------------------------------------------------------
# Run
# ---------------------------------------------------------------------------

def run(root: Path, config: str | None = None, rev_range: str | None = None,
        staged=False, only=None) -> dict:
    root = root.resolve()
    if git(root, "rev-parse", "--is-inside-work-tree", check=False).strip() != "true":
        raise ToolError(f"{root} is not inside a git work tree")
    cfg, cfg_path = load_config(root, config)
    ctx = Ctx(root, cfg, rev_range, staged)
    plan = {
        "C3": lambda: c3_entries(ctx), "C5": lambda: c5_references(ctx), "C6": lambda: c6_supersession(ctx),
        "C7": lambda: c7_inventory(ctx), "C8": lambda: c8_commits(ctx),
        "C10": lambda: c10_vendor(ctx),
    }
    wanted = only or list(plan)
    retired = [c for c in wanted if c in RETIRED]
    if retired:
        raise ToolError("; ".join(f"{c} is retired: {RETIRED[c]}" for c in retired))
    unknown = [c for c in wanted if c not in plan]
    if unknown:
        raise ToolError(f"unknown check(s): {', '.join(unknown)}")
    checks = []
    for cid in wanted:
        sev = cfg["severity"].get(cid, "error")
        if sev == "off" or (cid == "C8" and not (rev_range or staged)):
            checks.append({"id": cid, "name": CHECK_NAMES[cid], "severity": sev,
                           "status": "skip", "findings": []})
            continue
        before = len(ctx.findings)
        plan[cid]()
        fs = ctx.findings[before:]
        fs.sort(key=lambda x: (x.file, x.line, x.message))
        status = "fail" if any(x.severity == "error" for x in fs) else \
                 ("warn" if fs else "pass")
        checks.append({"id": cid, "name": CHECK_NAMES[cid], "severity": sev,
                       "status": status, "findings": [x.as_dict() for x in fs]})
    result = "fail" if any(c["status"] == "fail" for c in checks) else "pass"
    return {
        "tool": "flow_check", "tool_version": TOOL_VERSION,
        "config": rel_or_abs(cfg_path, root),
        "range": rev_range, "staged": staged, "result": result,
        "checks": checks,
    }


def rel_or_abs(p: Path | None, root: Path):
    if p is None:
        return None
    try:
        return p.resolve().relative_to(root).as_posix()
    except ValueError:
        return p.as_posix()


def human(report: dict, all_findings: bool = False) -> str:
    """Errors one line each; warnings only as counts unless all_findings. Old warnings are
    read at every session start, so listing them costs more than it tells (D-014)."""
    out, hidden = [], 0
    for c in report["checks"]:
        for f in c["findings"]:
            if f["severity"] != "error" and not all_findings:
                hidden += 1
                continue
            tag = "" if f["severity"] == "error" else " (warn)"
            out.append(f"{f['file']}:{f['line']}: [{c['id']}]{tag} {f['message']}")
    if hidden:
        out.append(f"{hidden} warning(s) not listed; --all lists them.")
    out.append("")
    for c in report["checks"]:
        n = len(c["findings"])
        out.append(f"{c['id']:<4} {c['name']:<16} {c['status']:<5} {n} finding(s)")
    out.append(f"result: {report['result']}")
    return "\n".join(out)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(prog="flow_check", description=__doc__.splitlines()[0])
    ap.add_argument("--root", default=".")
    ap.add_argument("--config")
    g = ap.add_mutually_exclusive_group()
    g.add_argument("--range", dest="rev_range")
    g.add_argument("--staged", action="store_true",
                   help="run the commit checks on the staged diff (the pre-commit hook)")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--all", action="store_true", dest="all_findings",
                    help="list warnings one per line too (text output only)")
    ap.add_argument("--only", help="comma-separated check ids, e.g. C3,C6")
    a = ap.parse_args(argv)
    try:
        report = run(Path(a.root), a.config, a.rev_range, a.staged,
                     a.only.split(",") if a.only else None)
    except ToolError as e:
        print(f"flow_check: {e}", file=sys.stderr)
        return 2
    except Exception as e:  # a crash must never read as "findings" (exit 1)
        print(f"flow_check: internal error: {e!r}", file=sys.stderr)
        return 2
    print(json.dumps(report, indent=2, ensure_ascii=False) if a.json else human(report, a.all_findings))
    return 1 if report["result"] == "fail" else 0


if __name__ == "__main__":
    sys.exit(main())
