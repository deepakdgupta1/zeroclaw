# security/ — Security, Runtime, and Gateway Changes

> **Risk tier: HIGH.** These modules carry high blast radius and are internet-adjacent.

## Required for All Changes Here

- Include threat/risk notes and rollback strategy in PR description.
- Add/update tests or validation evidence for failure modes and boundaries.
- Keep observability useful but **non-sensitive** — never log secrets, raw tokens, or sensitive payloads.
- Deny-by-default for access and exposure boundaries.
- Never silently broaden permissions or capabilities.
- Keep network/filesystem/shell scope as narrow as possible unless explicitly justified.

## For `.github/workflows/**` Changes

- Include Actions allowlist impact in PR notes.
- Update `docs/actions-source-policy.md` when sources change.
