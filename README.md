# Embedded Systems & Robotics Engineering Vault

<p align="center" style="font-size: 1.15em; color: var(--md-default-fg-color--light); margin-top: -0.5em; max-width: 780px; margin-left: auto; margin-right: auto;">
  An exhaustive, beginner-first masterwork curriculum uniting <strong>Safety-Critical Embedded Rust</strong> and <strong>Industrial Circuit Engineering</strong> for autonomous robots, drones, and precision machinery.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust_2021%2F2024-dea584?style=flat-square&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Architecture-ARM_Cortex--M_%7C_RISC--V-009688?style=flat-square" alt="Architecture">
  <img src="https://img.shields.io/badge/Safety-NASA_JPL_Power_of_10-critical?style=flat-square" alt="NASA JPL Safety">
  <img src="https://img.shields.io/badge/Async-Embassy_Baremetal-3b82f6?style=flat-square" alt="Embassy">
  <img src="https://img.shields.io/badge/Hardware-RP2040_%7C_ESP32--C3-f59e0b?style=flat-square" alt="Hardware">
</p>

---

## 🧭 Master Quick Navigation

- 📖 **Complete Compiled Book**: [`Embedded_Rust.pdf`](embedded_rust/Embedded_Rust.pdf) *(184 Pages, ISO A4 Full-Bleed, In-Page Jumps & Nested Bookmarks)*
- 🛡️ **Safety Engineering Standard**: [NASA Power of 10 Safety Rules](#nasa-power-of-10-rules-for-safety-critical-systems)
- ⚙️ **Full-Stack Symmetry**: [Software & Hardware Parallel Matrix](#️-the-full-stack-symmetry-software--hardware-working-together)
- 🦀 **Track 1**: [Embedded Rust & Firmware Engineering Syllabus](#-track-1-embedded-rust--firmware-engineering-syllabus)
- ⚡ **Track 2**: [Embedded Hardware & Robotics Design Syllabus](#-track-2-embedded-hardware--robotics-design-syllabus)
- 🔬 **Bench Inventory**: [`embedded_hardware/items.list`](embedded_hardware/items.list)

---

## ⚖️ The Full-Stack Symmetry: Software & Hardware Working Together

Autonomous machinery requires simultaneous mastery of the **mind** (firmware) and the **muscles** (circuits). Every unit in this curriculum is built with symmetrical software and hardware counterparts:

| Unit | Track 1: Embedded Rust (Mind) | Track 2: Embedded Hardware (Muscles) |
| :---: | :--- | :--- |
| **Unit 1** | Syntax, Fixed-Width Ints, RAM Physics, AOT Compilation | Circuit Physics, Ohm's Law, Passives (R, C, L), Diodes |
| **Unit 2** | Ownership, Move Semantics, Borrowing, Lifetimes | Power Trees, Battery Chemistries, LDOs, Buck Converters, BMS |
| **Unit 3** | Structs, Enums, Pattern Matching, Typestates | Transistors, Logic-Level MOSFETs, H-Bridges, Motor Drivers |
| **Unit 4** | Zero-Cost Abstractions, Traits, Driver Crates | Sensor Physics, Operational Amplifiers (Op-Amps), Filters, IMUs |
| **Unit 5** | Memory-Mapped I/O (MMIO), Registers, Bitmasking | Wire-Level Communication Buses (GPIO, I2C, SPI, UART, OLED) |
| **Unit 6** | Async Embassy, Real-Time RTOS, Concurrency | Noise, Ground Loops, EMC, Schematics, Multimeters, KiCad PCBs |

---

## 🛡️ NASA Power of 10: Rules for Safety-Critical Systems

> [!IMPORTANT] The Golden Safety Standard for Robotics & Drones
> The **Power of 10** rules were originally authored by Gerard J. Holzmann at NASA's Jet Propulsion Laboratory (JPL) to program deep-space probes and Mars rovers. In autonomous robotics, a crash is not an error log—it is physical damage or personal injury.

### 1. Eliminate All Recursion
* **Rule:** Never write a function that calls itself.
* **Silicon Reality:** Microcontrollers operate with constrained stack memory (e.g., 32 KB to 264 KB). Unbounded recursion or unexpected sensor values can cause stack overflow into data memory, causing hard faults. Use iterative loops (`for` / `while`) with fixed bounds.

### 2. Guarantee Fixed Upper Bounds on All Loops
* **Rule:** Every loop must have an absolute maximum upper limit of iterations.
* **Silicon Reality:** Control loops must meet strict real-time deadlines (e.g. 1 kHz PID loops). Never write `while !sensor_ready()`. Always bound waiting loops with `MAX_RETRIES` or `.take(N)`.

### 3. Ban Dynamic Heap Allocation After Boot (`#![no_std]` + `heapless`)
* **Rule:** Zero dynamic memory allocation (`malloc`, `Vec`, `String`) during operational control.
* **Silicon Reality:** Dynamic allocation causes heap fragmentation. In embedded systems lacking virtual memory, fragmentation eventually triggers allocation failure panics. Pre-allocate all queues and buffers statically using fixed-capacity types (`heapless::Vec`).

### 4. Keep Functions Modular and Under 60 Lines
* **Rule:** Every function must fit within 60 lines of code.
* **Silicon Reality:** Single-purpose, modular functions fit completely on a screen, preventing cognitive overload and making formal verification straightforward.

### 5. Zero Panics — Treat Panicking Code as a Compilation Bug
* **Rule:** Production firmware must be mathematically proven never to invoke panic handlers.
* **Silicon Reality:** A panic halts the CPU and drops PWM signals, causing robots to stall or drones to crash. Never use `.unwrap()`, `.expect()`, or direct unchecked slice indexing (`data[i]`). Use `data.get(i)` and `Option`/`Result`.

### 6. Minimize Variable Scopes and Enforce Immutability
* **Rule:** Variables must default to immutable (`let`), kept within the narrowest practical scope.
* **Silicon Reality:** Restricting mutation prevents accidental state corruption across concurrent tasks and interrupts.

### 7. Zero Unchecked Returns (`#![deny(unused_must_use)]`)
* **Rule:** Explicitly inspect and handle every return value.
* **Silicon Reality:** Ignoring a hardware bus error (e.g. I2C NACK) means operating on garbage data. Enforce `#![deny(unused_must_use)]` and propagate errors with the `?` operator.

### 8. Restrict Macro Metaprogramming in Safety Paths
* **Rule:** Avoid procedural macros or opaque macro loops in real-time control paths.
* **Silicon Reality:** Safety loops require transparent, easily auditable assembly code. Prefer plain functions, `const fn`, and traits over macro magic.

### 9. Pure Safe Rust Only (`#![forbid(unsafe_code)]`)
* **Rule:** Ban `unsafe` blocks from high-level robotic application logic.
* **Silicon Reality:** Isolate raw register accesses inside verified Hardware Abstraction Layers (HALs). Robot decision code, FSMs, and navigation logic must run in 100% verified Safe Rust.

### 10. Enforce Zero-Warning Compilation and Formal Verification
* **Rule:** Zero compiler warnings tolerated; verify safety via Clippy, Miri, and formal verifiers (Kani).
* **Silicon Reality:** Warnings signal latent bugs. Compile with `#![deny(warnings)]` and `#![warn(clippy::pedantic)]`.

#### Safety-Critical Starter Template
```rust
#![no_std]                 // Rule 3: No dynamic heap runtime
#![no_main]                // Standard: Baremetal reset vector boot
#![forbid(unsafe_code)]    // Rule 9: 100% Safe Rust in application logic
#![deny(warnings)]         // Rule 10: Zero compiler warnings tolerated
#![deny(unused_must_use)]  // Rule 7: Every Result and error handled
#![warn(clippy::pedantic)] // Rule 10: Strict linter checks

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // 1. Cut power to all actuator PWM lines immediately (safe shutdown)
    // 2. Set fault status LED to solid red
    // 3. Enter infinite halt loop or trigger watchdog hardware reset
    loop {}
}
```

---

## 🦀 Track 1: Embedded Rust & Firmware Engineering Syllabus

### [Unit 1: Foundations — Rust through an Embedded Lens](embedded_rust/unit_01_foundations/)
* **Chapter 1: [From Java/Python to Rust: Syntax & Execution Model](embedded_rust/unit_01_foundations/ch01_syntax_and_execution_model.md)**
  * 1.1 The Compilation Model: Bytecode & Interpreters vs. Ahead-of-Time (AOT) Machine Code
  * 1.2 Cargo, Toolchains, Cross-Compilation Targets (`thumbv7em`, `riscv32`)
  * 1.3 Variables, Immutability by Default, and Shadowing
  * 1.4 Primitive Types: Fixed-width Integers (`u8`-`u64`, `i8`-`i64`), Floats, Booleans, and Chars
  * 1.5 Functions, Expressions vs. Statements, and Return Values
  * 1.6 Control Flow: `if/else` as Expressions, `loop`, `while`, and `for` Iterators
  * 1.7 Practical Coding Traps: The 7 Compiler Pitfalls That Catch Java/Python Developers
* **Chapter 2: [The Core Mental Model: Ownership & Borrowing](embedded_rust/unit_01_foundations/ch02_ownership_and_borrowing.md)**
  * 2.1 The Problem with Garbage Collection in Real-Time & Microcontrollers
  * 2.2 Ownership Rules & Move Semantics (Why variables move instead of copying by reference)
  * 2.3 References and Borrowing: Immutable (`&T`) vs. Mutable (`&mut T`)
  * 2.4 Aliasing XOR Mutability: How Rust Guarantees Single Hardware Access at Compile Time
  * 2.5 Lifetimes Demystified: Static Data vs. Stack Data in Embedded Contexts
  * 2.6 The `Copy` vs. `Clone` Traits
* **Chapter 3: [Low-Level Types, Bit Manipulation & Hardware Registers](embedded_rust/unit_01_foundations/ch03_low_level_types_and_registers.md)**
  * 3.1 Hexadecimal, Binary Literals, and Byte Slices (`&[u8]`)
  * 3.2 Bitwise Operations: AND (`&`), OR (`|`), XOR (`^`), NOT (`!`), Shifts (`<<`, `>>`)
  * 3.3 Register Bitmasking: Setting, Clearing, and Toggling Individual Bits
  * 3.4 Endianness: Little-Endian vs. Big-Endian Conversions (`to_le_bytes`, `from_be_bytes`)
  * 3.5 Memory Layouts and Alignment: `#[repr(C)]` and `#[repr(packed)]` for Hardware Buffers
* **Chapter 4: Data Modeling for Robotics: Structs & Enums**
  * 4.1 Structs: Classic Structs, Tuple Structs, and Unit Structs
  * 4.2 Methods and Associated Functions (`impl` Blocks)
  * 4.3 Enums with Data: Why Rust Enums Are Algebraic Data Types (Unlike Java/Python Enums)
  * 4.4 Pattern Matching: Exhaustive `match`, Guards, and `if let`
  * 4.5 Modeling Finite State Machines (FSM) for Mini-Robots with Enums
* **Chapter 5: Error Handling without Exceptions**
  * 5.1 Why Embedded Systems Reject Exceptions: Predictability and Code Size
  * 5.2 The `Option<T>` Type: Eliminating `null` / `None` Pointer Errors
  * 5.3 The `Result<T, E>` Type: Idiomatic Hardware Failure Handling
  * 5.4 The `?` (Try) Operator: Propagating Hardware Errors Cleanly
  * 5.5 Custom Error Enums for Robot Subsystems (e.g., `SensorError::I2cTimeout`)

### Unit 2: Abstractions & Safety — Zero-Cost Architecture
* **Chapter 6: Traits and Polymorphism (Interfaces Done Right)**
  * 6.1 Traits vs. Java Interfaces / Python Abstract Base Classes
  * 6.2 Implementing Traits for Structs
  * 6.3 Common Standard Traits: `Default`, `Display`, `Debug`, `PartialEq`
  * 6.4 Static Dispatch (`impl Trait`, Generics) vs. Dynamic Dispatch (`dyn Trait`)
  * 6.5 Why Embedded Rust Favors Static Dispatch (Zero-Cost Abstractions)
* **Chapter 7: Generics and Compile-Time Guarantees**
  * 7.1 Generic Functions and Structs
  * 7.2 Trait Bounds and `where` Clauses
  * 7.3 Monomorphization: How the Compiler Generates Specialized Machine Code
  * 7.4 Const Generics: Fixed-Size Arrays and Ring Buffers without Heap Allocation
* **Chapter 8: The Typestate Pattern (Hardware Safety in Types)**
  * 8.1 The Typestate Pattern Concept: Encoding State into Types
  * 8.2 Safe Pin Reconfiguration: Preventing Writes to Input Pins at Compile Time
  * 8.3 State Machine Enforcement: Preventing Robot Motion Before Initialization
  * 8.4 Zero-Sized Types (ZST) and Compiler Erasure
* **Chapter 9: Working without an Operating System (`#![no_std]`)**
  * 9.1 The Anatomy of `#![no_std]` and `core` vs. `std`
  * 9.2 The Panic Handler: `panic-halt`, `panic-rtt-target`, and `panic-probe-sys`
  * 9.3 Memory Sections: Flash (.text/.rodata), RAM (.data/.bss), Stack, and Vector Table
  * 9.4 Static Variables and Safe Global State (`Atomic` types vs. critical sections)
* **Chapter 10: Unsafe Rust & Direct Memory Access (MMIO)**
  * 10.1 Safe Rust vs. Unsafe Rust: What `unsafe` Actually Means
  * 10.2 Raw Pointers (`*const T`, `*mut T`) and Memory-Mapped I/O (MMIO)
  * 10.3 Volatile Reads and Writes: Preventing Aggressive Compiler Optimizations
  * 10.4 Creating Safe Abstraction Wrappers over Raw Hardware Registers

### Unit 3: The Embedded Rust Hardware Stack
* **Chapter 11: The Architecture of Embedded Rust**
  * 11.1 SVD Files (System View Description) and `svd2rust`
  * 11.2 The PAC (Peripheral Access Crate): Low-Level Register Access
  * 11.3 The HAL (Hardware Abstraction Layer): Safe Peripheral Drivers
  * 11.4 The BSP (Board Support Package): Pinout Mappings and On-Board Devices
  * 11.5 Device Singletons & `take()`: Guaranteeing Only One Instance of a Peripheral
* **Chapter 12: `embedded-hal` & Write-Once Device Drivers**
  * 12.1 The `embedded-hal` Philosophy: Platform-Agnostic Drivers
  * 12.2 Digital I/O Traits (`OutputPin`, `InputPin`)
  * 12.3 Bus Traits (`I2c`, `Spi`, `PwmPin`)
  * 12.4 How to Use Off-the-Shelf Drivers from `crates.io`
  * 12.5 Writing a Custom, Platform-Agnostic Driver for an H-Bridge Motor Controller
* **Chapter 13: Embedded Tooling, Emulation & Debugging**
  * 13.1 Toolchain Setup: `rustup target add`, `cargo-generate`, `flip-link`
  * 13.2 Real-Time Transfer (RTT) and `defmt`: Efficient Binary Logging
  * 13.3 Flashing and Debugging with `probe-rs`
  * 13.4 Microcontroller Emulation: Using Wokwi and QEMU for Instant Testing without Hardware
* **Chapter 14: Interrupts and Real-Time Concurrency**
  * 14.1 Microcontroller Interrupts and NVIC (Nested Vectored Interrupt Controller)
  * 14.2 Writing Interrupt Service Routines (ISRs) in Rust
  * 14.3 Sharing Data Between Interrupts and Main Thread: `RefCell`, `Mutex`, Critical Sections
  * 14.4 Race Conditions, Deadlocks, and Priority Inversion in Embedded Systems

### Unit 4: Actuators, Sensors & Communication (Robotics Peripherals)
* **Chapter 15: GPIO & Digital I/O**
  * 15.1 Push-Pull vs. Open-Drain Outputs
  * 15.2 Pull-Up and Pull-Down Resistors for Sensor Inputs
  * 15.3 Pushbutton Reading and Software Debounce Algorithms
  * 15.4 External Interrupts (Pin Change Interrupts) for Wheel Encoders
* **Chapter 16: Timers, Clocks & Pulse Width Modulation (PWM)**
  * 16.1 Hardware Timers, Prescalers, and Counter Periods
  * 16.2 Understanding PWM: Duty Cycle, Frequency, and Resolution
  * 16.3 Driving DC Motors with H-Bridges (L298N, TB6612FNG)
  * 16.4 Controlling Hobby RC Servo Motors (1ms to 2ms Pulse Width Timing)
* **Chapter 17: Serial Communication & Telemetry (UART)**
  * 17.1 UART Fundamentals: Baud Rate, Parity, Framing
  * 17.2 Polling vs. Interrupt-Driven vs. DMA-Driven Serial
  * 17.3 Streaming Debug Logs and Robot Telemetry to a PC
  * 17.4 Parsing Serial Commands from a Host / Remote Controller
* **Chapter 18: Sensor Buses: I2C (Inter-Integrated Circuit)**
  * 18.1 I2C Protocol: Master/Slave, Addressing, SCL/SDA, Pull-up Requirements
  * 18.2 Handling Bus Errors and Deadlocks
  * 18.3 Interfacing with Inertial Measurement Units (IMU: MPU6050 / ICM-20948)
  * 18.4 Driving Small Displays (SSD1306 OLED) for Robot Status
* **Chapter 19: High-Speed Buses: SPI (Serial Peripheral Interface)**
  * 19.1 SPI Protocol: MOSI, MISO, SCK, CS/SS, and Clock Polarity/Phase (Modes 0-3)
  * 19.2 Reading Time-of-Flight (ToF) Distance Sensors & Optical Flow Sensors
  * 19.3 Interfacing with 2.4GHz Wireless Transceivers (nRF24L01)
  * 19.4 Data Logging to SPI Flash / MicroSD Cards
* **Chapter 20: Analog-to-Digital Conversion (ADC)**
  * 20.1 ADC Fundamentals: Sampling Rate, Resolution (10-bit / 12-bit), Reference Voltages
  * 20.2 Reading Potentiometers, Analog Joysticks, and Light/IR Sensors
  * 20.3 Battery Voltage Monitoring and Low-Battery Failsafes

### Unit 5: Modern Asynchronous Robotics with Embassy
* **Chapter 21: Async/Await on Bare Metal**
  * 21.1 Why Traditional RTOS (FreeRTOS) is Replaced by Async Rust
  * 21.2 The Embassy Architecture: Cooperative Scheduling without Overhead
  * 21.3 The Async Mental Model: `Future`, `poll`, `Waker`, and `await`
  * 21.4 Zero Allocations in Embassy: Static Future Memory Allocation
* **Chapter 22: Embassy Tasks, Timers & Communication Channels**
  * 22.1 Defining and Spawning Multiple Independent Tasks
  * 22.2 Non-Blocking Timers: `Timer::after()`, `Ticker`
  * 22.3 Inter-Task Communication: Embassy Channels and Signals
  * 22.4 Mutexes in Async Rust: Sharing Peripherals Safely Between Tasks
* **Chapter 23: Multi-Subsystem Robot Coordination with Embassy**
  * 23.1 Concurrent Execution: Sensor Polling Task + Motor Drive Task + Telemetry Task
  * 23.2 Software Watchdog and Safety Heartbeats
  * 23.3 Dynamic Priority & Cancellation: Aborting Motor Movements on Obstacle Alerts

### Unit 6: Mini-Robot Projects & Closed-Loop Control
* **Chapter 24: Differential Drive Kinematics & Motor Control**
  * 24.1 Differential Drive Robot Physics: Linear Velocity, Angular Velocity, Wheel Speeds
  * 24.2 Quadrature Encoders: Measuring RPM and Traveled Distance
  * 24.3 Writing a Robust Differential Drive Controller in Rust
* **Chapter 25: Sensor Filtering & State Estimation**
  * 25.1 Sensor Noise: Accelerometer Vibrations & Gyroscope Drift
  * 25.2 Fixed-Point vs. Floating-Point Math on Microcontrollers without FPU
  * 25.3 Implementing a Low-Pass Filter and Complementary Filter for Robot Tilt/Heading
* **Chapter 26: Closed-Loop PID Control**
  * 26.1 PID Concepts: Proportional (P), Integral (I), Derivative (D)
  * 26.2 Anti-Windup, Derivative Kick Mitigation, and Loop Timing
  * 26.3 Implementing a Generic, Reusable `PidController` Struct in Rust
  * 26.4 Tuning Motor Speed and Heading Loops
* **Chapter 27: Autonomous Behaviors & Navigation**
  * 27.1 Infrared Line Following with Multi-Sensor Array & PID
  * 27.2 Obstacle Avoidance using Ultrasonic / ToF Distance Sensors
  * 27.3 Wall Following and Simple Reactive Navigation
  * 27.4 Wall-E Style FSM: Idle, Exploring, Avoiding, Low Battery Warning
* **Chapter 28: Wireless Telemetry & Remote Control**
  * 28.1 Packet Serialization/Deserialization without Heap: `serde` + `postcard`
  * 28.2 Wireless Control via ESP-NOW / BLE / nRF24
  * 28.3 Real-Time Telemetry to a Python/Web Dashboard
  * 28.4 Capstone Review: Complete Architecture of a Rust-Powered Mini-Robot

---

## ⚡ Track 2: Embedded Hardware & Robotics Design Syllabus

### Unit 1: Foundations of Circuit Physics & Passive Components
* **Chapter 1: The Silicon Ground Truth: Voltage, Current, Resistance & Power**
  * Electron drift velocity vs. electromagnetic wave propagation across copper traces.
  * Ohm’s Law ($V = IR$), Kirchhoff’s Current & Voltage Laws (KCL, KVL).
  * Electrical Power & Joule’s Heating ($P = VI = I^2 R$): Thermal limits of tiny components.
  * Conductors, Insulators, and Semiconductor Bandgaps (Silicon vs. Gallium Nitride).
* **Chapter 2: Resistor Architecture & Voltage Slicing**
  * Resistor technologies: Thin-film, thick-film, carbon, and parasitic inductance.
  * Precise Voltage Dividers: The Loading Effect ($R_{\text{load}} \parallel R_2$) and input impedance.
  * Sizing Pull-Up and Pull-Down Resistors: Balancing rise-time ($\tau = RC$) against current consumption.
* **Chapter 3: Capacitors: Electric Fields, Dielectrics & Power Stabilization**
  * Physics of capacitance ($C = \frac{\varepsilon A}{d}$) and dielectric polarization.
  * Ceramic (MLCC) vs. Tantalum vs. Aluminum Electrolytic vs. Polymer capacitors.
  * The DC Bias Trap: Why a 10µF ceramic cap drops to 2µF at rated operating voltage.
  * Equivalent Series Resistance (ESR) and Equivalent Series Inductance (ESL).
* **Chapter 4: Inductors, Magnetic Coils & The Inductive Kickback Trap**
  * Magnetic flux, Faraday’s Law, Lenz’s Law, and back-EMF ($V = -L \frac{di}{dt}$).
  * Why cutting power to a motor generates a negative 100V spike that destroys microcontrollers.
  * Core saturation, hysteresis losses, and toroidal vs. drum inductors.
* **Chapter 5: Semiconductor Physics: Diodes & Voltage Clamping**
  * The P-N junction, barrier potential, and forward voltage drop ($V_f$).
  * Standard Silicon vs. Schottky Diodes (low $V_f \approx 0.2\text{V}$, nanosecond recovery).
  * Zener Diodes (voltage regulation) and TVS Diodes (Electrostatic Discharge / ESD clamping).
  * **The Flyback / Freewheeling Diode**: Sizing current rating and reverse recovery time.

### Unit 2: Power Trees, Battery Management & Voltage Regulation
* **Chapter 6: Power Tree Architecture for Robotics**
  * Designing multi-rail power distribution (e.g. 14.8V LiPo $\rightarrow$ 12V Motors $\rightarrow$ 5V Servos $\rightarrow$ 3.3V MCU).
  * Inrush current, soft-start circuits, reverse polarity protection, and fuse sizing.
* **Chapter 7: Linear Voltage Regulators (LDOs)**
  * Internal topology: Bandgap voltage reference, error amplifier, and pass transistor (NPN vs. P-MOSFET).
  * Dropout voltage ($V_{\text{drop}}$), thermal dissipation ($P = (V_{\text{in}} - V_{\text{out}}) \times I$), and thermal runaway.
  * Output capacitor ESR stability requirements (preventing LDO feedback oscillation).
* **Chapter 8: Switching Regulators (Buck, Boost & Buck-Boost)**
  * High-efficiency DC-DC conversion: Inductor energy storage cycle.
  * Step-Down (Buck) topology: Duty cycle equations ($V_{\text{out}} = D \times V_{\text{in}}$).
  * Step-Up (Boost) topology for battery-powered stepping.
  * Switching frequency tradeoffs (100 kHz vs. 2 MHz): Ripple voltage vs. magnetic coil volume.
* **Chapter 9: Battery Chemistry, Charging & Battery Management Systems (BMS)**
  * LiPo (Lithium Polymer) vs. 18650 Li-ion vs. LiFePO4: Nominal, fully charged, and cutoff voltages.
  * C-Ratings (Continuous vs. Burst discharge) and internal resistance ($R_{\text{int}}$) voltage sag under motor load.
  * Cell balancing (Passive vs. Active) and Over-Voltage / Under-Voltage / Over-Current cutoff circuits.
* **Chapter 10: Circuit Protection & Reverse Polarity Architecture**
  * Diode protection (and why it wastes too much power at high currents).
  * High-side P-Channel MOSFET reverse polarity protection circuits.
  * Ideal Diode Controllers: Zero-voltage-drop active rectification.

### Unit 3: Transistors, Power Switching & Actuator Drivers
* **Chapter 11: Bipolar Junction Transistors (BJT: NPN & PNP)**
  * Current-controlled current source: Base current ($I_B$), Collector current ($I_C$), and Gain ($\beta / h_{FE}$).
  * Saturation vs. Cutoff modes: Sizing base resistors for clean digital switching.
* **Chapter 12: MOSFETs: The Workhorses of Modern Robotics**
  * N-Channel vs. P-Channel: Voltage-controlled field-effect switching.
  * Gate threshold voltage ($V_{GS(\text{th})}$): Why "Logic-Level" MOSFETs are mandatory for 3.3V microcontrollers.
  * On-Resistance ($R_{DS(\text{on})}$), gate capacitance ($C_{\text{iss}}$), and gate charge ($Q_g$).
  * Gate Driver ICs: Supplying high-current pulses to switch MOSFET gates in nanoseconds.
* **Chapter 13: Brushed DC Motors & H-Bridge Topologies**
  * Motor electrical model: Armature resistance ($R$), inductance ($L$), and Back-EMF constant ($K_v$).
  * The 4-Switch H-Bridge: Shoot-through danger and dead-time insertion.
  * Driver ICs: DRV8833, TB6612FNG, and high-current discrete MOSFET H-bridges.
* **Chapter 14: Servo Motors & Stepper Motor Drive Circuits**
  * RC Hobby Servos: Internal feedback potentiometer, 50 Hz PWM timing, and torque sizing.
  * Bipolar Stepper Motors: Microstepping current chopping, A4988 vs. TMC2209 silent drivers.
* **Chapter 15: Brushless DC Motors (BLDC) & 3-Phase Inverters**
  * 3-Phase stator coils and permanent magnet rotors.
  * Sensored (Hall-effect) vs. Sensorless (Back-EMF zero-crossing detection).
  * Electronic Speed Controller (ESC) architecture: 6-MOSFET bridge, gate drivers, DShot digital protocol.

### Unit 4: Sensor Physics, Analog Signal Conditioning & Op-Amps
* **Chapter 16: Operational Amplifiers (Op-Amps) in Robotics**
  * The Ideal Op-Amp model: Infinite input impedance, zero output impedance, virtual short.
  * Voltage Follower / Buffer: Isolating high-impedance sensors from ADC loading.
  * Inverting and Non-Inverting Amplifiers: Sizing gain for weak analog sensors.
  * Differential Amplifiers & Current Shunt Sensing: Measuring high-side motor current.
* **Chapter 17: Analog Filters & Signal Conditioning**
  * Passive RC Low-Pass and High-Pass Filters: Cutoff frequency ($f_c = \frac{1}{2\pi RC}$).
  * Active Op-Amp Filters (Sallen-Key topology) for sharp sensor noise cutoff.
  * Anti-Aliasing Filters: Satisfying the Nyquist-Shannon sampling theorem before the ADC.
* **Chapter 18: Inertial Measurement Units (IMUs: Accelerometers & Gyroscopes)**
  * MEMS physics: Microscopic capacitive silicon comb finger arrays.
  * Coriolis vibratory gyroscopes: Measuring angular velocity in degrees per second.
  * Noise profiles: Accelerometer vibration noise vs. Gyroscope bias drift.
* **Chapter 19: Distance, Position & Environmental Sensors**
  * Time-of-Flight (ToF) Laser / LiDAR sensors (VL53L0X): Photon timing physics.
  * Ultrasonic transducers: Piezoelectric crystals, speed of sound, temperature drift.
  * Optical vs. Magnetic Hall-effect encoders: Quadrature pulse decoding ($A/B$ channels).

### Unit 5: Physical-Layer Communication Buses & Display Hardware
* **Chapter 20: Digital GPIO Electrical Characteristics**
  * Push-Pull vs. Open-Drain / Open-Collector silicon stages.
  * Logic threshold voltages ($V_{IH}, V_{IL}, V_{OH}, V_{OL}$) and noise margins.
  * Slew rate control and drive strength configuration in microcontroller registers.
* **Chapter 21: The I2C Physical Layer (Inter-Integrated Circuit)**
  * Open-drain bus architecture: Why pull-up resistors are physically required on `SDA` and `SCL`.
  * Bus Capacitance limits (400 pF standard) and rise-time calculations.
  * Multi-master arbitration, clock stretching, and addressing collisions.
* **Chapter 22: The SPI Physical Layer (Serial Peripheral Interface)**
  * Synchronous push-pull signaling: High-speed clock lines (up to 50+ MHz).
  * Transmission line reflections, signal termination resistors, and line length limits.
  * Multi-slave configurations: Individual Chip Selects (`CS`) vs. Daisy-chaining.
* **Chapter 23: UART, Asynchronous Serial & Differential Signaling**
  * Baud rate generators, bit timing sampling (16x oversampling), and clock drift tolerances.
  * Logic levels: TTL (0V–3.3V) vs. RS-232 ($\pm 12\text{V}$) vs. Differential RS-485.
* **Chapter 24: Visual Displays & Frame Buffer Hardware**
  * 0.96" OLED displays (SSD1306): Charge pumps (generating 7–10V on-chip), column/page addressing.
  * TFT LCDs (ST7789, ILI9341): Parallel vs. SPI, backlight LED current driver circuits.

### Unit 6: Electromagnetic Compatibility (EMC), Schematics & Lab Tools
* **Chapter 25: Noise, Ground Loops & Decoupling Architecture**
  * The physics of Ground: Ground loops, return current paths, and star-ground topologies.
  * Decoupling capacitor placement: Loop inductance and why capacitor distance from pins matters.
  * Analog Ground (AGND) vs. Digital Ground (DGND) on mixed-signal microcontrollers.
* **Chapter 26: Reading Schematics & Component Datasheets Like an Architect**
  * Decoding pin multiplexing tables, truth tables, and bus timing diagrams.
  * Interpreting "Absolute Maximum Ratings" vs. "Recommended Operating Conditions".
  * Understanding schematic net labels, power symbols, and bill-of-materials (BOM).
* **Chapter 27: The Hardware Lab Bench & Test Instruments**
  * Digital Multimeter (DMM): Measuring voltage, continuity, and measuring current without blowing fuses.
  * The USB Logic Analyzer & PulseView: Sniffing real I2C, SPI, and PWM pulses on your Linux PC.
  * Basic Oscilloscopes: Reading analog waveforms, noise spikes, and voltage ripple.
* **Chapter 28: Prototyping to Production: Breadboards, Perfboards & PCB Design**
  * Breadboard limitations: Parasitic capacitance, loose spring contacts, and current limits (<1A).
  * Stripboards & Perfboards: Reliable soldered prototypes.
  * Introduction to KiCad: Schematic capture, PCB footprint assignment, trace routing, and Gerber generation.

---

## 🔬 Hardware Lab Inventory & Equipment

Complete details and component links are tracked in [`embedded_hardware/items.list`](embedded_hardware/items.list).

* **Microcontrollers**: Raspberry Pi RP2040 (Dual ARM Cortex-M0+), ESP32-C3 Super Mini (RISC-V + Wi-Fi/BLE).
* **Prototyping**: 830-point Solderless Breadboard, MB102 Dual-Rail (3.3V/5V) Power Supply, Jumper Wires (M-M, M-F, F-F).
* **Actuation**: 2WD Smart Robot Car Chassis Kit (Dual DC Gear Motors + Wheels + Caster), SG90 Micro Servos.
* **Sensors**: HC-SR04P (3.3V/5V Ultrasonic Distance), VL53L0X (I2C ToF Laser LiDAR), MPU6050 (6-DOF IMU).
* **Power**: 2× 18650 Li-ion Cells (3.7V 2600mAh), 2-Slot 18650 Battery Holder, USB Smart Battery Charger.
* **Instrumentation & Test**: 24 MHz 8-Channel USB Logic Analyzer (Sigrok/PulseView), Digital Multimeter (DMM).
