# plugins/ — Adding a Plugin

1. Implement the plugin trait from `traits.rs`.
2. Register in `mod.rs` factory.
3. Plugins must be self-contained; avoid tight coupling with the core agent loop in `src/agent/`.
4. If the plugin needs config, extend `src/config/schema.rs` with a dedicated section.
5. Add tests for plugin lifecycle (init, execute, teardown).
