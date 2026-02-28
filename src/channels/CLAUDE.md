# channels/ — Adding a Channel

1. Implement the `Channel` trait from `traits.rs`.
2. Keep `send`, `listen`, `health_check`, typing semantics consistent with existing channels.
3. Cover auth/allowlist/health behavior with tests.
4. Register in `mod.rs` factory.
