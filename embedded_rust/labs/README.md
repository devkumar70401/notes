# Embedded Rust Hands-On Labs

Two runnable Rust projects paired with Unit 1 (`ch01`–`ch03`):

1. **`unit_01_drills/`** — `#![no_std]` library crate tested on your Linux host (`x86_64`) for fast feedback with `rustc`, `cargo test`, and `cargo clippy`.
2. **`esp32c3_starter/`** — Bare-metal `#![no_std]` `#![no_main]` firmware for the **ESP32-C3 SuperMini** (`riscv32imc-unknown-none-elf`), ready to flash with `espflash`.

---

## 1. Host Drills (`unit_01_drills/`)

Run all verified reference tests and strict linter checks:

```bash
cd unit_01_drills
cargo check --lib          # Enforces #![no_std] (proves zero std/heap usage)
cargo test --lib           # Runs reference implementations for Chapters 1-3
cargo clippy --all-targets -- -D warnings
```

Solve the 6 hands-on exercises in `tests/your_turn_drills.rs` one by one:

```bash
cargo test --test your_turn_drills -- --ignored
```

Replace each `todo!()` inside `tests/your_turn_drills.rs` until all 6 drills pass.

---

## 2. ESP32-C3 SuperMini Firmware (`esp32c3_starter/`)

Build check without hardware attached:

```bash
cd esp32c3_starter
cargo build --release
```

Flash and open the serial monitor over USB-C (when your ESP32-C3 SuperMini is plugged in):

```bash
cargo run --release
```

*(Uses `.cargo/config.toml` runner `espflash flash --monitor`.)*
