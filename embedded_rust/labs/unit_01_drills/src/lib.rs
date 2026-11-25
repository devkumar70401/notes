//! Compiler-verified `#![no_std]` modules accompanying Unit 1 (Chapters 1-3).
//!
//! Notice `#![cfg_attr(not(test), no_std)]`:
//! - `cargo check --lib` compiles strictly under `#![no_std]` (no heap, no OS).
//! - `cargo test` links `std` only for the host test runner.

#![cfg_attr(not(test), no_std)]

pub mod ch01_syntax;
pub mod ch02_ownership;
pub mod ch03_registers;
