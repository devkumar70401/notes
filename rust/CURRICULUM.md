# Rust for Microcontrollers & Mini-Robots: Complete Curriculum

> Tailored for developers with a Java/Python background transitioning to bare-metal embedded systems and robotics with zero prior Rust knowledge.

---

## Unit 1: Foundations — Rust through an Embedded Lens

### Chapter 1: From Java/Python to Rust: Syntax & Execution Model
* 1.1 The Compilation Model: Bytecode & Interpreters vs. Ahead-of-Time (AOT) Machine Code
* 1.2 Cargo, Toolchains, Cross-Compilation Targets (`thumbv7em`, `riscv32`)
* 1.3 Variables, Immutability by Default, and Shadowing
* 1.4 Primitive Types: Fixed-width Integers (`u8`-`u64`, `i8`-`i64`), Floats, Booleans, and Chars
* 1.5 Functions, Expressions vs. Statements, and Return Values
* 1.6 Control Flow: `if/else` as Expressions, `loop`, `while`, and `for` Iterators

### Chapter 2: The Core Mental Model: Ownership & Borrowing
* 2.1 The Problem with Garbage Collection in Real-Time & Microcontrollers
* 2.2 Ownership Rules & Move Semantics (Why variables move instead of copying by reference)
* 2.3 References and Borrowing: Immutable (`&T`) vs. Mutable (`&mut T`)
* 2.4 Aliasing XOR Mutability: How Rust Guarantees Single Hardware Access at Compile Time
* 2.5 Lifetimes Demystified: Static Data vs. Stack Data in Embedded Contexts
* 2.6 The `Copy` vs. `Clone` Traits

### Chapter 3: Low-Level Types, Bit Manipulation & Hardware Registers
* 3.1 Hexadecimal, Binary Literals, and Byte Slices (`&[u8]`)
* 3.2 Bitwise Operations: AND (`&`), OR (`|`), XOR (`^`), NOT (`!`), Shifts (`<<`, `>>`)
* 3.3 Register Bitmasking: Setting, Clearing, and Toggling Individual Bits
* 3.4 Endianness: Little-Endian vs. Big-Endian Conversions (`to_le_bytes`, `from_be_bytes`)
* 3.5 Memory Layouts and Alignment: `#[repr(C)]` and `#[repr(packed)]` for Hardware Buffers

### Chapter 4: Data Modeling for Robotics: Structs & Enums
* 4.1 Structs: Classic Structs, Tuple Structs, and Unit Structs
* 4.2 Methods and Associated Functions (`impl` Blocks)
* 4.3 Enums with Data: Why Rust Enums Are Algebraic Data Types (Unlike Java/Python Enums)
* 4.4 Pattern Matching: Exhaustive `match`, Guards, and `if let`
* 4.5 Modeling Finite State Machines (FSM) for Mini-Robots with Enums

### Chapter 5: Error Handling without Exceptions
* 5.1 Why Embedded Systems Reject Exceptions: Predictability and Code Size
* 5.2 The `Option<T>` Type: Eliminating `null` / `None` Pointer Errors
* 5.3 The `Result<T, E>` Type: Idiomatic Hardware Failure Handling
* 5.4 The `?` (Try) Operator: Propagating Hardware Errors Cleanly
* 5.5 Custom Error Enums for Robot Subsystems (e.g., `SensorError::I2cTimeout`)

---

## Unit 2: Abstractions & Safety — Zero-Cost Architecture

### Chapter 6: Traits and Polymorphism (Interfaces Done Right)
* 6.1 Traits vs. Java Interfaces / Python Abstract Base Classes
* 6.2 Implementing Traits for Structs
* 6.3 Common Standard Traits: `Default`, `Display`, `Debug`, `PartialEq`
* 6.4 Static Dispatch (`impl Trait`, Generics) vs. Dynamic Dispatch (`dyn Trait`)
* 6.5 Why Embedded Rust Favors Static Dispatch (Zero-Cost Abstractions)

### Chapter 7: Generics and Compile-Time Guarantees
* 7.1 Generic Functions and Structs
* 7.2 Trait Bounds and `where` Clauses
* 7.3 Monomorphization: How the Compiler Generates Specialized Machine Code
* 7.4 Const Generics: Fixed-Size Arrays and Ring Buffers without Heap Allocation

### Chapter 8: The Typestate Pattern (Hardware Safety in Types)
* 8.1 The Typestate Pattern Concept: Encoding State into Types
* 8.2 Safe Pin Reconfiguration: Preventing Writes to Input Pins at Compile Time
* 8.3 State Machine Enforcement: Preventing Robot Motion Before Initialization
* 8.4 Zero-Sized Types (ZST) and Compiler Erasure

### Chapter 9: Working without an Operating System (`#![no_std]`)
* 9.1 The Anatomy of `#![no_std]` and `core` vs. `std`
* 9.2 The Panic Handler: `panic-halt`, `panic-rtt-target`, and `panic-probe-sys`
* 9.3 Memory Sections: Flash (.text/.rodata), RAM (.data/.bss), Stack, and Vector Table
* 9.4 Static Variables and Safe Global State (`Atomic` types vs. critical sections)

### Chapter 10: Unsafe Rust & Direct Memory Access (MMIO)
* 10.1 Safe Rust vs. Unsafe Rust: What `unsafe` Actually Means
* 10.2 Raw Pointers (`*const T`, `*mut T`) and Memory-Mapped I/O (MMIO)
* 10.3 Volatile Reads and Writes: Preventing Aggressive Compiler Optimizations
* 10.4 Creating Safe Abstraction Wrappers over Raw Hardware Registers

---

## Unit 3: The Embedded Rust Hardware Stack

### Chapter 11: The Architecture of Embedded Rust
* 11.1 SVD Files (System View Description) and `svd2rust`
* 11.2 The PAC (Peripheral Access Crate): Low-Level Register Access
* 11.3 The HAL (Hardware Abstraction Layer): Safe Peripheral Drivers
* 11.4 The BSP (Board Support Package): Pinout Mappings and On-Board Devices
* 11.5 Device Singletons & `take()`: Guaranteeing Only One Instance of a Peripheral

### Chapter 12: `embedded-hal` & Write-Once Device Drivers
* 12.1 The `embedded-hal` Philosophy: Platform-Agnostic Drivers
* 12.2 Digital I/O Traits (`OutputPin`, `InputPin`)
* 12.3 Bus Traits (`I2c`, `Spi`, `PwmPin`)
* 12.4 How to Use Off-the-Shelf Drivers from `crates.io`
* 12.5 Writing a Custom, Platform-Agnostic Driver for an H-Bridge Motor Controller

### Chapter 13: Embedded Tooling, Emulation & Debugging
* 13.1 Toolchain Setup: `rustup target add`, `cargo-generate`, `flip-link`
* 13.2 Real-Time Transfer (RTT) and `defmt`: Efficient Binary Logging
* 13.3 Flashing and Debugging with `probe-rs`
* 13.4 Microcontroller Emulation: Using Wokwi and QEMU for Instant Testing without Hardware

### Chapter 14: Interrupts and Real-Time Concurrency
* 14.1 Microcontroller Interrupts and NVIC (Nested Vectored Interrupt Controller)
* 14.2 Writing Interrupt Service Routines (ISRs) in Rust
* 14.3 Sharing Data Between Interrupts and Main Thread: `RefCell`, `Mutex`, Critical Sections
* 14.4 Race Conditions, Deadlocks, and Priority Inversion in Embedded Systems

---

## Unit 4: Actuators, Sensors & Communication (Robotics Peripherals)

### Chapter 15: GPIO & Digital I/O
* 15.1 Push-Pull vs. Open-Drain Outputs
* 15.2 Pull-Up and Pull-Down Resistors for Sensor Inputs
* 15.3 Pushbutton Reading and Software Debounce Algorithms
* 15.4 External Interrupts (Pin Change Interrupts) for Wheel Encoders

### Chapter 16: Timers, Clocks & Pulse Width Modulation (PWM)
* 16.1 Hardware Timers, Prescalers, and Counter Periods
* 16.2 Understanding PWM: Duty Cycle, Frequency, and Resolution
* 16.3 Driving DC Motors with H-Bridges (L298N, TB6612FNG)
* 16.4 Controlling Hobby RC Servo Motors (1ms to 2ms Pulse Width Timing)

### Chapter 17: Serial Communication & Telemetry (UART)
* 17.1 UART Fundamentals: Baud Rate, Parity, Framing
* 17.2 Polling vs. Interrupt-Driven vs. DMA-Driven Serial
* 17.3 Streaming Debug Logs and Robot Telemetry to a PC
* 17.4 Parsing Serial Commands from a Host / Remote Controller

### Chapter 18: Sensor Buses: I2C (Inter-Integrated Circuit)
* 18.1 I2C Protocol: Master/Slave, Addressing, SCL/SDA, Pull-up Requirements
* 18.2 Handling Bus Errors and Deadlocks
* 18.3 Interfacing with Inertial Measurement Units (IMU: MPU6050 / ICM-20948)
* 18.4 Driving Small Displays (SSD1306 OLED) for Robot Status

### Chapter 19: High-Speed Buses: SPI (Serial Peripheral Interface)
* 19.1 SPI Protocol: MOSI, MISO, SCK, CS/SS, and Clock Polarity/Phase (Modes 0-3)
* 19.2 Reading Time-of-Flight (ToF) Distance Sensors & Optical Flow Sensors
* 19.3 Interfacing with 2.4GHz Wireless Transceivers (nRF24L01)
* 19.4 Data Logging to SPI Flash / MicroSD Cards

### Chapter 20: Analog-to-Digital Conversion (ADC)
* 20.1 ADC Fundamentals: Sampling Rate, Resolution (10-bit / 12-bit), Reference Voltages
* 20.2 Reading Potentiometers, Analog Joysticks, and Light/IR Sensors
* 20.3 Battery Voltage Monitoring and Low-Battery Failsafes

---

## Unit 5: Modern Asynchronous Robotics with Embassy

### Chapter 21: Async/Await on Bare Metal
* 21.1 Why Traditional RTOS (FreeRTOS) is Replaced by Async Rust
* 21.2 The Embassy Architecture: Cooperative Scheduling without Overhead
* 21.3 The Async Mental Model: `Future`, `poll`, `Waker`, and `await`
* 21.4 Zero Allocations in Embassy: Static Future Memory Allocation

### Chapter 22: Embassy Tasks, Timers & Communication Channels
* 22.1 Defining and Spawning Multiple Independent Tasks
* 22.2 Non-Blocking Timers: `Timer::after()`, `Ticker`
* 22.3 Inter-Task Communication: Embassy Channels and Signals
* 22.4 Mutexes in Async Rust: Sharing Peripherals Safely Between Tasks

### Chapter 23: Multi-Subsystem Robot Coordination with Embassy
* 23.1 Concurrent Execution: Sensor Polling Task + Motor Drive Task + Telemetry Task
* 23.2 Software Watchdog and Safety Heartbeats
* 23.3 Dynamic Priority & Cancellation: Aborting Motor Movements on Obstacle Alerts

---

## Unit 6: Mini-Robot Projects & Closed-Loop Control

### Chapter 24: Differential Drive Kinematics & Motor Control
* 24.1 Differential Drive Robot Physics: Linear Velocity, Angular Velocity, Wheel Speeds
* 24.2 Quadrature Encoders: Measuring RPM and Traveled Distance
* 24.3 Writing a Robust Differential Drive Controller in Rust

### Chapter 25: Sensor Filtering & State Estimation
* 25.1 Sensor Noise: Accelerometer Vibrations & Gyroscope Drift
* 25.2 Fixed-Point vs. Floating-Point Math on Microcontrollers without FPU
* 25.3 Implementing a Low-Pass Filter and Complementary Filter for Robot Tilt/Heading

### Chapter 26: Closed-Loop PID Control
* 26.1 PID Concepts: Proportional (P), Integral (I), Derivative (D)
* 26.2 Anti-Windup, Derivative Kick Mitigation, and Loop Timing
* 26.3 Implementing a Generic, Reusable `PidController` Struct in Rust
* 26.4 Tuning Motor Speed and Heading Loops

### Chapter 27: Autonomous Behaviors & Navigation
* 27.1 Infrared Line Following with Multi-Sensor Array & PID
* 27.2 Obstacle Avoidance using Ultrasonic / ToF Distance Sensors
* 27.3 Wall Following and Simple Reactive Navigation
* 27.4 Wall-E Style FSM: Idle, Exploring, Avoiding, Low Battery Warning

### Chapter 28: Wireless Telemetry & Remote Control
* 28.1 Packet Serialization/Deserialization without Heap: `serde` + `postcard`
* 28.2 Wireless Control via ESP-NOW / BLE / nRF24
* 28.3 Real-Time Telemetry to a Python/Web Dashboard
* 28.4 Capstone Review: Complete Architecture of a Rust-Powered Mini-Robot
