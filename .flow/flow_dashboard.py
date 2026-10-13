#!/usr/bin/env python3
"""flow_dashboard — the founder's one-page view of a FLOW project, generated, never edited.

    python .flow/flow_dashboard.py [--report REPORT.json] [--out .flow/dashboard.html] [--no-github]

Renders flow_check's report (run fresh unless --report is given) plus, when the GitHub CLI is
signed in, the issues waiting on the founder, open pull requests and CI on the default branch. It shows only what flow_check and
GitHub already compute, so it cannot disagree with CI (D-003, D-004). Standard library only.
The output is a generated file: keep it out of git.
"""
from __future__ import annotations

import argparse
import html
import importlib.util
import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent


def load_flow_check():
    """flow_check sits beside this file in a project (.flow/), or in ../flow_check in FLOW."""
    for p in (HERE / "flow_check.py", HERE.parent / "flow_check" / "flow_check.py"):
        if p.is_file():
            spec = importlib.util.spec_from_file_location("flow_check", p)
            mod = importlib.util.module_from_spec(spec)
            sys.modules["flow_check"] = mod
            spec.loader.exec_module(mod)
            return mod
    raise SystemExit("flow_dashboard: flow_check.py not found beside this file")


def gh(root: Path, *args):
    """GitHub data, or a reason it is missing. Never raises: the page renders without it."""
    try:
        r = subprocess.run(["gh", *args], cwd=root, capture_output=True, text=True, timeout=30)
    except FileNotFoundError:
        return None, "the GitHub CLI (gh) is not installed"
    except subprocess.TimeoutExpired:
        return None, "the GitHub CLI timed out"
    if r.returncode != 0:
        msg = (r.stderr.strip().splitlines() or ["unknown error"])[-1]
        msg = msg.split(". ")[0].rstrip(".")
        return None, f"gh failed: {msg[:140]}{'…' if len(msg) > 140 else ''}"
    try:
        return json.loads(r.stdout), None
    except json.JSONDecodeError:
        return None, "gh returned something that is not JSON"


# The issues the founder acts on, by label (FLOW_SYSTEM › *Issue*). Read through the REST API:
# it is the one gh path that works both locally and in a cloud session, where GraphQL is closed.
WAITING = (("needs-verification", "Waiting for your confirmation"),
           ("founder-only", "Only you can do"),
           ("active", "In progress"))


def issues(root: Path, label: str):
    data, err = gh(root, "api", "-X", "GET", "repos/{owner}/{repo}/issues",
                   "-f", "state=open", "-f", f"labels={label}", "-f", "per_page=100")
    if data is None:
        return None, err
    # The issues endpoint also returns pull requests; they are listed separately.
    return [{"number": i["number"], "title": i["title"], "url": i["html_url"]}
            for i in data if "pull_request" not in i], None


def github(root: Path):
    waiting = {label: issues(root, label) for label, _ in WAITING}
    prs, pr_err = gh(root, "pr", "list", "--state", "open", "--json",
                     "number,title,isDraft,url,statusCheckRollup")
    branch, _ = gh(root, "repo", "view", "--json", "defaultBranchRef")
    default = (branch or {}).get("defaultBranchRef", {}).get("name") or "main"
    runs, run_err = gh(root, "run", "list", "--branch", default, "--limit", "1", "--json",
                       "conclusion,status,workflowName,headSha,displayTitle,url")
    return {"prs": prs, "pr_error": pr_err, "default": default,
            "run": (runs or [None])[0], "run_error": run_err,
            "issues": {k: v[0] for k, v in waiting.items()},
            "issues_error": next((v[1] for v in waiting.values() if v[1]), None)}


def git(root: Path, *args) -> str | None:
    r = subprocess.run(["git", "-C", str(root), *args], capture_output=True, text=True)
    return r.stdout.strip() if r.returncode == 0 else None


def working_copy(root: Path) -> dict:
    """Where this checkout stands: the session-start questions, answered by git alone."""
    branch = git(root, "branch", "--show-current") or "(detached HEAD)"
    default = (git(root, "symbolic-ref", "--short", "refs/remotes/origin/HEAD") or "origin/main")
    counts = git(root, "rev-list", "--left-right", "--count", f"{default}...HEAD")
    behind, ahead = (int(x) for x in counts.split()) if counts else (None, None)
    track = git(root, "for-each-ref", "--format=%(upstream:short)|%(upstream:track)",
                f"refs/heads/{branch}") or "|"
    upstream, _, state = track.partition("|")
    dirty = git(root, "status", "--porcelain")
    return {"branch": branch, "default": default, "behind": behind, "ahead": ahead,
            "upstream": upstream or None, "upstream_gone": "gone" in state,
            "uncommitted": len(dirty.splitlines()) if dirty else 0}


# ---------------------------------------------------------------------------

E = html.escape
STATUS = {  # icon + label always travel with the colour: a status is never colour alone
    "pass": ("✓", "Pass", "good"), "warn": ("!", "Warnings", "warning"),
    "fail": ("✕", "Failing", "critical"), "skip": ("–", "Not run", "muted"),
}


def badge(state: str) -> str:
    icon, label, tone = STATUS.get(state, ("?", state, "muted"))
    return f'<span class="badge {tone}"><span aria-hidden="true">{icon}</span> {label}</span>'


def tile(label: str, value: str, note: str = "") -> str:
    return (f'<div class="tile"><div class="tile-label">{E(label)}</div>'
            f'<div class="tile-value">{value}</div>'
            f'<div class="tile-note">{E(note)}</div></div>')


def issue_list(rows) -> str:
    if not rows:
        return '<p class="empty">None.</p>'
    return '<ul class="issues">' + "".join(
        f"<li><a href='{E(i['url'])}'>#{i['number']}</a> {E(i['title'])}</li>" for i in rows) \
        + "</ul>"


def waiting_html(gh_data: dict | None) -> str:
    if gh_data is None:
        return '<p class="empty">Issues not requested (--no-github).</p>'
    if gh_data.get("issues_error"):
        return f'<p class="empty">Issues unavailable: {E(gh_data["issues_error"])}.</p>'
    found = gh_data.get("issues") or {}
    return "".join(f"<h3>{E(title)} <code>{E(label)}</code></h3>{issue_list(found.get(label))}"
                   for label, title in WAITING)


def waiting_count(gh_data: dict | None):
    if not gh_data or gh_data.get("issues_error") or not gh_data.get("issues"):
        return None
    seen = {i["number"] for label in ("needs-verification", "founder-only")
            for i in gh_data["issues"].get(label) or []}
    return len(seen)


def findings_table(rows) -> str:
    body = "".join(
        f"<tr><td><code>{E(c)}</code></td><td><code>{E(f['file'])}:{f['line']}</code></td>"
        f"<td>{E(f['message'])}</td></tr>" for c, f in rows)
    return ('<table><thead><tr><th>Check</th><th>Where</th><th>Finding</th></tr></thead>'
            f"<tbody>{body}</tbody></table>")


def pr_checks(pr) -> str:
    states = [(c.get("conclusion") or c.get("state") or c.get("status") or "").upper()
              for c in pr.get("statusCheckRollup") or []]
    if not states:
        return badge("skip").replace("Not run", "No checks")
    if any(s in ("FAILURE", "ERROR", "TIMED_OUT", "CANCELLED", "ACTION_REQUIRED") for s in states):
        return badge("fail")
    if any(s in ("PENDING", "QUEUED", "IN_PROGRESS", "EXPECTED", "") for s in states):
        return badge("warn").replace("Warnings", "Pending")
    return badge("pass")


def working_copy_html(wc: dict | None) -> str:
    if not wc:
        return '<p class="empty">Working copy not inspected.</p>'
    rows = [("Branch", f"<code>{E(wc['branch'])}</code>")]
    if wc["behind"] is None:
        rows.append(("Against " + E(wc["default"]), badge("skip").replace("Not run", "Unknown")))
    else:
        state = "pass" if wc["behind"] == 0 else "warn"
        label = "Up to date" if wc["behind"] == 0 else f"{wc['behind']} behind"
        rows.append((f"Against <code>{E(wc['default'])}</code>",
                     badge(state).replace(STATUS[state][1], label) +
                     (f" · {wc['ahead']} ahead" if wc["ahead"] else "")))
    if wc["upstream_gone"]:
        rows.append(("Remote branch", badge("fail").replace("Failing", "Deleted on the remote")))
    elif wc["upstream"]:
        rows.append(("Remote branch", f"<code>{E(wc['upstream'])}</code>"))
    rows.append(("Uncommitted files", str(wc["uncommitted"])))
    return "<table><tbody>" + "".join(f"<tr><th>{k}</th><td>{v}</td></tr>" for k, v in rows) + \
        "</tbody></table>"


def render(report: dict, gh_data: dict | None, project: str, wc: dict | None = None) -> str:
    m = report["metrics"]
    checks = report["checks"]
    errors = [(c["id"], f) for c in checks for f in c["findings"] if f["severity"] == "error"]
    warns = [(c["id"], f) for c in checks for f in c["findings"] if f["severity"] != "error"]
    rc, d = m["reading_cost"], m["docs_only_share"]
    waiting = waiting_count(gh_data)

    kpis = "".join([
        tile("FLOW checks", badge(report["result"]),
             f"{len(errors)} failing, {len(warns)} warnings"),
        tile("Phase", E(m.get("phase") or "unknown"), "STATUS › Current Phase"),
        tile("Read every session", f"{rc['session_start']:,}",
             f"words; {rc['with_optional']:,} with optional files"),
        tile("Docs-only commits", f"{round(d['share'] * 100)}%",
             f"{d['docs_only']} of {d['commits']} in {d['days']} days"),
        tile("Waiting on you", "–" if waiting is None else f"{waiting}",
             "issues to confirm or do" if waiting is not None else "GitHub data unavailable"),
    ])

    rule_rows = "".join(
        f"<tr><td><code>{c['id']}</code></td><td>{E(c['name'])}</td><td>{badge(c['status'])}</td>"
        f"<td class='num'>{sum(f['severity'] == 'error' for f in c['findings'])}</td>"
        f"<td class='num'>{sum(f['severity'] != 'error' for f in c['findings'])}</td></tr>"
        for c in checks)

    if gh_data is None:
        gh_html = '<p class="empty">GitHub data not requested (--no-github).</p>'
    else:
        if gh_data["prs"] is None:
            prs_html = f'<p class="empty">Pull requests unavailable: {E(gh_data["pr_error"])}.</p>'
        elif not gh_data["prs"]:
            prs_html = '<p class="empty">No open pull requests.</p>'
        else:
            prs_html = ("<table><thead><tr><th>#</th><th>Title</th><th>Checks</th></tr></thead>"
                        "<tbody>" + "".join(
                            f"<tr><td class='num'><a href='{E(p['url'])}'>{p['number']}</a></td>"
                            f"<td>{E(p['title'])}{' <em>(draft)</em>' if p.get('isDraft') else ''}"
                            f"</td><td>{pr_checks(p)}</td></tr>" for p in gh_data["prs"])
                        + "</tbody></table>")
        run = gh_data["run"]
        if run:
            state = {"success": "pass", "failure": "fail"}.get(run.get("conclusion") or "", "warn")
            ci_html = (f"<p>{badge(state)} <a href='{E(run['url'])}'>{E(run['workflowName'])}</a> "
                       f"on <code>{E(gh_data['default'])}</code> — {E(run['displayTitle'])} "
                       f"(<code>{E(run['headSha'][:7])}</code>)</p>")
        else:
            reason = gh_data["run_error"] or "no runs found"
            ci_html = f'<p class="empty">CI unavailable: {E(reason)}.</p>'
        gh_html = f"<h3>Open pull requests</h3>{prs_html}<h3>CI on the default branch</h3>{ci_html}"

    overrides = m.get("overrides") or []
    over_html = ('<p class="empty">None. Every limit is FLOW\'s default.</p>' if not overrides else
                 "<table><thead><tr><th>Setting</th><th>FLOW default</th><th>This project</th>"
                 "</tr></thead><tbody>" + "".join(
                     f"<tr><td><code>{E(o['key'])}</code></td><td>{E(repr(o['default']))}</td>"
                     f"<td>{E(repr(o['value']))}</td></tr>" for o in overrides)
                 + "</tbody></table>")

    errors_html = findings_table(errors) if errors else '<p class="empty">Nothing failing.</p>'
    warns_html = (f"<details><summary>{len(warns)} warnings — old entries over a limit, untouched "
                  f"by any pull request</summary>{findings_table(warns)}</details>") if warns else ""

    return f"""<!doctype html>
<html lang="en"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>FLOW · {E(project)}</title>
<style>
:root {{
  --bg:#fcfcfb; --surface:#ffffff; --ink:#1a1a19; --ink-2:#55554f; --ink-3:#7a7a73;
  --line:#e6e6e1; --good:#0ca30c; --warning:#fab219; --critical:#d03b3b; --muted:#9a9a93;
  --good-ink:#006300; --warning-ink:#8a5a00; --critical-ink:#a32a2a;
}}
@media (prefers-color-scheme: dark) {{
  :root {{
    --bg:#1a1a19; --surface:#232322; --ink:#f2f2ee; --ink-2:#bdbdb6; --ink-3:#8f8f88;
    --line:#34342f; --good-ink:#5fd15f; --warning-ink:#fab219; --critical-ink:#ff8a8a;
  }}
}}
* {{ box-sizing:border-box; }}
body {{ margin:0; background:var(--bg); color:var(--ink);
  font:15px/1.5 system-ui,-apple-system,"Segoe UI",Roboto,sans-serif; }}
main {{ max-width:1080px; margin:0 auto; padding:24px 16px 48px; }}
h1 {{ font-size:22px; margin:0 0 4px; }}
h2 {{ font-size:16px; margin:32px 0 10px; }}
h3 {{ font-size:14px; margin:18px 0 8px; color:var(--ink-2); }}
.sub {{ color:var(--ink-3); margin:0 0 20px; }}
.kpis {{ display:grid; grid-template-columns:repeat(auto-fit,minmax(180px,1fr)); gap:12px; }}
.tile {{ background:var(--surface); border:1px solid var(--line); border-radius:10px; padding:14px; }}
.tile-label {{ color:var(--ink-2); font-size:13px; }}
.tile-value {{ font-size:28px; font-weight:600; margin:2px 0; font-variant-numeric:tabular-nums; }}
.tile-note {{ color:var(--ink-3); font-size:12px; }}
.badge {{ display:inline-flex; align-items:center; gap:4px; font-size:13px; font-weight:600;
  padding:1px 8px; border-radius:999px; border:1px solid currentColor; white-space:nowrap; }}
.tile .badge {{ font-size:16px; padding:3px 10px; }}
.badge.good {{ color:var(--good-ink); }} .badge.warning {{ color:var(--warning-ink); }}
.badge.critical {{ color:var(--critical-ink); }} .badge.muted {{ color:var(--ink-3); }}
.kpis > *, .cols > * {{ min-width:0; }}
.empty {{ overflow-wrap:anywhere; }}
td {{ overflow-wrap:break-word; }}
.cols {{ display:grid; grid-template-columns:repeat(auto-fit,minmax(300px,1fr)); gap:0 28px; }}
table {{ width:100%; border-collapse:collapse; font-size:14px; }}
th {{ text-align:left; color:var(--ink-2); font-weight:600; font-size:12px; }}
th, td {{ padding:6px 8px; border-bottom:1px solid var(--line); vertical-align:top; }}
td.num, th.num {{ text-align:right; font-variant-numeric:tabular-nums; }}
.scroll {{ overflow-x:auto; }}
code {{ font:13px ui-monospace,SFMono-Regular,Menlo,Consolas,monospace; overflow-wrap:anywhere; }}
ul.issues {{ list-style:none; padding:0; margin:0; }}
ul.issues li {{ padding:4px 0; border-bottom:1px solid var(--line); overflow-wrap:anywhere; }}
.empty {{ color:var(--ink-3); margin:4px 0; }}
details summary {{ cursor:pointer; color:var(--ink-2); margin:12px 0 6px; }}
a {{ color:inherit; }}
footer {{ margin-top:36px; color:var(--ink-3); font-size:12px; }}
</style></head>
<body><main>
<h1>{E(project)}</h1>
<p class="sub">FLOW dashboard · generated from flow_check {E(report['tool_version'])} — regenerate,
don't edit</p>
<section class="kpis" aria-label="Summary">{kpis}</section>

<div class="cols">
<section><h2>Waiting on you</h2>
{waiting_html(gh_data)}
{gh_html}
<h3>This working copy</h3>{working_copy_html(wc)}
</section>
<section><h2>Rule health</h2>
<div class="scroll"><table><thead><tr><th>Check</th><th>Rule</th><th>Status</th>
<th class="num">Fail</th><th class="num">Warn</th></tr></thead>
<tbody>{rule_rows}</tbody></table></div>
</section>
</div>

<section><h2>What is failing</h2><div class="scroll">{errors_html}{warns_html}</div></section>
<section><h2>Limits changed from FLOW's defaults</h2><div class="scroll">{over_html}</div></section>
<footer>Definitions of every check: <code>.flow/SPEC.md</code>.</footer>
</main></body></html>
"""


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(prog="flow_dashboard", description=__doc__.splitlines()[0])
    ap.add_argument("--root", default=".")
    ap.add_argument("--report", help="a flow_check --json report; otherwise run flow_check now")
    ap.add_argument("--out", default=".flow/dashboard.html")
    ap.add_argument("--no-github", action="store_true")
    a = ap.parse_args(argv)
    root = Path(a.root).resolve()
    if a.report:
        report = json.loads(Path(a.report).read_text(encoding="utf-8"))
    else:
        fc = load_flow_check()
        try:
            report = fc.run(root)
        except fc.ToolError as e:
            print(f"flow_dashboard: flow_check could not run: {e}", file=sys.stderr)
            return 2
    out = root / a.out if not Path(a.out).is_absolute() else Path(a.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(render(report, None if a.no_github else github(root), root.name,
                          working_copy(root)), encoding="utf-8")
    print(f"wrote {out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
