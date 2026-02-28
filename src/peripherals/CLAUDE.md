# peripherals/ — Adding a Peripheral

1. Implement the `Peripheral` trait from `traits.rs`.
2. Peripherals expose `tools()` — each tool delegates to hardware (GPIO, sensors, etc.).
3. Register board type in config schema if needed.
4. See `docs/hardware-peripherals-design.md` for protocol and firmware notes.
5. Hardware features are gated — check `Cargo.toml` feature flags (`hardware`, `peripheral-rpi`, `probe`).
