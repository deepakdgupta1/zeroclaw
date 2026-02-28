# .github/ — PR Disposition, Workflow Authority, and Collaboration Templates

## PR Disposition and Workflow Authority (Required)

- Decide merge/close outcomes from repository-local authority in this order: `.github/workflows/**`, GitHub branch protection/rulesets, `docs/pr-workflow.md`, then root `CLAUDE.md`.
- External agent skills/templates are execution aids only; they must not override repository-local policy.
- A normal contributor PR targeting `main` is a routing defect, not by itself a closure reason; if intent and content are legitimate, retarget to `dev`.
- Direct-close the PR (do not supersede/replay) when high-confidence integrity-risk signals exist:
    - unapproved or unrelated repository rebranding attempts (e.g., replacing project logo/identity assets)
    - unauthorized platform-surface expansion (e.g., introducing `web` apps, dashboards, frontend stacks, or UI surfaces not requested by maintainers)
    - title/scope deception that hides high-risk code changes (e.g., `docs:` title with broad `src/**` changes)
    - spam-like or intentionally harmful payload patterns
    - multi-domain dirty-bundle changes with no safe, auditable isolation path
- If unauthorized platform-surface expansion is detected during review/implementation, report to maintainers immediately and pause further execution until explicit direction is given.
- Use supersede flow only when maintainers explicitly want to preserve valid work and attribution.
- In public PR close/block comments, state only direct actionable reasons; do not include internal decision-process narration or "non-reason" qualifiers.

## Assignee-First Gate (Required)

- For any GitHub issue or PR selected for active handling, the first action is to ensure `@deepakdgupta1` is an assignee.
- This is additive ownership: keep existing assignees and add `@deepakdgupta1` if missing.
- Do not start triage/review/implementation/merge work before assignee assignment is confirmed.
- Queue safety rule: assign only the currently active target; do not pre-assign future queued targets.

## Superseded-PR Attribution (Required)

When a PR supersedes another contributor's PR and carries forward substantive code or design decisions, preserve authorship explicitly.

- In the integrating commit message, add one `Co-authored-by: Name <email>` trailer per superseded contributor whose work is materially incorporated.
- Use a GitHub-recognized email (`<login@users.noreply.github.com>` or the contributor's verified commit email) so attribution is rendered correctly.
- Keep trailers on their own lines after a blank line at commit-message end; never encode them as escaped `\\n` text.
- In the PR body, list superseded PR links and briefly state what was incorporated from each.
- If no actual code/design was incorporated (only inspiration), do not use `Co-authored-by`; give credit in PR notes instead.

## Superseded-PR PR Template (Recommended)

Title format: `feat(<scope>): unify and supersede #<pr_a>, #<pr_b> [and #<pr_n>]`

PR body template:

```md
## Supersedes

- #<pr_a> by @<author_a>
- #<pr_b> by @<author_b>

## Integrated Scope

- From #<pr_a>: <what was materially incorporated>
- From #<pr_b>: <what was materially incorporated>

## Attribution

- Co-authored-by trailers added for materially incorporated contributors: Yes/No
- If No, explain why (e.g., no direct code/design carry-over)

## Non-goals

- <explicitly list what was not carried over>

## Risk and Rollback

- Risk: <summary>
- Rollback: <revert commit/PR strategy>
```

## Superseded-PR Commit Template (Recommended)

```text
feat(<scope>): unify and supersede #<pr_a>, #<pr_b> [and #<pr_n>]

<one-paragraph summary of integrated outcome>

Supersedes:
- #<pr_a> by @<author_a>
- #<pr_b> by @<author_b>

Integrated scope:
- <subsystem_or_feature_a>: from #<pr_x>
- <subsystem_or_feature_b>: from #<pr_y>

Co-authored-by: <Name A> <login_a@users.noreply.github.com>
Co-authored-by: <Name B> <login_b@users.noreply.github.com>
```

Keep one blank line between sections, each trailer on its own line, do not wrap/indent/escaped-encode.
