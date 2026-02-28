# docs/ — Documentation System Contract

Treat documentation as a first-class product surface, not a post-merge artifact.

## Canonical Entry Points

- Repository landing: `README.md`
- Docs hub: `docs/README.md`
- Unified TOC: `docs/SUMMARY.md`
- i18n governance: `docs/i18n-guide.md`, `docs/i18n/README.md`, `docs/i18n-coverage.md`

## Supported Locales

`en`, `zh-CN`, `ja`, `ru`, `fr`, `vi`, `el`

Localized hubs live under `docs/i18n/<locale>/`.

## Runtime-Contract References

These must be updated when corresponding behavior changes:

`commands-reference.md` · `providers-reference.md` · `channels-reference.md` · `config-reference.md` · `operations-runbook.md` · `troubleshooting.md` · `one-click-bootstrap.md`

(All under `docs/`.)

## Collection Indexes

`getting-started/` · `reference/` · `operations/` · `security/` · `hardware/` · `contributing/` · `project/`

(All under `docs/<name>/README.md`.)

## i18n Governance

Full i18n rules are in [`docs/i18n-guide.md`](i18n-guide.md). Key points:

- Keep entry-point parity across all supported locales when changing navigation architecture.
- If a change touches docs IA, runtime-contract references, or shared wording, perform i18n follow-through in the same PR using `docs/i18n-guide.md` as checklist.
- Update `docs/i18n-coverage.md` when coverage status or locale topology changes.
- If any translation must be deferred, record explicit owner + follow-up issue/PR in the PR description.

## i18n Completion Gate

For any PR that changes docs IA, locale navigation, or shared docs wording:

1. Complete i18n follow-through in the same PR using `docs/i18n-guide.md`.
2. Keep all supported locale hubs/summaries navigable through canonical `docs/i18n/<locale>/` paths.
3. Update `docs/i18n-coverage.md` when coverage status or locale topology changes.
4. If any translation must be deferred, record explicit owner + follow-up issue/PR in the PR description.

## Docs Change Playbook

- Treat docs navigation as product UX: preserve clear pathing from README → docs hub → SUMMARY → category index.
- Keep top-level nav concise; avoid duplicative links across adjacent nav blocks.
- When runtime surfaces change, update related references (`commands/providers/channels/config/runbook/troubleshooting`).
- Keep multilingual entry-point parity for all supported locales when nav or key wording changes.
- When shared docs wording changes, sync corresponding localized docs in the same PR (or explicitly document deferral + follow-up PR).
- Treat `docs/i18n/<locale>/**` as canonical for localized hubs/summaries; keep docs-root compatibility shims aligned when edited.
- Apply `docs/i18n-guide.md` completion checklist before merge and include i18n status in PR notes.
- For docs snapshots, add new date-stamped files for new sprints rather than rewriting historical context.
