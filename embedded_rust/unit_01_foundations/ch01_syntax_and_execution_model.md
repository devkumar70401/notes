# Chapter 1: From Java/Python to Rust: Syntax & Execution Model

## 1.1 The Compilation Model: Bytecode & Interpreters vs. Ahead-of-Time (AOT) Machine Code

### Overview: Interpreters vs. JITs vs. Native Ahead-of-Time (AOT)

* Programs written in Java and Python run on top of an intermediary abstraction layer: a Virtual Machine (JVM) or an Interpreter (CPython).
* When programming microcontrollers for robotics, this managed execution model collapses under strict hardware and timing limits. To build reliable systems, we must understand why managed runtimes fail on bare metal and how Rust's **Ahead-of-Time (AOT)** compilation delivers bare-metal control with zero overhead.

---

### Architectural Pipeline Comparison: Python vs. Java vs. Rust

Here is what happens between our source code and the physical CPU:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. PYTHON (Interpreted Bytecode Pipeline)                                   │
│                                                                             │
│   code.py ──> Bytecode (.pyc) ──> [ CPython Interpreter (in C) ] ──> CPU    │
│                                   ├── Requires Host OS & Dynamic Allocator  │
│                                   ├── Reference Counting & Cyclic GC        │
│                                   └── High Overhead (~10MB+ RAM baseline)   │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│ 2. JAVA (Just-In-Time Compiled Bytecode Pipeline)                           │
│                                                                             │
│   code.java ──> Bytecode (.class) ──> [ JVM (Java Virtual Machine) ] ──> CPU│
│                                       ├── JIT Warmup & Compilation Jitter   │
│                                       ├── Tracing Garbage Collector Pauses  │
│                                       └── Massive Footprint (~30MB-100MB+)  │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│ 3. RUST (Ahead-Of-Time Native Machine Code Pipeline)                        │
│                                                                             │
│   main.rs ──> [ rustc + LLVM ] ──> Pure Machine Code (.bin/.elf) ──> CPU    │
│               Ahead-Of-Time        ├── Zero Runtime / No Interpreter        │
│               Compilation & Safety ├── Direct Memory-Mapped I/O Registers   │
│               Checks               └── Cycle-Accurate (< 2KB RAM footprint) │
└─────────────────────────────────────────────────────────────────────────────┘
```

| Dimension | Python | Java | Rust |
| :--- | :--- | :--- | :--- |
| **Translation** | Interpreted Bytecode | JIT-compiled Bytecode | Ahead-of-Time (AOT) to Machine Code |
| **Runtime Engine** | CPython interpreter binary | JVM (Java Virtual Machine) | None (Zero runtime overhead) |
| **Memory Management** | Reference counting + cyclic GC | Tracing Garbage Collector | Compile-time Ownership (No GC) |
| **RAM Footprint** | Tens of megabytes (10 MB+ minimum) | Hundreds of megabytes (30–64 MB+ minimum) | Bytes to kilobytes (operates within < 2 KB RAM) |
| **Execution Timing** | Non-deterministic (slow, dynamic dispatch) | Jittery (JIT compilation warm-up + GC pauses) | Cycle-accurate, hard-real-time determinism |

---

### Physical Silicon Constraints of Microcontrollers

Microcontrollers (such as the RP2350 on the Raspberry Pi Pico 2 or an STM32H7 on a drone flight controller) typically have:

* **Flash (Code Storage):** `< 4 MB` on-chip (up to 32 MB via external QSPI Flash)
* **RAM (Active Working Memory):** `64 KB` to `1.4 MB`
* **Clock Speed:** `48 MHz` to `600 MHz`

---

### The Triad of Failure: Why Managed Runtimes Break Bare Metal

1. **The RAM Cliff:**
   A standard JVM or Python interpreter binary requires tens of megabytes just to initialize its runtime engine. Its baseline memory overhead exceeds the entire physical RAM of most microcontrollers before user code even starts executing.

2. **The "Stop-the-World" Garbage Collection Problem (Fatal for Robotics):**
   * Imagine our mini-robot reads an IMU / gyroscope 1,000 times per second (1 kHz) to balance on two wheels.
   * If a garbage collector pauses execution for even 10 milliseconds to reclaim unused memory, the control loop misses 10 consecutive sensor updates and motor corrections. The robot loses balance and crashes.

3. **Direct Silicon Access via Hardware Registers:**
   * On a microcontroller, hardware peripherals (GPIO, PWM, I2C, SPI) are controlled by reading from and writing to specific physical addresses in memory (**Memory-Mapped I/O**).
   * In Rust, modifying a hardware register compiles directly into a single CPU `str` (store) instruction. There is no interpreter or virtual machine translating instructions or introducing latency.

> [!NOTE] Ahead-of-Time (AOT)
> In **Ahead-of-Time (AOT)** compilation, 100% of translation, optimization, borrow checking, and type validation occurs on your host machine before the binary touches silicon. When flashing a microcontroller, you write pure machine code directly into the chip's Flash memory. The CPU begins executing instructions immediately on power-up with zero startup lag, zero runtime overhead, and cycle-accurate predictability.

---

## 1.2 Cargo, Toolchains, and Cross-Compilation Targets

In Java and Python, we rarely think about the CPU architecture of our machine. In embedded robotics, **it is the very first thing we configure**.

> [!NOTE] What is "Embedded"?
> - **Embedded** means the computer hardware and software are permanently built inside a specific physical product (like a drone, a microwave, or a mini-robot) to do **one dedicated job**, rather than running on a general-purpose PC.
> - In simple words: Stop thinking like a desktop developer with infinite RAM, a massive operating system, and a monitor. Start thinking like an engineer working directly with **voltages**, **hardware pins**, **sensors**, and **real-time physical limits**.

> [!NOTE] What is "Firmware"?
> - **Firmware** is the low-level code permanently saved inside a hardware chip's Flash memory. It gives the device its basic brain and operating logic right from the moment it gets power.

---

### Host vs. Target: Why We Cross-Compile

* **Host (Your Development Machine):**
  * The laptop or PC you are sitting in front of right now (e.g., your PC running Linux on a 64-bit Intel/AMD processor).
  * Has a full operating system (OS) and gigabytes of RAM.

* **Target (The Robot's Brain):**
  * The actual tiny microcontroller chip inside our robot or drone (for example, an STM32F4 running an ARM Cortex-M4 processor).
  * Has no operating system (Bare Metal) and very little RAM (kilobytes).
  * Microcontrollers can be 8-bit, 16-bit, 32-bit, or 64-bit. For modern robotics, **32-bit is the sweet spot**.

Because the target chip is too small to run a compiler by itself, our laptop (the **Host**) must compile machine code **specifically formatted for the chip's CPU (the Target)**.

This process is called **Cross-Compilation**.

> [!NOTE] What is Cross-Compilation?
> - **Cross-compilation** simply means: compiling code on Machine A (the Host) to produce an executable that runs on Machine B (the Target).
> - It does **not** mean the code runs everywhere. It produces a binary file made **specifically and only** for that target chip!

> [!NOTE] `.bin` vs. `.elf` Files
> - **`.bin` (Raw Binary):** A pure list of machine code `0`s and `1`s that the target chip directly understands.
> - **`.elf` (Executable and Linkable Format):** Contains the exact same machine code `0`s and `1`s, PLUS extra helper information (metadata): memory addresses, function names, variable names, and debugging symbols.
> - **How they are used:** The `.elf` file stays on your laptop so your debugger can show function names and line numbers. When flashing, only the raw machine code goes into the chip. Using `.elf` is the standard and recommended way in embedded Rust.

---

### Anatomy of a Target Triple

A **Target Triple** is like an **order form** you give to the compiler.

When we tell Rust: *"Hey compiler, please translate my code into machine code,"* the compiler needs answers to **4 specific questions:**
1. **Architecture:** What instruction dialect does the CPU speak?
2. **Vendor:** Who made the chip? (Usually `unknown` or `none`)
3. **OS:** What operating system manages the hardware? (For bare metal, this is `none`)
4. **Environment / ABI:** What low-level calling rules must it follow?

> [!NOTE] Why is it called a "Triple"?
> Historically it had 3 parts: `arch-vendor-os`. Later, people added a 4th part for the environment/ABI, but everyone still calls it a "triple"!

---

### Deep Dive: Decoding the ARM Drone Target (`thumbv7em-none-eabihf`)

Let's break down this common drone target string:
* `thumbv7em` — Architecture
* `none` — Vendor
* `none` — Operating System
* `eabihf` — ABI / Environment

#### 1. Architecture: `thumbv7em` (ARMv7-EM & Thumb Mode)
* Standard ARM processors originally used 32-bit long instructions. Later, ARM created a compressed, memory-saving instruction mode called **Thumb** so programs take up less Flash storage on tiny chips.
* `v7em` stands for the **ARMv7-EM** architecture (found in chips like ARM Cortex-M4 and Cortex-M7).
* **What this tells Rust:** *"Only produce machine instructions that a Cortex-M4 or Cortex-M7 processor can understand."*

#### 2. Vendor: `none` (Vendor-Agnostic Silicon)
* Hardware vendor is unassigned (the code is not tied to Apple, PC, etc., and works across chip makers).

#### 3. Operating System: `none` (Bare Metal & `#![no_std]`)
This is one of the most important concepts in embedded programming:
* On your laptop, the target is `x86_64-unknown-linux-gnu`. When your code wants to print text or allocate memory, it asks the Linux kernel to do it.
* On your drone chip, there is **no operating system** — no Linux, no Windows, no helper software.
* When our code starts running, it is in direct and total control of the hardware. This is called **Bare Metal**.
* **What this tells Rust:** *"Do not include any code that tries to make Linux or Windows system calls. There is no OS here to answer them!"* (This is why in embedded Rust, we must add `#![no_std]` at the top of our program).

#### 4. Environment & ABI: `eabihf` (Hardware Float Calling Conventions)
Let's split `eabihf` into two simple parts: **`eabi`** and **`hf`**.

##### Part A: What is an ABI (and EABI)?
* **API** is for humans writing code: *"I call the function `add(5, 10)`."*
* **ABI (Application Binary Interface)** is the rulebook for the CPU hardware:
  * When `add(5, 10)` is called, does `5` go into CPU hardware register `R0` or `R1`?
  * Does the answer `15` come back in register `R0` or on the stack?
* Once code is compiled into `0`s and `1`s, variable names like `a` and `b` disappear. The CPU only has physical storage slots called **Registers** and RAM.
* **EABI (Embedded ABI):** A standard rulebook created by ARM for microcontrollers so that code written in Rust, C, or Assembly can talk to each other without mixing up CPU registers.

```text
Source Code (API):            result = add(5, 10);
                                      │
                         Compiler applies the ABI rules
                                      ▼
Binary Machine Code (ABI):    1. LOAD 5 INTO Register R0
                              2. LOAD 10 INTO Register R1
                              3. CALL FUNCTION `add`
                              4. READ ANSWER FROM Register R0
```

> The ABI is the rulebook that says:
> *"Whenever any function is called on this ARM chip, parameter 1 goes to register R0, parameter 2 goes to R1, parameter 3 goes to R2, parameter 4 goes to R3. The return value comes back in R0."*
> 
> If two pieces of code used different ABIs, Function A would place `5` into `R0`, but Function B would expect it in `R1`. The program would read garbage and crash immediately.

##### Part B: What is Hardware Floating Point (`hf`)?
This is where the magic happens for robotics and drones!

In robotics, we constantly work with decimals (`f32` in Rust), such as measuring angles:
```rust
let angle: f32 = 45.25 * 3.14159;
```

How does a CPU calculate `45.25 * 3.14159`? There are two completely different ways:
* **Software Floating Point (regular `eabi` without `hf`):** The CPU has no decimal math hardware. The compiler has to run dozens of slow integer tricks to estimate the decimal. This is slow!
* **Hardware Floating Point (`eabihf`):** The chip has a dedicated physical math unit called an **FPU (Floating Point Unit)**.
  * The compiler generates direct FPU machine instructions (`VADD`, `VMUL`, `VSQRT`).
  * It passes our `f32` numbers directly inside the chip's ultra-fast dedicated FPU registers (`s0`, `s1`).
  * Math finishes in a single clock cycle!

---

### ARM Microcontroller Target Comparison Table

| Target String | Meaning in Plain English |
| :--- | :--- |
| `thumbv7m-none-eabi` | **Cortex-M3:** Bare-metal MCU, no hardware floating point (uses slow software math for decimals). |
| `thumbv7em-none-eabi` | **Cortex-M4:** Bare-metal MCU with FPU disabled or not present. |
| `thumbv7em-none-eabihf` | **Cortex-M4 / M7:** Bare-metal MCU with Hardware Floating Point enabled (super fast decimal math for drones, motors, and PID controllers). |

---

### Core Silicon Principles: Architecture, Bus Width & Clock Cycles

> [!NOTE] What is a CPU Architecture?
> - An architecture (Instruction Set Architecture, or ISA) is not software. It is the blueprint and dictionary of a processor.
> - It defines:
>   - What physical instructions the circuitry understands (e.g., `ADD`, `SUB`, `JUMP`).
>   - How many physical registers the CPU has and how wide they are.
>   - How memory addresses work.
> - Think of it like spoken languages:
>   - `x86_64` is like English.
>   - `ARM (Thumb)` is like Spanish.
>   - `RISC-V` is like Japanese.
> 
> An ARM chip physically cannot understand an x86 instruction because its internal transistor circuits are wired completely differently.

> [!NOTE] What does "32-bit" or "8-bit" mean?
> - It does **not** mean running 32 instructions at the same time!
> - It means: **The width of the data pipes and registers inside the CPU is 32 bits wide.**
> - In a **32-bit CPU** (like Cortex-M4):
>   - Each general register holds a 32-bit binary number (`00000000 00000000 00000000 00000000`).
>   - In one clock cycle (one "tick" of electricity), the math unit can add two 32-bit numbers together (up to 4 billion!).
>   - A memory address is 32 bits long, meaning it can address up to 4 Gigabytes of memory.
> - In an **8-bit CPU** (like the old Arduino Uno / ATmega328P):
>   - Registers are only 8 bits wide (maximum value 255). To add numbers larger than 255, the CPU has to do multiple slow steps: add the lower byte, carry the 1, and add the upper byte.

> [!NOTE] What is a Clock Cycle?
> - Microcontrollers have a quartz crystal oscillator that sends electrical pulses (clock ticks), say at 100 MHz (100 million ticks per second).
> - On each tick, the CPU does three basic steps:
>   1. **Fetch:** Electricity pulls instruction bits from Flash memory into the CPU.
>   2. **Decode:** Transistors figure out what the bits mean (e.g., `0001` means `ADD`).
>   3. **Execute:** The math unit runs the operation (e.g., adds Register `R0` to `R1`).
> - Most simple instructions take just 1 clock cycle (one tick) to execute.

> [!NOTE] What is ARM?
> - ARM is a company (ARM Ltd., based in the UK).
> - **ARM does not manufacture physical silicon chips!** They do not own chip factories (fabs).
> - Instead, ARM designs the CPU blueprints (the architecture and core circuits) and licenses them to chip manufacturers like STMicroelectronics (STM32), Raspberry Pi (RP2040/RP2350), NXP, and Apple. Those companies take ARM's blueprint, add peripherals (GPIO pins, timers, I2C, USB), and manufacture the real physical chips you buy.

---

### Decoding the Modular RISC-V Target (`riscv32imc-unknown-none-elf`)

While ARM dominates microcontrollers today, **RISC-V** (pronounced *"risk-five"*) is the open-source architecture taking over the embedded industry (found in chips like the ESP32-C3, ESP32-C6, CH32v, and the dual ARM/RISC-V Raspberry Pi 2350).

Let's decode `riscv32imc-unknown-none-elf`:
- `riscv32` — 32-bit RISC-V architecture
- `imc` — Hardware extension capabilities enabled on this chip 
- `unknown` — Hardware vendor
- `none` — Bare metal (no OS, requires `#![no_std]`)
- `elf` — Standard embedded binary format with debug symbols

#### Modular ISA Extensions (`I`, `M`, `A`, `C`, `F`, `D`)
In RISC-V, chips are modular. Manufacturers pick only the hardware extensions they need:
- **`I` (Base Integer):** The fundamental 32-bit integer instruction set (load, store, add, branch). Every RISC-V chip has this.
- **`M` (Hardware Integer Multiply & Divide):** The chip has dedicated silicon circuits for integer multiplication and division. (Without `M`, multiplying two numbers requires dozens of slow addition loops in software).
- **`A` (Atomic Instructions):** Hardware instructions for atomic memory operations (`load-reserved`, `store-conditional`, `atomic swap/add`). Essential for multi-core processors and lock-free concurrency.
- **`C` (Compressed Instructions):** Enables 16-bit compressed instructions (identical in concept to ARM's **Thumb** mode). This reduces compiled binary size by 25%–30% to fit in tiny Flash storage.
- **`F` / `D` (Hardware Floating Point):** Single (`F`, 32-bit `f32`) and Double (`D`, 64-bit `f64`) precision hardware Floating-Point Units (analogous to ARM's `hf`).

| Target | Architecture | Common Example Chips |
| :--- | :--- | :--- |
| `thumbv7em-none-eabihf` | ARM Cortex-M4F / M7 | STM32F4, STM32F7, nRF52840 |
| `riscv32imc-unknown-none-elf` | RISC-V 32-bit | ESP32-C3, BL602 |
| `riscv32imac-unknown-none-elf` | RISC-V 32-bit (+ Atomic instructions) | ESP32-C6, RP2350 (Hazard3 core) |

> [!TIP] Hardware Recommendation for Robotics
> - If we are building an **IoT sensor node**, home automation gadget, or simple WiFi-controlled robot: **ESP32-C3** is cost-effective and capable.
> - If we are building a **self-balancing robot**, a **drone flight controller**, an **inverse kinematics robot arm** or running **Kalman filters**: pick a chip with hardware floating-point support (an ARM Cortex-M4F like **STM32F4 / RP2040+FPU**, or a RISC-V chip with `F/D` like the ESP32-S3 / ESP32-P4).

---

### Toolchain Management: `rustup` vs. `Cargo`
In Java, we manage JDKs and Maven/Gradle. In Python, we juggle `python3`, `pyenv`, and `pip`.

Rust unifies all of this under two tools:
1. **`rustup`**: Manages compiler versions and cross-compilation target modules.
2. **`Cargo`**: The package manager, dependency resolver, compiler orchestrator, and test runner.

#### 1. Adding Target Architecture Support with `rustup`
When we install Rust, it only knows how to build binaries for our laptop (e.g., `x86_64`). To cross-compile for our robot, tell `rustup` to download the pre-compiled `core` library for our chip's architecture:

```bash
# Add ARM Cortex-M4F support:
rustup target add thumbv7em-none-eabihf

# Add RISC-V support:
rustup target add riscv32imc-unknown-none-elf
```

#### 2. Cross-Compiling with `Cargo`
To compile our project for the microcontroller:

```bash
cargo build --target thumbv7em-none-eabihf
```

---

### Concept 1: What is a "Crate"? (Packages in Rust)

In Rust, a **Crate** is simply a package of code.

| Language | What it's called | Where packages come from |
| :--- | :--- | :--- |
| **Python** | Module / package (e.g., `requests`, `numpy`) | **PyPI** (`pip install`) |
| **Java** | Package / JAR library (e.g., `commons-lang.jar`) | **Maven Central** (`pom.xml`) |
| **Rust** | **Crate** (e.g., `heapless`, `embedded-hal`) | [crates.io](https://crates.io) (`Cargo.toml`) |

There are only two kinds of crates:
1. **Binary Crate:** Has a `fn main()` function. It compiles into a real runnable program or firmware file that we can flash to a chip.
2. **Library Crate:** Does **not** have a `main()` function. It is reusable helper code that we import into our project (like importing `math` in Python).

---

### Concept 2: Project Manifest vs. Workshop Config (`Cargo.toml` vs. `.cargo/config.toml`)

#### TOML Syntax Basics
- **TOML** stands for *Tom's Obvious, Minimal Language*.
- Created by Tom Preston-Werner (co-founder of GitHub) as a cleaner alternative to YAML and JSON.
- **Key Features:**
  - Natively supports comments (`#`), trailing commas, and explicit key-value pairs.
  - Does **not** rely on whitespace indentation for hierarchy.

| Concept | TOML Syntax | Cargo Manifest Usage |
| :--- | :--- | :--- |
| **Key-Value Pairs** | `name = "value"` | Setting metadata like `name`, `version`, and `edition` |
| **Tables (Sections)** | `[section_name]` | Declaring namespaces such as `[package]` or `[dependencies]` |
| **Inline Tables** | `crate = { version = "1.0", features = ["derive"] }` | Concise single-line dependency declarations with options |
| **Arrays** | `members = ["crates/core", "crates/cli"]` | Declaring lists of features, authors, or workspace members |

#### Why Two Files? (The Workshop Analogy)

Think of your project like an engineering workshop:

```text
┌─────────────────────────────────────────────────────────────────────────────┐      
│ 1. Cargo.toml (The Project Recipe)                                          │      
│    - What is my project called?                                             │      
│    - What version is it?                                                    │      
│    - What external crates (libraries) do I need?                            │      
│    - Equivalent to: pom.xml in Java, or requirements.txt in Python.         │      
├─────────────────────────────────────────────────────────────────────────────┤      
│ 2. .cargo/config.toml (The Hardware & Workshop Settings)                    │      
│    - What physical chip am I building for right now?                        │      
│    - What command flashes the USB cable when I type `cargo run`?            │      
│    - What low-level memory maps should the linker use?                      │      
│    - Equivalent to: Compiler flags and IDE run configurations.              │      
└─────────────────────────────────────────────────────────────────────────────┘      
```

#### Where do they sit in our project folder?

```text
my_drone_project/                                                                    
├── .cargo/                                                                          
│   └── config.toml      <-- Hardware & toolchain settings (target, runner, flags)   
├── src/                                                                             
│   └── main.rs          <-- Your actual Rust code                                   
└── Cargo.toml           <-- Project name & library dependencies                     
```

---

### Concept 3: Compiler Profiles & Binary Size Optimization (`opt-level`, LTO, `codegen-units`, `panic="abort"`)

Microcontrollers have extremely limited Flash storage. By default, debug builds contain extra checks and unoptimized code that may not fit onto a 64 KB or 512 KB Flash chip.

In embedded Rust, we configure specific compiler flags in `Cargo.toml` to optimize binary size and remove unnecessary overhead:

```toml
[profile.dev]
# Level 1 optimization makes debug builds small enough to fit on the chip:
opt-level = 1

[profile.release]
# Optimize for binary size ("z" or "s") or speed (3):
opt-level = "s"
# Link-Time Optimization: removes dead functions across all crates:
lto = true
# Compile the entire project as 1 unit to maximize optimizations:
codegen-units = 1
# Remove stack unwinding tables; embedded crashes halt immediately:
panic = "abort"
```

Here is what each setting means:
* **`opt-level` (Optimization Level):**
  * `0`: Zero optimization. Compiles fast, but the binary is huge and slow (default on laptops).
  * `1` or `2`: Medium optimization. Makes debug code small enough to fit on our chip while keeping debugging easy.
  * `3`: Maximum speed (runs calculations as fast as possible).
  * `"s"`: Optimize for Size. Makes the binary as few bytes as possible.
  * `"z"`: Optimize for size aggressively (strips even more bytes).
* **`lto = true` (Link-Time Optimization):**
  * Imagine packing a backpack for a hike. If you pack a giant Swiss Army knife but only need scissors, LTO cuts away the knife, saw, and bottle opener. It deletes any unused library function across all crates so it doesn't waste chip space.
* **`codegen-units = 1`:**
  * Normally, the compiler splits our code into multiple chunks so laptop CPU cores can compile them in parallel.
  * Setting this to `1` forces the compiler to analyze all code as a single unit. It takes a few seconds longer to compile, but the compiler finds more opportunities to shrink the binary.
* **`panic = "abort"`:**
  * On a laptop, when a program crashes, it prints a 50-line "stack trace" showing which file and line caused the error.
  * Printing a stack trace requires hundreds of lines of string-formatting code that eats up Flash memory. A drone in mid-air has no screen anyway!
  * `"abort"` tells the chip: *"If an error happens, just stop the CPU immediately. Do not waste memory storing crash-printing code."*

---

### Concept 4: Low-Level Compiler Switches & Linker Scripts (`rustflags` & `-Tlink.x`)

Typing `--target thumbv7em-none-eabihf` on every single terminal command is tedious. In embedded projects, we automate this in `.cargo/config.toml`:

```toml
[build]
# Default compilation target (no need to pass --target in terminal):
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
# Instructs `cargo run` to directly flash the chip using probe-rs:
runner = "probe-rs run --chip STM32F401RE"

# Pass hardware-specific linker scripts:
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```

* **`rustflags`:** Low-level command-line switches passed straight to the raw `rustc` compiler behind the scenes.
* **What is `-C link-arg=-Tlink.x`?**
  * When we compile code on a laptop, the Linux or Windows operating system chooses where to load the program in RAM.
  * Microcontrollers have no operating system. The CPU needs a map of the physical silicon:

```text
Physical Silicon Memory Map:
Flash Memory (Code)  : Starts at address 0x08000000 (Length: 512 KB)
RAM (Variables)      : Starts at address 0x20000000 (Length: 96 KB)
```

* A **Linker Script** (often named `link.x` or `memory.x`) is that physical map.
* `-Tlink.x` tells the compiler: *"Look at this memory map file so you place instructions in physical Flash and variables in physical RAM!"*

---

### Concept 5: Automated Hardware Flashing & Live Debugging (`runner = "probe-rs run"`)

On our laptop, typing `cargo run` launches the program on our laptop's Intel/AMD processor.

But our compiled microcontroller firmware is written for ARM or RISC-V. Our laptop cannot run it directly.

The `runner` setting in `.cargo/config.toml` tells Cargo:
> *"Hey Cargo, when I type `cargo run`, don't run this binary on my laptop! Send it through the USB cable, flash it directly into the microcontroller's Flash memory, and stream its output back into my terminal!"*

`probe-rs` is the modern, open-source tool that handles flashing and live debugging over USB.

---

## 1.3 Variables, Immutability by Default, and Shadowing

### 1. Immutability by Default (`let`)

When we create a variable using `let`, we are not creating a changeable container. We are creating a permanent binding between a name and a value.

```rust
fn main() {
    let target_altitude = 50; // In meters 

    // ERROR! The compiler refuses to compile this:
    target_altitude = 65;
}
```

If we try to change `target_altitude`, the Rust compiler stops us cold with an error.

---

### 2. Explicit Mutability (`let mut`)

What if a value needs to change—like current motor speed, battery percentage, or a loop counter?

We must explicitly ask the compiler for permission by adding the keyword `mut` (short for *mutable*):

```rust
fn main() {
    let mut current_rpm: u16 = 0;

    current_rpm = 1200; // Allowed!
    current_rpm = 1850; // Allowed!
}
```

#### Why force you to type `mut`?
1. **Human Clarity:** When we read 500 lines of flight control code, our brain only has to track the variables marked `mut`. Everything else is guaranteed never to change behind our back.
2. **Compiler Optimization:** When the compiler knows a value never changes, it can place it directly into a CPU hardware register or inline it into physical Flash storage, saving precious RAM.

> [!TIP] NASA Power of 10 Rule #6 Connection
> Remember **Rule 6** from our Power of 10: *"Minimize Variable Scopes and Enforce Immutability."*
> 
> Always start with plain `let`. Only add `mut` if you get a compiler error saying the variable must be modified.

> [!NOTE] Loop Variables
> - In a standard `for` loop, we don't need `mut` for the loop counter itself (`for i in 0..10`).
> - We only need `mut` if we have an accumulator outside the loop or if we explicitly mutate an external variable inside the loop body.

---

### 3. Variable Shadowing: Rust's Secret Weapon

In Rust, we can declare a **new** variable with the **exact same name** as an older variable using the `let` keyword again:

```rust
fn main() {
    let sensor_pin = 4;

    // We "shadow" the old sensor_pin by declaring a new one with 'let':
    let sensor_pin = sensor_pin + 1;

    // sensor_pin is now 5!
}
```

At first glance, it looks like we are modifying a variable, then why not use `let mut`?

**NO! Shadowing is completely different from mutation in two huge ways:**

#### Difference 1: We can change the Type (without losing type safety)
* First, the chip's ADC (Analog-to-Digital Converter) gives us a raw integer between `0` and `4095`.
* Then, we convert that integer into physical distance (a decimal float in centimeters: `15.4 cm`).

With `let mut`, we **cannot** change types:
```rust
let mut distance = 1024;  // distance is an integer (i32)
distance = 15.4;         // ❌ ERROR! Rust says: "Expected integer, found float"
```

With Shadowing, we can safely transform types:
```rust
let distance = 1024;              // 1. Raw integer reading from hardware
let distance = distance as f32;  // 2. Convert to float
let distance = distance * 0.05; // 3. Calibrated distance in cm: 51.2 cm
```

More idiomatic Rust form using distinct, descriptive names:
```rust
let raw_adc: u16 = 1024;
let distance_cm: f32 = (raw_adc as f32) * 0.05;
```

#### Difference 2: The result is frozen (Immutable) again!
Notice that every step uses plain `let` (not `let mut`).
* During the calculation, you shadow the intermediate value.
* Once the math is done, `distance` is completely **immutable**.
* No other function or line of code can accidentally corrupt `distance` later!

```text
┌─────────────────────────────────────────────────────────────────────────────┐                  
│ SHADOWING VS. MUTATION: THE EMBEDDED PERSPECTIVE                            │                  
│                                                                             │                  
│ 1. MUTATION (`let mut x`):                                                  │                  
│    [Same RAM slot] ───> Overwrite value (Type CANNOT change)                │                  
│    Risk: Stays changeable forever throughout the entire function.           │                  
│                                                                             │                  
│ 2. SHADOWING (`let x ... let x`):                                           │                  
│    [Old variable]  ───> Hidden / Dropped by compiler                        │                  
│    [New variable]  ───> Fresh binding (Type CAN change! Can be immutable!)  │                  
│    Safety: Transforms data step-by-step, then locks it down cold.           │                   
└─────────────────────────────────────────────────────────────────────────────┘                  
```

---

### 4. Constants (`const`) vs. Immutable Variables (`let`)

Beginners often ask: *"If `let x = 5;` cannot change, why does Rust also have `const X: u32 = 5;`?"*

Here is the fundamental difference:

| Feature | `let x = 5;` (Immutable Variable) | `const X: u32 = 5;` (Compile-Time Constant) |
| :--- | :--- | :--- |
| **When it runs** | Evaluated at **Runtime** when that line executes | Computed at **Compile-Time** by `rustc` |
| **Type Annotation** | Optional (Rust infers it automatically) | **Mandatory** (you must write `: u32`) |
| **Where it can live** | Only inside functions or blocks `{ }` | **Anywhere** (including global module scope) |
| **RAM Usage** | Uses a stack slot or CPU register | **Zero RAM!** Inlined directly into Flash memory |
| **Naming Convention** | `snake_case` | `UPPER_SNAKE_CASE` |

#### Practical Robotics Example:
```rust
const SYSTEM_CLOCK_HZ: u32 = 168_000_000; // 168 MHz (STM32F4)
const MAX_STEERING_ANGLE_DEG: f32 = 45.0;

fn calculate_pwm(duty_cycle: f32) {
    // Runtime immutable variable (computed dynamically from sensor):
    let current_voltage = read_battery_voltage();
}
```

> [!TIP] Number Readability in Rust
> Notice the underscore in `168_000_000`? In Rust, we can place underscores anywhere inside numeric literals to make them easy to read, just like commas in English (`168,000,000`). The compiler simply ignores them!

#### Deep Dive: Two Silicon Rules for `const` vs. `let`

##### Rule 1: `const` physically cannot use runtime data
A `const` value must be 100% known to the compiler while sitting on your host laptop during compilation:

```rust
// ❌ This will not compile: sensor values do not exist at compile-time!
const CURRENT_TEMP: f32 = read_temperature_sensor();

// ✅ Must use `let`:
let current_temp: f32 = read_temperature_sensor();
```

Any value that comes from:
- A sensor reading
- An ADC voltage conversion
- A function parameter (`fn drive(speed: u8)`)
- Hardware timers or clocks

... **cannot** be a `const`. They must be `let`.

##### Rule 2: The compiler optimizes `let` with Constant Folding (Zero-Cost Abstraction)
In release mode (`cargo build --release`), the LLVM optimizer runs an optimization called **Constant Folding & Propagation**:

```rust
fn calculate_motor_speed() {
    let base_speed = 50;
    let boost = 10;
    let final_speed = base_speed + boost;
    set_motor(final_speed);
}
```

Even though we used `let`, the compiler is smart enough to realize `base_speed` and `boost` never change. It computes `50 + 10 = 60` during compilation on your laptop and translates the whole function into:

```text
set_motor(60); // Uses zero RAM! Directly hardcoded by optimizer
```

#### The Golden Rule: Which One to Use?

| Use `const` for: | Use `let` (immutable) for: |
| :--- | :--- |
| 1. Hardware Pin Definitions (`const MOTOR_PWM_PIN: u8 = 9;`) | 1. All local calculations & algorithm steps |
| 2. Physics & System Constants (`const MAX_RPM: u32 = 6000;`) | 2. Function parameters and return values |
| 3. Buffer Dimensions (`const RX_BUF_SIZE: usize = 64;`) | 3. Intermediate sensor & data transformation steps |

---

### 5. Variable Scope and Block Expressions

In Rust, curly brackets `{ ... }` create a lexical scope.

Variables created inside a block die immediately when the closing bracket `}` is reached. Their memory is instantly reclaimed:

```rust
fn main() {
    let main_motor = 1;

    {
        let temporary_boost = 50;
        println!("Boost active: {}", temporary_boost);
    } // <-- `temporary_boost` DIES right here!

    // ❌ ERROR! temporary_boost no longer exists:
    // println!("{}", temporary_boost);
}
```

#### Expression Blocks (Returning Values from `{}`)

In Python and Java, `{}` is just a syntax container for statements.

In Rust, **a block is an expression that can return a value!**

If you omit the semicolon on the last line of the block, that line becomes the return value of the entire block:

```rust
fn main() {
    let motor_rpm = {
        let battery_voltage = 11.1;   // 3S LiPo battery
        let motor_kv = 920.0;         // Drone motor rating

        battery_voltage * motor_kv    // <-- NO semicolon! This is returned!
    };

    // `motor_rpm` is now 10212.0!
    // And best of all: `battery_voltage` and `motor_kv` are already deleted
    // from memory! Zero wasted RAM on our microcontroller!
}
```

---

### 6. Language Paradigm Comparison: Python vs. Java vs. Rust

| Feature | Python | Java | Rust |
| :--- | :--- | :--- | :--- |
| **Default Behavior** | Mutable (`x = 5; x = 10`) | Mutable (`int x = 5;`) | **Immutable** (`let x = 5;`) |
| **To Make Changeable** | Always changeable | Changeable by default | Must write **`let mut`** |
| **To Make Permanent** | Not enforced (only conventions like `PI = 3.14`) | Write `final int x = 5;` | Default (`let x = 5;`) or `const` |
| **Shadowing in Same Scope** | Allowed, but changes type dynamically at runtime | **Illegal** (compiler error: duplicate variable) | **Fully Supported** with compile-time safety |
| **Block Returns Value** | No | No | **Yes** (`let x = { ... val };`) |

---

### 7. Real-World Case Study: How Physical Reality Becomes Rust Numbers (The ADC & Sensor Signal Chain)

When building robots and drones, how does a real physical obstacle in the world turn into a software number like `15.4 cm`? 

Microcontrollers cannot read real-world continuous signals directly—the CPU only understands binary bits (`0` and `1`). 

Here is the complete journey of how an obstacle in front of your robot becomes a usable number in Rust:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│ HOW PHYSICAL REALITY BECOMES RUST NUMBERS (THE SENSOR SIGNAL CHAIN)         │
│                                                                             │
│ 1. Physical World : Obstacle is placed 15.4 cm in front of the robot.       │
│                               │                                             │
│                               ▼                                             │
│ 2. Analog Sensor  : Infrared detector outputs an analog voltage (1.54 V).   │
│                               │                                             │
│                               ▼                                             │
│ 3. 12-Bit ADC     : Microcontroller slices 0 V – 3.3 V into 0 – 4095.       │
│                     (1.54 V / 3.3 V) * 4095 = Raw Integer 1912              │
│                               │                                             │
│                               ▼                                             │
│ 4. Rust Firmware  : Math formula converts 1912 -> 1.54 V -> 15.4 cm.        │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Step A: Where does 4095 come from? (The 12-Bit ADC)

The silicon hardware block inside your chip that measures analog voltage is called an **ADC** (**Analog-to-Digital Converter**).

Most modern microcontrollers (such as the **STM32**, **ESP32**, and **RP2040**) have a **12-bit ADC**:
* A **1-bit ADC** can only count $2^1 = 2$ values: `0` or `1`.
* An **8-bit ADC** (used in older chips like the Arduino Uno) can count $2^8 = 256$ values: `0` to `255`.
* A **12-bit ADC** has 12 binary switches: $2^{12} = 4096$ possible discrete steps.
* Because digital systems count starting from `0`, the integer range is **`0` to `4095`** ($4096 - 1$).

#### How the ADC Slices Voltage
Your microcontroller typically runs on **`3.3 V`**. The 12-bit ADC chops that `3.3 V` range into **4,095 equal electrical slices**:

| Physical Voltage on Pin | Raw ADC Integer Read by Rust (`u16`) | Electrical Meaning |
| :--- | :--- | :--- |
| **0.0 V** | `0` | Ground (No voltage) |
| **1.65 V** | `2047` | Halfway voltage |
| **3.3 V** | `4095` | Maximum operating voltage |

---

#### Step B: Where does 15.4 cm come from? (The Sensor Calibration Math)

Suppose your robot uses an analog infrared distance sensor (e.g., an optical distance sensor):

1. **Light Reflection:** When an obstacle is `15.4 cm` away, the sensor's optical receiver outputs an analog signal of **`1.54 V`**.
2. **Pin Input:** That voltage enters an ADC-capable GPIO pin on your chip.
3. **Silicon Quantization:** The 12-bit ADC converts `1.54 V` into an integer:
   $$\text{Raw Integer} = \frac{1.54\text{ V}}{3.3\text{ V}} \times 4095 \approx 1912$$
4. **Firmware Math:** In your Rust code, you read the integer `1912` and convert it back into voltage, then into physical centimeters using the sensor manufacturer's datasheet formula:

```rust
// 1. Read raw ADC integer from the hardware pin:
let raw_adc: u16 = adc.read_pin(); // e.g., 1912

// 2. Convert raw integer (0..4095) into physical voltage:
let voltage: f32 = (raw_adc as f32 / 4095.0) * 3.3; // 1.54 V

// 3. Sensor datasheet formula: converts Voltage -> Centimeters:
let distance_cm: f32 = voltage * 10.0; // 15.4 cm!
```

> [!TIP] Why Variable Shadowing Fits Here
> Notice how the measurement transforms step-by-step: raw ADC integer (`u16`) $\rightarrow$ voltage (`f32`) $\rightarrow$ final distance (`f32`). This is where Rust's **variable shadowing** or distinct variable naming shines: intermediate raw values are processed, and the final physical measurement is locked down as an immutable value.

---

### 8. Explicit Type Annotations vs. Type Inference (`let x: u32 = 5;` vs. `let x = 5;`)

In Rust, you can declare variables with or without explicit types:

```rust
let x: u32 = 5; // Explicit type annotation
let x = 5;      // Type inference (compiler figures it out)
```

#### Does this affect machine code or runtime speed?
**NO.** Both forms produce the **exact same machine code**.

At runtime on the physical microcontroller:
* Both versions execute at the exact same clock cycle speed.
* Both take up the exact same number of bytes in memory.
* There is **zero performance penalty**. 

Rust's type deduction is purely a compile-time convenience performed on your development PC before generating the binary.

---

#### The 3 Critical Traps in Embedded Programming

While desktop developers can rely heavily on type inference, embedded developers must be aware of three critical traps:

##### Trap 1: The Hidden Float Trap (`f64` vs. `f32`) ⚠️ (CRITICAL FOR ROBOTICS)
If you write a decimal number without specifying a type:
```rust
let speed = 15.4; // Rust defaults this to f64 (64-bit double precision)!
```

* **Why this is dangerous on microcontrollers:**
  * Most robotics microcontrollers (like the **ARM Cortex-M4F** or **Cortex-M7**) only have a **32-bit hardware FPU** (`f32`). They do **not** have physical hardware circuits for 64-bit `f64` math.
  * If you leave a decimal as `f64`, the compiler is forced to inject hundreds of instructions of **software math emulation**, making your motor and sensor control loops **up to 50× slower**!
* **The Embedded Fix:** Always explicitly specify `f32` for decimals:
  ```rust
  let speed: f32 = 15.4;
  // OR use a type suffix:
  let speed = 15.4f32;
  ```

##### Trap 2: The Hidden Integer Trap (`i32` default)
If you write a whole number without specifying a type:
```rust
let pin_number = 13; // Rust defaults this to i32 (signed 32-bit integer)!
```

* **Why this causes compile errors:**
  * Microcontroller pin numbers, peripheral register bitmasks, and byte buffers almost always expect an unsigned 8-bit integer (**`u8`**).
  * If you pass `pin_number` (`i32`) into a hardware driver expecting `u8`, the compiler stops you because a signed 32-bit integer cannot safely fit into an 8-bit unsigned slot.
* **The Embedded Fix:**
  ```rust
  let pin_number: u8 = 13;
  // OR use a type suffix:
  let pin_number = 13u8;
  ```

##### Trap 3: The "Cannot Infer Type" Compiler Ambiguity
Sometimes the compiler cannot deduce your intended type because multiple types are valid. A classic example is string parsing:
```rust
// ❌ Compilation Error: "cannot infer type"
// The compiler doesn't know whether to parse into u8, u16, u32, or i32:
let sensor_id = "42".parse().unwrap();

// ✅ Solution: Provide an explicit type hint:
let sensor_id: u8 = "42".parse().unwrap();
```

---

#### Practical Rule of Thumb for Embedded Rust

| Situation | What to Write | Engineering Reason |
| :--- | :--- | :--- |
| **Normal local calculations** | `let total = count * 2;` | Clean and readable. Rust infers the type directly from `count`. |
| **Decimals on microcontrollers** | `let dist: f32 = 15.4;`<br>*(or `15.4f32`)* | **Prevents the slow `f64` software-emulation trap.** Guarantees single-cycle hardware FPU execution. |
| **Hardware pins & byte buffers** | `let pin: u8 = 5;`<br>*(or `5u8`)* | Microcontroller registers require exact bit-widths (`u8`, `u16`, `u32`). |
| **Ambiguous function calls** | `let val: u32 = ...;` | Resolves compiler ambiguity when multiple types implement the same method. |

---

### 9. Deferred Initialization: Zero Garbage Memory (Definite Assignment Analysis)

In older languages like C and C++, you can declare a variable without giving it a value:

```c
// Dangerous C code on a microcontroller:
int sensor_val; // UNINITIALIZED!

if (mode == FAST_MODE) {
    sensor_val = read_fast_sensor();
}
// BUG: What if mode is SLOW_MODE?
// `sensor_val` still holds whatever random electrical garbage was left in RAM!
send_motor_pwm(sensor_val); // Motors might spin violently!
```

In microcontrollers, RAM chips power on with random `1`s and `0`s left over from stray capacitance. Reading an uninitialized variable in C means you are operating on **random electrical noise**.

#### How Rust Solves This Safely
In Rust, you **are** allowed to declare a variable before assigning it, but the compiler enforces **Definite Assignment Analysis**:

```rust
let sensor_val: u16; // 1. Declared with an explicit type, but unassigned

if emergency_stop {
    sensor_val = 0;
} else {
    sensor_val = adc.read_pin();
}

// 2. Safe to use! The compiler PROVES that every single branch gave it a value:
set_motor_speed(sensor_val);
```

#### What happens if you forget an execution branch?
If you forget the `else` branch:

```rust
let sensor_val: u16;

if emergency_stop {
    sensor_val = 0;
}

// ❌ COMPILER ERROR:
// Rust stops the build: "used binding `sensor_val` isn't initialized"
set_motor_speed(sensor_val);
```

> [!NOTE] Idiomatic Alternative: `if` as an Expression
> While deferred initialization is 100% safe in Rust, an even cleaner way is to use `if` as an expression directly:
> ```rust
> let sensor_val: u16 = if emergency_stop {
>     0
> } else {
>     adc.read_pin()
> };
> ```
> This binds and initializes the variable in a single step with zero ambiguity.

---

### 10. The Three Storage Classes in Silicon: `let` vs. `const` vs. `static`

When you write firmware, every byte lives in one of three places in physical silicon:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│ HOW RUST VARIABLES LIVE IN MICROCONTROLLER SILICON                          │
│                                                                             │
│ 1. `let x = 5;`        ──> STACK / CPU REGISTERS                            │
│                            • Created dynamically when function runs.        │
│                            • Instantly reclaimed when function exits.       │
│                            • Zero heap memory used.                         │
│                                                                             │
│ 2. `const X: u32 = 5;` ──> INLINED INTO FLASH INSTRUCTIONS (NO MEMORY SLOT) │
│                            • Replaced at compile-time by literal numbers.   │
│                            • Assembly: `MOV R0, #5`                         │
│                            • Has NO physical RAM address!                   │
│                                                                             │
│ 3. `static X: u32 = 5;`──> FIXED PERMANENT SILICON ADDRESS (FLASH or RAM)   │
│                            • Stays at the exact same memory address         │
│                              from power-on until the battery dies.          │
│                            • Placed in `.rodata` (Flash) or `.data` (RAM).  │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Detailed Comparison Table

| Feature | `let` (Local Variable) | `const` (Compile-Time Constant) | `static` (Global Storage) |
| :--- | :--- | :--- | :--- |
| **Physical Memory Location** | Stack (RAM) or CPU Registers (`R0`–`R12`) | None (Inlined into CPU instructions in Flash) | Permanent address in Flash (`.rodata`) or RAM (`.data`/`.bss`) |
| **Memory Address** | Temporary address on the stack | **Has no memory address** | **Permanent fixed memory address** |
| **Lifespan** | Dies when enclosing block `{ }` ends | Lives forever (inlined into code) | Lives forever (from boot to shutdown) |
| **Evaluation Time** | Runtime | Compile-time | Compile-time initialization |
| **Type Annotation** | Optional (inferred) | **Mandatory** | **Mandatory** |
| **Primary Use in Robotics** | Local calculations & algorithm steps | Pin numbers, calibration factors, physics math | Global hardware buffers (DMA queues, UART buffers), shared interrupt state |

#### Sneak Peek: Decoding an Embedded `static` Buffer

```rust
static UART_RX_BUFFER: [u8; 128] = [0; 128];
// │         │              │            │  
// │         │              │            └─ VALUE: "Repeat the number 0, exactly 128 times"
// │         │              └────────────── DATA_TYPE: "A fixed array of 128 bytes"
// │         └───────────────────────────── NAME: Variable name (UPPERCASE because it's static)
// └─────────────────────────────────────── KEYWORD: Permanent fixed address in memory
```

##### 1. What does `[u8; 128]` mean? (The Data Type)

In Python, a list `[1, 2, 3]` can grow or shrink whenever you call `.append()`. That requires a dynamic heap memory allocator (which we banned under **NASA Rule #3**).

In Bare-Metal Embedded Rust, arrays must have a fixed, unchangeable size known at compile time:

$$\text{[ Type ; How many ]}$$

* `[u8; 4]` means: An array of exactly 4 bytes (e.g., `[10, 20, 30, 40]`).
* `[u8; 128]` means: An array of exactly 128 bytes (`u8`), sitting side-by-side in physical RAM.

---

##### 2. What does `[0; 128]` mean? (The Value)

If you need an array with 128 zeros, typing:

```rust
[0, 0, 0, 0, 0, 0, ... /* 128 times! */]
```

would be painful and messy.

Rust gives you a shortcut syntax to fill an array with repeating values:

$$\text{[ Value to repeat ; How many times ]}$$

* `[0; 128]` tells the compiler: "Fill all 128 slots with the number `0`."
* `[255; 4]` produces: `[255, 255, 255, 255]`.

---

##### 3. Why is it used with `static`? (The Hardware Story)

In robotics, **UART** (Serial) is the hardware communication wire that connects your microcontroller to a GPS module, Bluetooth, or your PC.

When the GPS sends data, characters arrive byte-by-byte into the chip.

* The microcontroller needs a permanent parking lot in RAM to store those incoming bytes until your code is ready to read them.
* That parking lot is called a **Buffer**.
* Because the serial hardware runs continuously in the background (even while your main code is doing motor calculations), this buffer cannot be a temporary local variable inside a function (`let`) that dies when the function ends.
* It must sit at a permanent, fixed address in physical RAM from power-on until the battery dies. That is why it is marked **`static`**!

```text
Physical Microcontroller RAM:
┌─────────────────────────────────────────────────────────────┐
│ 0x20000000: UART_RX_BUFFER (128 bytes reserved for GPS)     │
│ [ 0 | 0 | 0 | 0 | 0 | ... 128 zeroed byte slots ... | 0 ]   │
└─────────────────────────────────────────────────────────────┘
```

---



