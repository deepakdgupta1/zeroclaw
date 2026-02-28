# src/ — Code Conventions

These rules apply to all code under `src/`. They supplement the root `CLAUDE.md`.

## Code Naming Contract

- Use Rust standard casing: modules/files `snake_case`, types/traits/enums `PascalCase`, functions/variables `snake_case`, constants/statics `SCREAMING_SNAKE_CASE`.
- Name types and modules by domain role, not implementation detail (`DiscordChannel`, `SecurityPolicy`, `MemoryStore` — not `Manager`/`Helper`).
- Trait implementers: `<ProviderName>Provider`, `<ChannelName>Channel`, `<ToolName>Tool`, `<BackendName>Memory`.
- Factory registration keys: stable, lowercase, user-facing (`"openai"`, `"discord"`, `"shell"`). Avoid alias sprawl without migration need.
- Test names: `<subject>_<expected_behavior>`. Keep fixture identifiers neutral/project-scoped.
- Identity-like naming in tests/examples: use ZeroClaw-native labels only (`ZeroClawAgent`, `zeroclaw_user`, `zeroclaw_node`).

## Architecture Boundary Contract

- Extend capabilities by adding **trait implementations + factory wiring** first; avoid cross-module rewrites for isolated features.
- Dependency direction is **inward** to contracts: concrete integrations depend on trait/config/util layers, not on other concrete integrations.
- No cross-subsystem coupling (e.g., provider code importing channel internals, tool code mutating gateway policy).
- Module responsibilities are single-purpose: orchestration in `agent/`, transport in `channels/`, model I/O in `providers/`, policy in `security/`, execution in `tools/`.
- New shared abstractions only after **rule-of-three** with at least one real caller in current scope.
- Config/schema keys are public contract: document defaults, compatibility impact, and migration/rollback path.

## Error Handling

- `thiserror` for **library-facing** errors — typed enums in trait implementations, public API boundaries, matchable error variants.
- `anyhow` for **application-facing** errors — CLI entrypoints, orchestration glue, operational failures where callers only display or propagate.
- No `unwrap()`/`expect()` in runtime paths. Panics only for truly unrecoverable programmer-bug invariants.
- Error messages: lowercase, no trailing punctuation, actionable (say what to do, not just what went wrong).
