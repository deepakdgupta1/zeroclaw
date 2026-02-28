# CLAUDE.md — ZeroClaw Agent Engineering Protocol

<!-- Last updated: 2026-02-28 | Protocol version: 3.0 -->

> **Canonical source:** [`AGENTS.md`](AGENTS.md) is the generic, agent-neutral version of this protocol.
> This file (`CLAUDE.md`) is a Claude-specific alias. Keep both in sync.
>
> **This file is intentionally compact.** Detailed rules live in directory-scoped `CLAUDE.md` files
> that load automatically when you work in those directories. See [§9 Content Map](#9-content-map).

This file defines the default working protocol for Claude agents in this repository.
Scope: entire repository.

## 1) Quick Start for Agents

1. **Architecture is trait-driven.** Add features by implementing a trait + registering in a factory. Do not restructure modules for isolated features.
2. **Security is first-class.** Gateway, tools, and runtime can execute real-world actions. Default to deny. Never log secrets. Never silently broaden permissions.
3. **Binary size is a product goal.** The project compiles with `opt-level = "z"` and `lto = "fat"`. Avoid adding crate dependencies unless strongly justified.

| Task                 | Start Here                                                              |
| -------------------- | ----------------------------------------------------------------------- |
| Add a model provider | `src/providers/traits.rs` → implement `Provider` → register in `mod.rs` |
| Add a channel        | `src/channels/traits.rs` → implement `Channel` → register in `mod.rs`   |
| Add a tool           | `src/tools/traits.rs` → implement `Tool` → register in `mod.rs`         |
| Add a plugin         | `src/plugins/traits.rs` → implement plugin trait → register in `mod.rs` |
| Fix a bug            | Read module, factory wiring, and adjacent tests before editing          |
| Submit a PR          | Follow `/pr-submission` workflow                                        |
| Update docs          | See `docs/CLAUDE.md` for docs system contract                           |

## 2) Project Snapshot

ZeroClaw is a Rust-first autonomous agent runtime optimized for high performance, efficiency, stability, extensibility, sustainability, and security.

Core architecture is trait-driven and modular. Extension points (all have a `traits.rs`):

`providers` · `channels` · `tools` · `memory` · `observability` · `runtime` · `peripherals` · `config` · `hooks` · `plugins` · `security`

### Codebase Realities Driving This Protocol

- **Trait + factory = stability backbone.** Features via trait impl + factory registration, not cross-cutting rewrites.
- **Security surfaces are internet-adjacent.** `gateway/`, `security/`, `tools/`, `runtime/` carry high blast radius. Secure-by-default.
- **Performance and binary size are product goals.** Release profile optimizes for size. Convenience deps silently regress.
- **Config and CLI are user-facing API.** Backward compatibility and explicit migration matter.
- **High-concurrency collaboration mode.** CI + docs governance + label routing are part of product delivery. PR throughput is a design constraint.
- **Feature flags gate subsystems.** Many modules are conditionally compiled — verify which features are active before modifying gated code.

## 3) Engineering Principles

These are implementation constraints, not slogans. All are mandatory.

- **KISS:** Straightforward control flow over clever meta-programming. Explicit match branches, typed structs, localized error paths.
- **YAGNI:** No new config keys, trait methods, or feature flags without a concrete accepted use case. No speculative abstractions.
- **DRY + Rule of Three:** Duplicate small local logic for clarity. Extract shared utilities only after stable, repeated patterns (×3).
- **SRP + ISP:** One concern per module. Extend via narrow traits. No fat interfaces or god modules.
- **Fail Fast:** Explicit `bail!`/errors for unsupported states. Never silently broaden permissions. Document intentional fallbacks.
- **Secure by Default:** Deny-by-default. Never log secrets. Narrowest possible scope.
- **Determinism:** Reproducible CI. No flaky tests. Local validation maps to CI expectations.
- **Reversibility:** Small scope, clear blast radius. Define rollback before merge. No mixed mega-patches.
- **Dependency Policy:** Before adding a crate, check if <50 lines or an existing dep covers it. Prefer `default-features = false`. Gate niche deps behind feature flags. Justify in PR description.
- **Error Handling:** `thiserror` for library-facing typed errors; `anyhow` for application-facing operational errors. No `unwrap()` in runtime paths. Messages: lowercase, no trailing punctuation, actionable.

## 4) Repository Map

### `src/` — Core Application

| Group                   | Modules                                                                            |
| ----------------------- | ---------------------------------------------------------------------------------- |
| **Agent Runtime**       | `agent/`, `config/`, `runtime/`, `coordination/`, `daemon/`, `service/`            |
| **AI & Knowledge**      | `providers/`, `memory/`, `rag/`, `skills/`, `skillforge/`, `sop/`, `goals/`        |
| **Communication**       | `channels/`, `gateway/`, `tunnel/`                                                 |
| **Execution**           | `tools/`, `plugins/`, `hooks/`                                                     |
| **Security**            | `security/`, `auth/`, `approval/`                                                  |
| **Ops & Observability** | `observability/`, `health/`, `heartbeat/`, `doctor/`, `cost/`, `cron/`             |
| **Hardware**            | `peripherals/`, `hardware/`, `integrations/`                                       |
| **Other**               | `onboard/`, `identity.rs`, `migration.rs`, `multimodal.rs`, `update.rs`, `util.rs` |

### Top-Level Directories

`crates/` (workspace sub-crates) · `clients/` · `python/` · `web/` · `firmware/` · `scripts/` · `dev/` (local CI) · `fuzz/` · `templates/` · `extensions/` · `examples/` · `tests/` · `benches/` · `docs/` · `.github/` · `site/`

## 5) Risk Tiers

- **Low:** docs, chore, tests-only
- **Medium:** most `src/**` without boundary/security impact
- **High:** `src/security/**`, `src/auth/**`, `src/approval/**`, `src/runtime/**`, `src/gateway/**`, `src/tools/**`, `src/tunnel/**`, `.github/workflows/**`

When uncertain, classify higher.

## 6) Agent Workflow

1. **Read before write** — inspect module, factory wiring, and adjacent tests.
2. **One concern per PR** — no mixed feature+refactor+infra patches.
3. **Implement minimal patch** — KISS/YAGNI/DRY rule-of-three.
4. **Validate by risk tier** — see `/pr-submission` workflow for full matrix.
5. **Document impact** — update runtime-contract references if behavior changed. See `docs/CLAUDE.md` for i18n rules.
6. **Queue hygiene** — `Depends on #...` for stacked PRs. `Supersedes #...` for replacements.

**Note:** `.claude.local.md` is **not** gitignored in this repo. Create session-local context only if you add `.claude.local.md` to `.gitignore` first, or use auto-memory instead.

## 7) Anti-Patterns (Do Not)

- Add heavy deps for minor convenience (use `format!()`, not a templating crate for one string).
- Silently weaken security policy (no deny→allow without PR discussion).
- Add speculative config/feature flags with no current caller.
- Mix formatting-only changes with functional changes (separate commits).
- Modify unrelated modules "while here" (separate PRs).
- Bypass failing checks without explicit explanation.
- Hide behavior changes in refactor commits.
- Include personal identity or sensitive data in test data, docs, or commits.
- Attempt repository rebranding unless maintainers explicitly requested it.
- Introduce new platform surfaces (web apps, dashboards) unless explicitly requested.

## 8) Vibe Coding Guardrails


- Keep each iteration reversible (small commits, clear rollback).
- Validate assumptions with code search before implementing.
- Prefer deterministic behavior over clever shortcuts.
- Do not "ship and hope" on security-sensitive paths.
- If uncertain, leave a concrete TODO with verification context, not a hidden guess.

## 9) Content Map

Detailed rules load automatically when you work in these directories:

| Directory                   | What's There                                                              |
| --------------------------- | ------------------------------------------------------------------------- |
| `src/CLAUDE.md`             | Code naming contract, architecture boundaries, error handling conventions |
| `src/providers/CLAUDE.md`   | Provider playbook                                                         |
| `src/channels/CLAUDE.md`    | Channel playbook                                                          |
| `src/tools/CLAUDE.md`       | Tool playbook (high-risk)                                                 |
| `src/plugins/CLAUDE.md`     | Plugin playbook                                                           |
| `src/hooks/CLAUDE.md`       | Hook playbook                                                             |
| `src/skills/CLAUDE.md`      | Skill playbook                                                            |
| `src/peripherals/CLAUDE.md` | Peripheral playbook                                                       |
| `src/security/CLAUDE.md`    | Security/runtime/gateway playbook (high-risk)                             |
| `docs/CLAUDE.md`            | Docs system contract, i18n governance, docs change playbook               |
| `.github/CLAUDE.md`         | PR disposition, assignee gate, supersede attribution templates            |

### Workflows (invoked on demand)

| Workflow         | What's There                                                                                     |
| ---------------- | ------------------------------------------------------------------------------------------------ |
| `/pr-submission` | Branch/commit/PR flow, worktree protocol, validation matrix, handoff template, privacy checklist |

### Reference Docs

`CONTRIBUTING.md` · `docs/README.md` · `docs/SUMMARY.md` · `docs/i18n-guide.md` · `docs/i18n-coverage.md` · `docs/commands-reference.md` · `docs/providers-reference.md` · `docs/channels-reference.md` · `docs/config-reference.md` · `docs/operations-runbook.md` · `docs/troubleshooting.md` · `docs/pr-workflow.md` · `docs/reviewer-playbook.md` · `docs/ci-map.md` · `docs/actions-source-policy.md`
