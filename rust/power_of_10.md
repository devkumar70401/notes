# NASA Power of 10 

10 Rules that needs to be followed.

## 1. Eliminate All recursion

- Never implement Recursive function calls.
- Convert all graph traversals, tree walks, and iterative mathematics to iterative algorithms using explicit loop bounds or bounded indices.

## 2. Guarantee Fixed Upper Bounds on All Loops

- Every loop must possess an invariant , statically verifiable loop counter or an iterator ceiling.
- Use explicit iteration ranges (`0..MAX_ITERATIONS`) or the `.take(N)` iterator adapter.

## 3. Ban Dynamic Heap Allocation After Boot (`#![no_std] + heapless`)

- Compile strictly with `#![no_std]`.
- Ban runtime heap allocators.
- All collection capacities, packet frames, and flight telemetry queues must have static, compile-time size limits using standard arrays or statically backend crates like `heapless`.

## 4. Keep Functions Modular and Under 60 Lines 

- A function must execute a single task with clear inputs and outputs.
- Break complex processing steps into pure helper functions.

## 5. Zero Panics - Treat Panicking Code as a Compilation Bug

- Standard `assert!`, `panic!`, and direct slice indexing (`&slice[i]`) are forbidden in production code paths.
- All failure conditions must return an explicit `Result<T,E>`.
- Use the **Typestate Pattern** to enforce valid states at compile time rather than relying on runtime validation.
- Use explicit methods for fallible operations: `.get()`, `.checked_add()`, or `.saturating_mul()`.

## 6. Minimize Variable Scopes and Enforce Immutability

- Variables must default to immutable (`let`).
- Restrict `let mut` strictly to the smallest inner lexical scope requiring mutation.
- Use block expressions and variable shadowing to discard intermediate values immediately.

## 7. Zero Unchecked Returns (`#![deny(unused_must_use)]` & No `.unwrap()`)

- Enforce `#![deny(unused_must_use)]` across all crates
- Banish `.unwrap()` and `.expect()` from safety-critical builds.
- Unpack every `Result` explicitly using `match` , `if let`, or the `?` error propagation operator.

## 8. Restrict Macro Metaprogramming in Safety Paths

- Ban procedural macros and complex recursive `macro_rules!` from core control loops and state machines.
- Replace compile-time code generation with native `const fn`, `const` generics, and standard traits.

## 9. Pure Safe Rust only (`#![forbid(unsafe_code)]`)

- Declare `#![forbid(unsafe_code)]` across all system logic, navigation, state machines, and mathematical engines.
- Isolate low-level memory-mapped I/O (MMIO) and register writes inside certified, heavily audited Hardware Abstraction layers (HALs) or Peripheral Access Crates (PACs). Application logic must remain 100% safe.
- Prefer static dispatch using generics and traits over dynamic dispatch (`&dyn Trati`) to avoid indirect calls and vtable overhead.

## 10. Enforce Zero-Warning Compilation and Formal Verification

- Enforce strict linting at the workspace level: 
  - ```rust 
       #![deny(warnings)]
       #![warn(clippy::pedantic)]
       #![warn(clippy::nursery)]
       ```
- Use formal verification and model checking tools:
  - **Kani Rust Verifier:** Formally proves properties, absense of panics and absence of arithmetic overflow using bounded model checking.
  - **Miri (`cargo miri`):** Detets undefined behavior, memory leaks, and alignment errors in dependencies.

```rust
#![no_std]
#![no_main]
#![forbid(unsafe_code)]
#![deny(warnings)]
#![deny(unused_must_use)]
#![warn(clippy::pedantic)]

use core::panic::PanicInfo;
```