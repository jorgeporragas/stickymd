# Patterns worth adopting

Shipped with FLOW as `.flow/PATTERNS.md`. Read at Initialization, and when a project sets up or
changes CI. These are patterns, not rules: each is worth its cost in most projects, and how to
build it depends on the stack and host. Each one was proven in Banana Splits; the field reports
in the FLOW repository have the details.

## CI checks; it never deploys production

CI runs the checks on every pull request and every push to the default branch. Deploying to
production stays a deliberate, manual step, because a live domain is behind it. A red run is
fixed, never bypassed: no skipped tests, loosened flags or `continue-on-error` to get green.

## A preview for every pull request

When the checks pass, deploy the bundle **they built** — not a second build — to a temporary,
per-pull-request address, so the founder can review on a phone before merging. A failing pull
request gets no preview. State plainly what a preview cannot test: anything tied to the
production domain (passkeys, OAuth redirects, deep links) usually fails on a preview host.

## Keyless credentials in CI

CI reaches a cloud provider through short-lived federated tokens (OIDC, workload identity), not
a stored key. The trust admits one repository; the identity holds the one role a deploy is
proven to need, and nothing more. A permission failure is reported with the role the error
names, never fixed by guessing one or creating a key.

## A build guard that fails closed

Before anything ships, a script inspects the **built artifact** — not the command that built it
— for the mistakes that ship silently: a development backend baked in, a required file missing.
It refuses when unsure. A false positive blocks a legitimate deploy, which is noisy; a false
negative ships something broken that nobody notices. Prefer the noise.

## Generated reference docs, checked fresh

Whatever the code can describe about itself — a schema reference from the migrations, token
tables from the token source, generated code from its inputs — is produced by a command, not
written by hand (FLOW_SYSTEM › *Derive, don't record*). The generated file opens with that
command and a do-not-edit line. CI runs the command and fails on `git diff --exit-code`, so a
stale file is red, not silent. Before trusting the step, check that a clean run on the CI
machine reproduces the committed file byte for byte: a different generator version or line
ending turns it red on day one. Point a co-change rule that guarded the hand-written copy at the
generated file, or drop it — the freshness step already covers it.

## A workflow change is reviewed before it runs

A pull request runs its own branch's workflow, with whatever credentials CI holds, before anyone
has reviewed the change. Agent settings ask before any edit to CI workflow files, and every
workflow change is named in its pull request.
