# Fundamentals of Rust

## 1.1 The Compilation Model: Bytecode & Interpreters vs. Ahead-of-Time (AOT) Machine Code

### The Compilation Model

- Java and Python are programs running on top of an intermediary layer ( a Virtual Machine or an Interpreter)
- To write code for a microcontroller powering a robot, we need to understand why that model fails on bare-metal silicon and how Rust's **Ahead-of-Time (AOT)** compilation works.

### The Three Models comparison

Let's see what happens btw our source code and CPU:

---
Python
- code.py $\rightarrow$  Bytecode (.pyc) $\rightarrow$ [CPython Interpreter (C program)] (Needs OS, heap, large RAM, GC) $\rightarrow$ CPU

Java
- code.java $\rightarrow$  Bytecode (.class) $\rightarrow$ [JVM , JIT, GC, 100MB+ RAM] $\rightarrow$ CPU

Rust
- main.rs $\rightarrow$  [rustc + LLVM] (compiles AOT directly into raw CPU instructions) $\rightarrow$ CPU
---

|Dimension|Python|Java|Rust|
|---|---|---|---|
|**Translation**|Interpreted bytecode|JIT compiled bytecode|Ahead-of-Time (AOT) to Machine Code|
|**Runtime Engine**| CPython interpreter binary|JVM (Java Virtual Machine)| None (Zero Runtime)|
|**Memory Management**|Reference counting + GC | Tracing GC | Compile-time Ownership (No GC)| 
|**RAM Footprint**|Tens of Megabytes (Min 10MB+)|Hundreds of Megabytes (Min 30-64MB)|Bytes to Kilobytes (Can run in 2KB RAM)|
|**Execution Timing**|Non-deterministic (slow, dynamic)|Jittery (JIT warm up + GC pauses)| Cycle-accurate & Deterministic|

### Why this matters for Microcontrollers & Mini-Robots

Microcontrollers (like the RP2350 on the Raspberry Pi Pico or an STM32H7) typically have:

- Flash (Storage for your code): < 4MB (on chip) / 32MB (via SPI)
- RAM (active working memory): 520KB to 1.4MB
- Clock Speed : 150 MHz to 600 MHz

#### The problems with Java & Python on Bare Metal:

1. **The RAM Cliff**: A standard JVM cannot even start up because its base overhead exceeds the total RAM of the microcontroller.
2. **The "stop the World" GC Problem (Catastrophic for Robotics)**: 
   1. Imagine our mini-robot is reading a gyroscope at 1000 times per second (1 KHz) to balance on two wheels.
   2. If a GC kicks in for even 10 milliseconds to free unused objects, the motor loop misses 10 critical updates. The robot loses balance and crashes.
3. **Hardware Registers (Direct Silicon Access)**: 
   1. On a Microcontroller, hardware pins (GPIO, PWM, I2C) are controlled by writing to specific addresses in memory (Memory-Mapped I/O).
   2. In Rust, writing to a pin compiles into a single CPU str (store) instruction. There is no interpreter or VM translating our intention.

### Key Takeaway

**Ahead-of-Time (AOT) compilation** means 100% of the translation, optimization, and type checking happens on our computer before the program touches the chip. When you flash a microcontroller, you are writing pure machine code directly into the chip's Flash memory. The CPU starts executing instructions immediately on power-up.

## 1.2. Cargo, Toolchains, and Cross-Compilation Targets

In Java and Python, we rarely think about the CPU architecture of our machine. In embedded robotics , **it is the very first thing we configure**.

>NOTE on `Embedded`
>- "Embedded" means that the computer hardware and software are permanently built inside a specific physical product to do one dedicated job, rather than running on a general-purpose computer.
>- when "Embedded" is said it means that stop thinking like a desktop software developer who has infinite power, a massive operating system, and a monitor. They want you think an engineer working directly with **voltages**, **registers**, **sensors**, and **real-time physical constraints**.

>NOTE on `Firmware`
> - Firmware is a specific type of software that is permanently burned into a hardware device's non-volatile memory (like Flash) to give that device its basic instructions and operating logic.

### "Host" vs "Target"

Host
- The machine we are sitting in front of right now (our PC running Linux on an x86_64 Intel/AMD Processor).
- It has OS.
- Generally, 64 bit.

Target
- The actual chip inside our robot or drone (for example, an STM32F4 running an ARM cortex-M4 processor).
- It has no OS.
- They can be 8,16,32,64 bits. But "32" bit is sweet spot.

Therefore, our laptop (the Host) must compile machine code **specifically formatted for the chip's CPU (The Target)**.

This Process is called **Cross-Compilation**.

> NOTE on `Cross-Compilation`
> - Cross complation means: compiling code on Machine A (The Host) to produce an executable that runs on Machine B (The target).
> - It does not mean the resulting binary runs everywhere. In fact, it produces a binary made **specifically and only** for that target chip!.

> NOTE on `.bin` and `.elf`
> - `.bin` file is machine code executable that only contains pure sequence of 0 and 1 that is directly understand by target machine.
> - `.elf` It stands for "Executable and Linkable Format". It contains the exact same machine code 0101.... PLUS Metadata: where to load each chunk in memory, symbol names (function names, variables), and debugging information. It is divided into structured sections: ELF Header, Sections/Segments, Memory Addresses, Symbol & Debug Table (DWARF).
> - .elf is only for Host in both ways only the raw machine code will go to the target. 
> - and using .elf is standard and recommended way.

### What is a "Target Triple"?

- Target Triple is like an **order from** us to the compiler.
- When we tell Rust or C: "Hey Compiler, please translate my code into machine language," the compiler needs answers to **4 specific questions:**
  - **What dialect/instruction set does the CPU speak? (Architecture)**
  - **Who made the Board? (Vendor - often just unknown or none)**
  - **What operating system manages the hardware? (OS)**
  - **What low-level calling convention or C library rules must it follow? (Environment / ABI)**

> NOTE
> - It is historically called a "triple" because it originally had 3 parts: `arch-vendor-os`. Over the years, people added a 4th part for the environment/ABI, but the old name "triple" stuck around!

Let's decode the drone target: `thumbv7em-none-eabihf`
- thumbv7em - Arch
- none - Vendor
- none - OS
- eabihf - ABI / Environment

#### 1. thumbv7em - The Instruction Dialect

- **ARM Processors** originally had 32-bit instructions. Later, ARM invented a compressed, ultra-efficient instruction mode called "Thumb" to save flash memory on tiny microcontrollers.
- `v7em` stands for **ARMv7-EM**  Architecture (found in chips like ARM Cortex-M4 and Cortex-M7).
- What this tells Rust: "Only emit machine instructions that a Cortex-M4/M7 processor can understand."

#### 2. none - The Vendor
- Hardware vendor is unspecified (not tied to Apple, PC, etc.)

#### 3. none - The Operating System
This is one of the most important concepts in embedded programming!

- On your laptop, the triple is `x86_64-unknown-linux-gnu`. When your Rust program wants to print to screen or allocate memory, it asks the linux kernel (linux) to do it.
- On your drone chip, there is no operating system - no linux, no windows, no FreeRTOS.
- When our code starts running, it is in total control of the hardware directly. This is called Bare Metal.
- What this tells Rust: "Do not include code that tries to make Linux/Windows system calls. There is no OS here to answer them!" (This is why in embedded Rust, we must write `#![no_std]`)

#### 4. `eabihf` - This is the part that looks confusing

Let's split `eabihf` into two parts: `eabi` and `hf`.

##### Part A: What is an ABI (and EABI)

- ABI stands for Application Binary Interface.
- It is a interface between CPU and Binary Code (Machine level)
- API (Application Programming Interface) is for humans writing source code: "I call function calculate_speed(distance, time)".
- ABI is for the CPU Hardware:
  - "When calculate_speed is called, does distance go into CPU register R0 or R1?"
  - "Does the result come back in Register R0 or on the stack?"
- An ABI is not for human users; it is for pieces of compiled binary machine code talking to each other.
- EABI means Embedded ABI: It's a standard rulebook created by ARM for microcontrollers so that code written in Rust, C, or Assembly Language can call each other without mixing up CPU registers.

ABI in Detail:
- Once code is turned into 0s and 1s, variable names like a and b disappear. The CPU only has physical hardware storage slots called Registers and RAM.
- Suppose we write add(5, 10) in Rust, and it calls an add function compiled from C.
  - How does the CPU pass 5 and 10?
  - Does it put 5 in hardware register R0 and 10 in R1?
  - or does it put 5 in R1 and 10 in R2?
  - Does it put the answer 15 back in R0?

```
Source Code (API):            result = add(5,10);

         compiler applies the ABI rules

Binary Machine Code:          LOAD 5 INTO Register R0
(ABI at work)                 LOAD 10 INTO Register R1
                              CALL FUNCTION `add`
                              READ ANSWER FROM Register R0.
```

> The ABI is the rulebook that says:
> - "Whenever any function is called on this ARM chip, parameter 1 goes to register R0, parameter 2 goes to R1, parameter 3 goes to R2, parameter 4 goes to R3. The return value must be in R0."
> 
> If two pieces of code were compiled with different ABIs, function A would place 5 into R0, but function B would expect it in R1. The program would read garbage and crash immediately.


##### Part B: What does hf (Hardware floating point) mean?
This is where the magic happens for robotics and drones!

In code, we often work with decimals (floating point numbers, like f32 in Rust):
```let angle: f32 = 45.25 * 3.14159;```

How does a CPU calculate `45.25 * 3.14159`?

There are two completely different ways:

If we compile with regular eabi (software floating points), the compiler passes numbers using standard general-purpose registers (r0,r1).

If we compile with eabihf (hardware floating point):

- The compiler generates direct FPU machine instructions (VADD, VMUL, VSQRT)
- It passes our f32 numbers directly inside the chip's ultra-fast dedicated FPU registers (s0,s1).

#### Quick Comparison

|Target String|Meaning in Plain English|
|---|---|
|thumbv7m-none-eabi|Cortex-M3: Bare-metal mcu, no hardware floating point (uses software math for decimals).|
|thumbv7em-none-eabi|Cortex-M4: Bare metal microcontroller, FPU disabled or not present|
|thumbv7em-none-eabihf|Cortex-M4/M7: Bare-metal mcu, Hardware floating-point enabled (super fast decimal for drones, motors, PID, controllers).|

>Note on `Architecture`
> - An architecture (Instruction Set Architecture, or ISA) is not software or code. It is the blueprint and dictionary of a processor.
> - It defines:
>     - What instructions the hardware circuitry physically supports (e.g., ADD, SUB, JUMP).
>     - How many physical registers the CPU has and their sizes.
>     - How memory addresses work.
> - Think of it as spoken language:
>     - x86_64 is like English.
>     - ARM (Thumb) is like Spanish.
>     - RISC-V is like japanese.
>
> An ARM chip physically cannot understand an x86 instruction because its interal transistor circuits are wired completely differently.

> Note on `32-bit` or `64-bit`
> - It does not mean running 32 instructions in one go!
> - It means: The width of the data pipes and registers inside the CPU is 32 bits wide.
> - In a 32-bit CPU (like cortex M4):
>     - Each general register holds a 32-bit binary number (00000000 00000000 00000000 00000000).
>     - In one clock cycle (one "tick" of electricity), the math unit ALU can add two 32-bit numbers together.
>     - A memory address is 32 bits long, meaning it can address up to $2^{32}$ (4 Gigabytes) of memory.
> - In a **8-bit CPU** (like the old arduino Uno / ATmega328P):
>     - Registers are only 8 bits wide (max value 255). To add numbers larger than 255, the CPU must do multiple steps: add the lower byte, carry the 1, add the upper byte.


> Note on `Clock Cycle`
> - The CPU has a quartz crystal oscillator that sends electrical pulses (clock ticks), say at 100 MHz (100 Million times per second)
> - On each tick:
>     - **Fetch:** Electricity pulls instruction bits from Flash ROM into the CPU.
>     - **Decode:** Transistors decode what the bits mean (e.g., 0001 means ADD).
>     - **Execute:** The math unit runs the operation (e.g., adds Register `R0` to `R1`).
>
> Most basic instructions take 1 clock cycle (one tick) to execute.

> Note on `ARM` 
> - ARM is a company (ARM Ltd., based in the UK).
> ARM does not make physical chips! They do not own chip factories (fabs). Instead 















