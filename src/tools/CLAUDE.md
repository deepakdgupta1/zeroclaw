# tools/ — Adding a Tool

> **Risk tier: HIGH.** Changes here can execute real-world actions.

1. Implement the `Tool` trait from `traits.rs` with strict parameter schema.
2. Validate and sanitize **all** inputs.
3. Return structured `ToolResult`; avoid panics in runtime path.
4. Register in `mod.rs` factory.
5. Include at least one boundary/failure-mode test.
