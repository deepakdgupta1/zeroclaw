---
description: PR submission, worktree workflow, validation, and handoff
---

# PR Submission Workflow

## Branch / Commit / PR Flow

1. Create and work from a non-`main` branch.
2. Commit changes with clear, scoped commit messages using conventional commit titles.
3. Open a PR to `main` by default (`dev` is optional for integration batching); do not push directly to `dev` or `main`.
4. `main` accepts direct PR merges after required checks and review policy pass.
5. Wait for required checks and review outcomes before merging.
6. Merge via PR controls (squash/rebase/merge as repository policy allows).
7. After merge/close, clean up task branches/worktrees.
8. Keep long-lived branches only when intentionally maintained with clear owner and purpose.

## Worktree Workflow

- Use one dedicated worktree per active branch/PR stream; do not implement directly in a shared default workspace.
- Keep each worktree on a single branch and a single concern; do not mix unrelated edits.
- Before each commit/push, verify commit hygiene: `git status --short` and `git diff --cached` so only scoped files are included.
- Run validation commands inside the corresponding worktree before commit/PR.
- Name worktrees clearly by scope (e.g., `wt/ci-hardening`, `wt/provider-fix`).
- After PR merge/close (or task abandonment), remove stale worktrees/branches and prune refs (`git worktree prune`, `git fetch --prune`).
- Local Codex automation may use: `~/.codex/skills/zeroclaw-pr-issue-automation/scripts/cleanup_track.sh --repo-dir <repo_dir> --worktree <worktree_path> --branch <branch_name>`.

## Validation Matrix

Default local checks for code changes:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Preferred pre-PR validation (recommended, not required):

```bash
./dev/ci.sh all
```

Notes:

- Local Docker-based CI is strongly recommended when Docker is available.
- If Docker CI is unavailable, run the most relevant native checks and document what was run.
- For feature-gated code, test with the relevant feature enabled (e.g., `cargo test --features hardware`).

Additional expectations by change type:

- **Docs/template-only**: markdown lint + link-integrity; verify locale navigation parity if touching README/docs-hub/SUMMARY.
- **Workflow changes**: validate YAML syntax; run workflow lint when available.
- **Security/runtime/gateway/tools**: include at least one boundary/failure-mode validation.
- **Bootstrap docs/scripts**: run `bash -n bootstrap.sh scripts/bootstrap.sh scripts/install.sh`.

If full checks are impractical, run the most relevant subset and document what was skipped.

## Handoff Template (Agent → Agent / Maintainer)

When handing off work, include:

1. What changed
2. What did not change
3. Validation run and results
4. Remaining risks / unknowns
5. Next recommended action

## PR Discipline Checklist

- Follow `.github/pull_request_template.md` fully (including side effects / blast radius).
- Keep PR descriptions concrete: problem, change, non-goals, risk, rollback.
- Add explicit issue-closing keywords in the **PR body** (e.g., `Closes #1502`).
- Default to one issue per clean commit/PR track.
- If bundling multiple issues, document the coupling rationale in PR summary.
- Stage only task-scoped files; split unrelated changes into separate commits/worktrees.
- After merge/close, clean stale local branches/worktrees before starting the next track.
- Prefer small PRs (`size: XS/S/M`) when possible.

## Privacy/Sensitive Data Checklist

- Never commit personal or sensitive data (real names, emails, phone numbers, tokens, API keys, credentials, private URLs).
- Use neutral project-scoped placeholders (`user_a`, `test_user`, `project_bot`, `example.com`).
- Identity-safe naming palette when identity context is required:
    - actor: `ZeroClawAgent`, `ZeroClawOperator`, `ZeroClawMaintainer`, `zeroclaw_user`
    - service: `zeroclaw_bot`, `zeroclaw_service`, `zeroclaw_runtime`, `zeroclaw_node`
    - environment: `zeroclaw_project`, `zeroclaw_workspace`, `zeroclaw_channel`
- Before push, review `git diff --cached` for accidental sensitive strings and identity leakage.
