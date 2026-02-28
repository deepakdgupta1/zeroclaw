# providers/ — Adding a Provider

> **Active primary provider: Gemini** (`gemini-flash` / `gemini-pro`). Target Gemini when implementing provider features or testing.

1. Implement the `Provider` trait from `traits.rs`.
2. Register in `mod.rs` factory.
3. Add focused tests for factory wiring and error paths.
4. Avoid provider-specific behavior leaks into shared orchestration code in `src/agent/`.
