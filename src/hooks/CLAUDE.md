# hooks/ — Adding a Hook

1. Implement the hook trait from `traits.rs`.
2. Register in `mod.rs`.
3. Hooks fire on lifecycle events — keep them lightweight and **non-blocking**.
4. Do not mutate core agent state from within hooks; use the event data immutably.
5. Test both successful and failing hook execution paths.
