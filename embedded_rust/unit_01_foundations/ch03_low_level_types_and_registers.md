# Chapter 3: Low-Level Types, Bit Manipulation & Hardware Registers

## 3.1 Hexadecimal, Binary Literals, and Byte Slices (`&[u8]`)

In Chapters 1 and 2, we mastered how Rust protects system memory on the stack and ensures that variables never point to dead data. But when building autonomous robots, drones, and embedded systems, our code does not live in an abstract mathematical universe—it lives directly on physical silicon chips.

A microcontroller doesn't understand high-level concepts like "customer objects" or "json strings." At the physical copper level, a microcontroller is connected to external sensors, motor drivers, and radio transmitters through wires carrying electrical voltages:

* A wire with **0 Volts ($0\,\text{V}$)** represents a logic `0`.
* A wire with **3.3 Volts ($3.3\,\text{V}$)** represents a logic `1`.

To command motors, read gyroscopes, and send radio telemetry, we must speak the language of raw bits, hexadecimal addresses, and byte buffers. 

In this section, we will strip away all mystery and learn how Rust lets us interact directly with raw hardware bytes with **zero runtime overhead** and **mathematical memory safety**.

---

### 1. The Story of the 8 Light Switches and the Nickname Notebook

Imagine you are standing in front of a wall in a robotics lab. On that wall is a row of **8 individual toggle switches**:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                   THE 8 TOGGLE SWITCHES (1 BYTE = 8 BITS)                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│     Switch 7   Switch 6   Switch 5   Switch 4   Switch 3   Switch 2   Switch 1   Switch 0 │
│       [▲]        [▼]        [▲]        [▲]        [▼]        [▼]        [▼]        [▲]    │
│        1          0          1          1          0          0          0          1     │
│       ON         OFF        ON         ON         OFF        OFF        OFF        ON     │
│     (3.3V)      (0V)      (3.3V)     (3.3V)       (0V)       (0V)       (0V)     (3.3V)   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

1. **A Bit:** Each individual switch can only be in one of two states: **UP (`1`)** or **DOWN (`0`)**. In silicon, this single switch is a **Bit** (the smallest unit of digital data).
2. **A Byte:** When you group **8 switches together**, you have created a **Byte** (also called an **Octet**).
3. **The Total Combinations:** Since each switch has 2 states, 8 switches can produce:
   $$2 \times 2 \times 2 \times 2 \times 2 \times 2 \times 2 \times 2 = 2^8 = 256 \text{ unique patterns (from } 0 \text{ to } 255\text{)}$$

#### Why Decimal Fails for Hardware Control
If you want to tell a fellow robotics engineer which switches are currently flipped UP, you *could* say:
> *"The switch board is currently set to decimal number **177**."*

Think about how useless that is! If someone says "177", can you instantly tell whether Switch 5 is ON or OFF? No! You would have to sit down with a pencil and divide 177 by 2 repeatedly to reconstruct which physical switches are ON.

#### Solution 1: Binary Literals (`0b...`)
Instead of decimal, you can write out the physical position of every single switch directly. In Rust, we prefix binary numbers with **`0b`**:

```rust
let motor_control_byte = 0b1011_0001;
```

Now, looking at `0b1011_0001`, your eyes immediately see the exact position of every single physical wire: Switch 7 is `1`, Switch 6 is `0`, and so on.

#### Solution 2: The Hexadecimal Shortcut (4-Bit Nibbles)
Binary is wonderful for 8 switches. But modern microcontrollers (like the ARM Cortex-M4 or ESP32) have **32-bit registers**. 

Imagine writing out 32 switches by hand in binary:
```rust
let gpio_direction = 0b01000000000000100000000000000000; // 😱 Headache-inducing!
```
One missing zero or one accidental extra digit, and your motor spins in reverse or your drone crashes into a wall!

Engineers solved this by splitting 8 switches into two groups of 4 switches. A 4-bit group is called a **Nibble**:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                      FROM BINARY SWITCHES TO HEXADECIMAL                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   Full 8-Bit Byte:               1   0   1   1   0   0   0   1              │
│                                 └───┬───┘   └───┬───┘                       │
│   Split into 4-bit Nibbles:     [ 1 0 1 1 ] [ 0 0 0 1 ]                     │
│                                      │           │                          │
│   Each 4-bit value (0 to 15):       11           1                          │
│                                      │           │                          │
│   Single Hexadecimal Digit:          B           1     ──► Hex: 0xB1        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

Because 4 bits can produce exactly $2^4 = 16$ possible combinations ($0$ through $15$), we can represent every 4-bit nibble using **one single symbol**:

| Binary (4 Bits) | Decimal Value | Hexadecimal Digit |
| :---: | :---: | :---: |
| `0000` | 0 | `0` |
| `0001` | 1 | `1` |
| `0010` | 2 | `2` |
| `0011` | 3 | `3` |
| `0100` | 4 | `4` |
| `0101` | 5 | `5` |
| `0110` | 6 | `6` |
| `0111` | 7 | `7` |
| `1000` | 8 | `8` |
| `1001` | 9 | `9` |
| `1010` | 10 | `A` |
| `1011` | 11 | `B` |
| `1100` | 12 | `C` |
| `1101` | 13 | `D` |
| `1110` | 14 | `E` |
| `1111` | 15 | `F` |

> [!NOTE] What is Hexadecimal?
> Hexadecimal (base-16) is simply a compact human shorthand for binary. **Exactly two hex characters represent exactly one 8-bit byte** (`0x00` = 0, `0xFF` = 255). It adds zero performance cost and zero magic—it is purely a way for human eyes to read raw binary bits without errors.

---

### 2. Preemptive Syntax Training: Low-Level Literals in Rust

Before writing complex driver code, let's deconstruct every single syntax token and prefix Rust provides for handling raw numbers and bytes.

#### 1. Number Prefixes (`0b`, `0x`, `0o`)
In Rust, the prefix before a number tells the compiler what base you are writing in:

```rust
let binary_val  = 0b1111_0000; // Binary (Base 2)
let hex_val     = 0xF0;        // Hexadecimal (Base 16)
let octal_val   = 0o360;       // Octal (Base 8)
let decimal_val = 240;         // Decimal (Base 10)

// All 4 variables hold the exact same electrical byte (240) in silicon!
```

#### 2. The Visual Digit Separator (`_`)
Long numbers in C or Python are easy to mistype. Rust allows you to place underscore characters (`_`) anywhere inside a number literal to group digits:

```rust
// Readable binary (grouped into 4-bit nibbles):
let mask = 0b1100_1010_1111_0000_u16;

// Readable 32-bit register address (grouped into 16-bit halves):
let gpio_base = 0x4002_0000_u32;
```
> [!TIP] Zero Cost of `_`
> The underscore `_` is completely discarded by the compiler during parsing. It generates zero machine instructions and has zero impact on runtime performance. Use it generously to keep hardware addresses readable!

#### 3. Explicit Type Suffixes (`_u8`, `_u16`, `_u32`, `_usize`)
By default, Rust infers integer literals as `i32` (signed 32-bit integer). However, microcontroller hardware registers and raw byte buffers are almost always **unsigned integers** (`u8`, `u16`, `u32`). You can affix the type directly to the literal:

```rust
let small_reg: u8  = 0xFF_u8;     // Exactly 8 bits (1 byte)
let bus_addr: u32  = 0x4000_u32;   // Exactly 32 bits (4 bytes)
let array_idx      = 12_usize;     // CPU architecture pointer-sized int
```

#### 4. Byte Literals (`b'...'`)
If you write `'A'`, Rust creates a **`char`**, which is a 4-byte (32-bit) Unicode scalar value! But serial ports and radio transceivers transmit raw 1-byte ASCII characters. 

Putting a `b` in front of single quotes creates a **`u8` byte literal**:

```rust
let ascii_char: char = 'A';  // ❌ 4 BYTES (Unicode: U+0041)
let ascii_byte: u8   = b'A'; // ✅ EXACTLY 1 BYTE (Decimal 65, Hex 0x41)
```

#### 5. Byte String Literals (`b"..."`)
Putting a `b` in front of double quotes creates a **byte string literal**. Its type is a reference to a fixed array of unsigned bytes (`&'static [u8; N]`):

```rust
let handshake = b"PING"; // Type: &'static [u8; 4]
// In memory: [0x50, 0x49, 0x4E, 0x47] stored in Flash ROM (.rodata)
```

---

### 3. Silicon-Level Ground Truth: Why Microcontrollers Live in Hex

Why do microchip datasheets (such as STMicroelectronics STM32 or Raspberry Pi RP2040) specify memory maps in hexadecimal rather than decimal?

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                 MICROCONTROLLER 32-BIT MEMORY MAP (ARM CORTEX-M)            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   Memory Boundary       Address in Hex        Address in Decimal            │
│   ──────────────────────────────────────────────────────────────            │
│   Top of Memory         0xFFFF_FFFF           4,294,967,295 (4 GB limit)    │
│   System Control (NVIC) 0xE000_E000           3,758,153,728                 │
│   Peripherals (AHB1)    0x4002_0000 (GPIOA)   1,073,872,896                 │
│   Peripherals (APB1)    0x4000_4400 (USART2)  1,073,759,232                 │
│   SRAM (RAM)            0x2000_0000           536,870,912                   │
│   Flash Memory (ROM)    0x0800_0000           134,217,728                   │
│   Bottom of Memory      0x0000_0000           0                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### The Copper Bus Reality: 1 Hex Digit = 4 Physical Silicon Wires
In a 32-bit ARM CPU, the address bus consists of **32 microscopic copper traces** running across the silicon die from the processor core to the memory bus matrix.

When the CPU reads address `0x4002_0000`:
* Look at the first hex digit: `4` is binary `0100`. The CPU activates line #30 and leaves lines #31, #29, and #28 at 0V.
* Every single hex character maps **1-to-1** to a group of 4 physical bus wires.
* In decimal (`1,073,872,896`), there is no human way to know which bus lines are active without a scientific calculator!

---

### 4. Arrays vs. Byte Slices: The Bread Loaf & The Magnifying Glass

When reading packets from a GPS, an IMU (gyroscope/accelerometer), or a radio transceiver, data arrives as a sequence of bytes. 

To handle this data efficiently without copying bytes or wasting precious SRAM, Rust provides two related but fundamentally different types: **Fixed Arrays** and **Byte Slices (`&[u8]`)**.

#### The Tangible Story: The Whole Loaf vs. The Slicing Frame
Imagine a bakery:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                   THE BREAD LOAF VS. THE SLICING FRAME                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. THE FIXED ARRAY: `[u8; 6]` (The Entire Baked Loaf)                      │
│     [ Byte 0 ] [ Byte 1 ] [ Byte 2 ] [ Byte 3 ] [ Byte 4 ] [ Byte 5 ]       │
│     • The array OWNS all 6 bytes.                                           │
│     • Size is fixed forever in stone at compile time.                        │
│     • Moving the array copies all 6 bytes across memory.                    │
│                                                                             │
│  2. THE BYTE SLICE: `&[u8]` (The Magnifying Glass / Window)                 │
│                       ┌───────────────────────┐                             │
│     [ Byte 0 ] [ Byte 1 ] [ Byte 2 ] [ Byte 3 ] [ Byte 4 ] [ Byte 5 ]       │
│                       └───────────────────────┘                             │
│                       ▲                       ▲                             │
│                       │                       │                             │
│     The Slice starts here (ptr)      Slice length = 3 bytes                 │
│                                                                             │
│     • The slice does NOT own the bread! It only looks at it.                │
│     • It is a lightweight viewing window into existing memory.              │
│     • Creating a slice copies ZERO bytes of payload data!                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### What is an Array (`[T; N]`)?
An array in Rust has its size `N` baked directly into its type definition:

```rust
let packet_buffer: [u8; 4] = [0xAA, 0x55, 0x01, 0xFF];
```
The type is `[u8; 4]`. An array of `[u8; 4]` is a completely different type from `[u8; 5]`! You cannot pass a `[u8; 4]` to a function expecting a `[u8; 5]`.

#### What is a Byte Slice (`&[u8]`)?
In networking and robotics, packet sizes fluctuate. A radio packet might be 12 bytes long, while the next one is 64 bytes long. We cannot write a separate function for every possible array length!

This is where a **Byte Slice (`&[u8]`)** enters. A slice is a borrowed reference to a contiguous sequence of bytes anywhere in memory (whether in Flash ROM, on the Stack, or in Static RAM).

---

### 5. Silicon Architecture: The Anatomy of a Fat Pointer

How does a slice reference `&[u8]` actually look in microcontroller registers and memory?

In languages like C, a pointer (`uint8_t*`) is just a single 32-bit memory address. The pointer knows where the data starts, but it has **no idea how long the buffer is**! If you read past the end in C, you trigger a silent buffer overflow and compromise system security.

In Rust, a slice reference `&[u8]` is a **Fat Pointer**:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                 ANATOMY OF A FAT POINTER IN CPU REGISTERS                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   Rust Slice Reference: `&[u8]`                                             │
│   Consists of exactly TWO words in CPU registers:                           │
│                                                                             │
│   Register R0 (Pointer):    0x2000_0104 ──► Points to Address in SRAM       │
│   Register R1 (Length):     3           ──► Holds the count of valid bytes  │
│                                                                             │
│   Total Size on 32-bit ARM: 4 bytes (ptr) + 4 bytes (len) = 8 bytes total   │
│   Total Size on 64-bit CPU: 8 bytes (ptr) + 8 bytes (len) = 16 bytes total  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

When you pass a slice `&[u8]` to a function, the CPU simply loads the starting memory address into register `R0` and the length into register `R1`. 

Because the slice carries its length everywhere it goes, Rust can perform **instant compile-time or runtime bounds checking** without needing any dynamic heap allocation.

---

### 6. Preemptive Syntax Training: Slicing Ranges

To create a slice from an array, we borrow the array using Rust's range operator syntax (`..`):

```rust
let raw_data: [u8; 6] = [0x10, 0x20, 0x30, 0x40, 0x50, 0x60];

// 1. Exclusive Range (from index 1 up to, but not including, index 4):
let middle_slice: &[u8] = &raw_data[1..4]; // Contains: [0x20, 0x30, 0x40] (len = 3)

// 2. Open-Ended Start (from index 0 up to index 3):
let start_slice: &[u8] = &raw_data[..3];   // Contains: [0x10, 0x20, 0x30] (len = 3)

// 3. Open-Ended End (from index 3 to the very end):
let tail_slice: &[u8] = &raw_data[3..];    // Contains: [0x40, 0x50, 0x60] (len = 3)

// 4. Full Slice (borrows the entire array as a slice):
let all_slice: &[u8] = &raw_data[..];      // Contains all 6 bytes (len = 6)

// 5. Inclusive Range (from index 1 up to AND INCLUDING index 3):
let inclusive_slice: &[u8] = &raw_data[1..=3]; // Contains: [0x20, 0x30, 0x40] (len = 3)
```

---

### 7. Practical Robotics Case Study: Zero-Copy UART Telemetry Framing

Let's see how professional embedded software engineers use hexadecimal literals and byte slices to decode incoming robot telemetry packets without allocating a single byte on the heap.

#### The Problem Specification:
Our robot receives a packet over a serial port (UART) containing:
1. **Preamble / Magic Header:** 2 bytes (`0xAA`, `0x55`) indicating the start of a valid frame.
2. **Packet ID:** 1 byte (`0x01` for Motor Command, `0x02` for Sensor Request).
3. **Payload Length:** 1 byte ($N$).
4. **Payload Data:** $N$ bytes (e.g., Target Speeds).
5. **CRC Checksum:** 1 byte.

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                       ROBOT SERIAL TELEMETRY PACKET                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   Offset:      0x00     0x01     0x02     0x03     0x04     0x05     0x06   │
│   Field:     [ 0xAA ] [ 0x55 ] [ 0x01 ] [ 0x02 ] [ 0x64 ] [ 0xC8 ] [ 0x33 ]│
│              └───────┬───────┘    │        │     └───────┬───────┘    │     │
│                   Header          ID      Len         Payload        CRC    │
│                 (Preamble)               (2 bytes)  (Left/Right)            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Safe Rust Implementation:

```rust
#![no_std]

/// Parses an incoming telemetry frame using zero-copy byte slicing.
/// Returns Ok(payload_slice) if valid, or Err(&str) if corrupted.
pub fn parse_robot_telemetry(packet: &[u8]) -> Result<&[u8], &'static str> {
    // 1. Minimum packet size check: 2 (header) + 1 (id) + 1 (len) + 1 (crc) = 5 bytes
    if packet.len() < 5 {
        return Err("Packet too short");
    }

    // 2. Verify Magic Header (0xAA, 0x55) using sub-slice comparison
    if &packet[0..2] != [0xAA, 0x55] {
        return Err("Invalid magic preamble header");
    }

    let payload_len = packet[3] as usize;

    // 3. Mathematical bounds verification: Ensure entire claimed frame exists
    if packet.len() < 4 + payload_len + 1 {
        return Err("Packet truncated: payload incomplete");
    }

    // 4. Extract the payload slice with ZERO MEMORY ALLOCATION
    let payload_slice = &packet[4..4 + payload_len];

    // 5. Verify the simple XOR checksum
    let mut calculated_crc: u8 = 0;
    for &byte in &packet[0..4 + payload_len] {
        calculated_crc ^= byte;
    }

    let expected_crc = packet[4 + payload_len];
    if calculated_crc != expected_crc {
        return Err("CRC Checksum mismatch");
    }

    Ok(payload_slice)
}

// Example usage on bare-metal target:
pub fn process_incoming_buffer() {
    let rx_buffer: [u8; 7] = [
        0xAA, 0x55, // Magic Header
        0x01,       // Command ID: Set Motor Speeds
        0x02,       // Payload Length: 2 bytes
        0x64,       // Left Motor: +100 RPM
        0xC8,       // Right Motor: +200 RPM
        0x33,       // Checksum byte
    ];

    match parse_robot_telemetry(&rx_buffer) {
        Ok(motors) => {
            let left_speed  = motors[0];
            let right_speed = motors[1];
            // Send speeds directly to PWM hardware registers!
        }
        Err(_err) => {
            // Drop corrupted packet and reset UART receiver state machine
        }
    }
}
```

---

### 8. Line-by-Line Code & Silicon Execution Breakdown

| Code Expression / Line | Plain English Meaning | Hardware & Silicon Execution |
| :--- | :--- | :--- |
| `pub fn parse_robot_telemetry(packet: &[u8])` | Function takes a borrowed slice of bytes of arbitrary length. | Microcontroller CPU loads the buffer address into `R0` and buffer length into `R1`. No heap or stack allocation occurs. |
| `if packet.len() < 5` | Checks if total incoming bytes are at least 5. | CPU performs single cycle integer comparison: `cmp r1, #5`. Blazing fast ($1$ clock cycle at $168\,\text{MHz} = 5.95\,\text{ns}$). |
| `if &packet[0..2] != [0xAA, 0x55]` | Verifies the first two bytes match the hardware synchronization sequence. | Loads two bytes from SRAM via `ldrh` or two `ldrb` instructions and compares against constants in Flash ROM (`.rodata`). |
| `let payload_len = packet[3] as usize;` | Reads length byte and casts to architecture pointer width (`u32` on Cortex-M). | Zero-extends byte into 32-bit register `uxtb r2, r3`. |
| `let payload_slice = &packet[4..4 + payload_len];` | Creates a new sub-slice pointing directly to the motor speed bytes. | CPU calculates `ptr + 4` and length `payload_len`. **Zero data is copied.** A new fat pointer `(ptr, len)` is formed in registers. |
| `for &byte in &packet[0..4 + payload_len]` | Iterates over the packet bytes to compute verification checksum. | Tight loop using indexed load `ldrb` and exclusive-or `eors` instructions. |

---

### 9. Multi-Language Contrast Matrix: Raw Buffers & Slices

| Language | How Slices / Buffers Are Represented | Memory Footprint | Out-of-Bounds Behavior | Memory Safety Rating |
| :--- | :--- | :--- | :--- | :--- |
| **C (Bare Metal)** | Raw Pointer + separate integer: `uint8_t *buf, size_t len` | 8 bytes (32-bit) / 16 bytes (64-bit) | ❌ **Silent Undefined Behavior:** Corrupts adjacent memory, crashes CPU, or triggers security vulnerability. | ☠️ **Extremely Unsafe** |
| **C++ (`std::span`)** | Pointer + Size: `std::span<const uint8_t>` | 8 bytes (32-bit) / 16 bytes (64-bit) | ⚠️ **Unchecked by default:** `operator[]` does NOT check bounds unless `.at()` is used or special debug flags enabled. | ⚠️ **Partially Safe** |
| **Java** | `ByteBuffer` or `byte[]` | Massive object header (16+ bytes) + Heap allocation + Pointer indirection | Throws `IndexOutOfBoundsException` (Allocates exception object, pauses real-time execution). | 🛡️ **Safe, but High Overhead** |
| **Python** | `memoryview` or `bytes` object | Heavy PyObject header (56+ bytes) + Garbage Collector tracking | Raises `IndexError` (High latency, dynamic interpreter overhead). | 🛡️ **Safe, but Non-Deterministic** |
| **Rust (`no_std`)** | **Fat Pointer: `&[u8]` (`ptr` + `len`)** | **Exactly 2 CPU words (8 bytes on 32-bit, 16 bytes on 64-bit)** | ✅ **Guaranteed Safety:** Bounds checked at compile time or triggers deterministic panic handler. Zero allocation! | 🏆 **Aerospace-Grade Safe** |

---

### 10. Exhaustive Traps, Caveats & Compiler Diagnostics

#### Trap 1: The Out-of-Bounds Direct Indexing Landmine (`panic!`)

In safety-critical robotics (e.g., drone flight controllers or medical actuators), **a panic stops the motor loop and can cause catastrophic physical crashes**.

If you use direct array indexing with square brackets (`[]`) and the index is beyond the slice length:

```rust
let buffer: &[u8] = &[0x10, 0x20];
let byte = buffer[5]; // ❌ RUNTIME PANIC! Index 5 does not exist!
```

* **What happens:** At runtime, the CPU invokes the panic handler (`panic-halt` or `panic-probe-sys`), halting the microcontroller dead in its tracks!
* **The NASA JPL Rule #5 Fix:** In safety-critical embedded systems, **never use direct square bracket indexing on unchecked external data**. Use the safe `.get()` method instead, which returns an `Option<&u8>`:

```rust
let buffer: &[u8] = &[0x10, 0x20];

// ✅ 100% Panic-Free (NASA JPL Compliant):
match buffer.get(5) {
    Some(&byte) => {
        // Process valid byte
    }
    None => {
        // Gracefully handle missing data without panicking!
    }
}
```

---

#### Trap 2: Constant Literal Overflow (`error[E0080]`)

Because integer types are strictly sized in embedded Rust, attempting to assign a value larger than 255 to a `u8` causes an immediate compilation failure:

```rust
let bad_byte: u8 = 0x100; // 0x100 is 256 in decimal (requires 9 bits!)
```

```text
error[E0080]: evaluation of constant value failed
 --> src/main.rs:2:24
  |
2 |     let bad_byte: u8 = 0x100;
  |                        ^^^^^ attempt to compute `256_u8`, which would overflow
```

* **Why it happens:** A `u8` consists of exactly 8 bits ($2^8 = 256$, valid values are $0$ through $255$). The value `0x100` requires 9 bits (`0b1_0000_0000`).
* **The Fix:** Ensure the hex literal fits inside the target register type, or widen the variable to `u16` (`0x100_u16`).

---

#### Trap 3: Returning a Slice of a Local Stack Buffer (`error[E0515]`)

Beginners transitioning from C often attempt to create a buffer on the local stack and return a slice of it to a caller:

```rust
fn get_sensor_packet() -> &[u8] {
    let local_data = [0xAA, 0x55, 0x01]; // Created on local stack frame
    &local_data[..] // ❌ COMPILATION FAILS!
}
```

```text
error[E0515]: cannot return value referencing local variable `local_data`
 --> src/main.rs:3:5
  |
3 |     &local_data[..]
  |     ^^^^^^^^^^^^^^^ returns a value referencing data owned by the current function
```

* **Why it happens (The Silicon Ground Truth):** `local_data` lives on the CPU stack. When the function returns, the Stack Pointer (`SP` / `R13`) moves back up, destroying the stack frame. If Rust allowed this, the caller would receive a dangling pointer pointing to corrupted stack memory.
* **The Fix:** Return an owned fixed array `[u8; 3]`, write into a caller-provided mutable buffer `&mut [u8]`, or return a reference to static Flash memory (`&'static [u8]`).

---

#### Trap 4: Confusing Character Strings (`&str`) with Byte Slices (`&[u8]`)

In high-level languages like Python, binary bytes and text strings are frequently interchanged. In Rust, they are fundamentally distinct:

* A **`&str`** is guaranteed to be valid **UTF-8 encoded text**.
* A **`&[u8]`** is a raw stream of arbitrary binary bytes ($0$ to $255$).

If an IMU sends raw sensor packets, they contain arbitrary byte values (e.g., `0xFF`, `0x80`, `0x00`) which are **illegal UTF-8 sequences**:

```rust
// ❌ WRONG: Hardware telemetry is NOT valid UTF-8!
let raw_uart_bytes: &[u8] = &[0xFF, 0xFE, 0x01];
// let text: &str = core::str::from_utf8(raw_uart_bytes).unwrap(); // 💥 CRASH / PANIC!

// ✅ RIGHT: Keep raw hardware data as byte slices (&[u8])!
let command_header: &[u8] = b"CMD"; // &'static [u8; 3]
if &raw_uart_bytes[0..3] == command_header {
    // Process command
}
```

---

### 11. Section 3.1 Self-Check & Mental Model Review

Before advancing to Section 3.2 (*Bitwise Operations: AND, OR, XOR, NOT, Shifts*), verify that you can answer these three engineering questions:

1. **Why is `0x4002_0000` preferred over decimal `1,073,872,896` for hardware registers?**
   * *Answer:* Every hex character maps directly to 4 physical address bus wires (`0x4` = lines 31..28 are `0100`). Decimal hides the physical wire state.
2. **What are the two parts of a Fat Pointer `&[u8]` on a 32-bit microcontroller?**
   * *Answer:* A 32-bit starting memory address (pointer) in register `R0`, and a 32-bit length count in register `R1` (8 bytes total).
3. **Why does sub-slicing an array (`&buffer[2..6]`) generate zero CPU memory copy instructions?**
   * *Answer:* Because the slice only constructs a new `(ptr + 2, len = 4)` fat pointer. The underlying data remains untouched in its original SRAM or Flash location.


---

## 3.2 Bitwise Operations: AND (`&`), OR (`|`), XOR (`^`), NOT (`!`), Shifts (`<<`, `>>`)

In Section 3.1, we discovered that a microcontroller does not see numbers—it sees rows of tiny electrical transistor switches that are either **ON (3.3V)** or **OFF (0V)**.

On a normal computer, if you write `let x = 10 + 5;`, the CPU adds two numbers. But in robotics and embedded systems, our primary job is often **not math**—it is **state control**. A single 8-bit byte received from an ultrasonic distance sensor or an electric motor driver is almost never a single integer. Instead, each individual bit inside that byte represents a separate hardware sensor:

```text
Byte from Motor Controller: 0b1000_0101
                              │   │  └─► Bit 0: Motor Enabled (1 = YES)
                              │   └────► Bit 2: Direction (1 = Forward, 0 = Reverse)
                              └────────► Bit 7: OVERHEATING ALARM! (1 = FIRE HAZARD!)
```

To read that overheating alarm, change the direction without stopping the motor, or combine two 8-bit sensor bytes into one 16-bit measurement, we use **Bitwise Operations**.

In this section, we will break down the physical intuition of all six bitwise operators, deconstruct Rust's unique bitwise syntax, and observe what happens inside the silicon ALU (Arithmetic Logic Unit).

---

### 1. The Physical Intuition: Stories of the 6 Silicon Tools

To make bitwise logic second nature, think of each operator as a physical tool or a real-world everyday situation.

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                     THE 6 SILICON BITWISE OPERATORS                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   Operator   Name            Everyday Intuition                             │
│   ────────────────────────────────────────────────────────────────────────  │
│      &       Bitwise AND     The Two-Key Bank Vault (Both keys required)    │
│      |       Bitwise OR      The Emergency Exit Door (Either push bar opens)│
│      ^       Bitwise XOR     The Staircase Two-Way Light Switch             │
│      !       Bitwise NOT     Opposite Day (Every switch is flipped)         │
│      <<      Left Shift      The Factory Assembly Conveyor Belt (Pushed ◄)  │
│      >>      Right Shift     The Drop Chute Conveyor Belt (Pushed ►)        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 1. Bitwise AND (`&`): The Two-Key Bank Vault
Imagine a high-security bank vault with two keyholes. The vault door **only opens (1)** if **both Key A AND Key B are turned (1)**. If either key is missing (0), the door remains closed (0).

* In silicon: `&` is used as a **Filter (or Stencil)**. If you want to check whether Bit 7 is ON while ignoring all other 7 bits, you run it through an `&` filter!

#### 2. Bitwise OR (`|`): The Emergency Exit Door
Imagine a fire exit door with two push bars: one on the left, one on the right. If **either bar is pushed (1)**, the door swings open (1). The door only stays shut (0) if **neither bar is pushed (0)**.

* In silicon: `|` is used as an **Activator (or Combiner)**. If you want to turn ON a motor without messing up any existing speed settings, you use `|`.

#### 3. Bitwise XOR (`^`): The Two-Way Staircase Light Switch
Imagine the light on your stairs. There is a switch downstairs and a switch upstairs:
* If both switches are DOWN, the light is **OFF (0)**.
* If you flip downstairs UP, the light turns **ON (1)**.
* If you then flip upstairs UP as well, the light turns back **OFF (0)**!
* The light is only ON when the two switches are in **different positions**.

* In silicon: `^` (Exclusive OR) is used as a **Toggle Switch** or a **Difference Detector**. If two bits are identical, `^` produces `0`. If they are different, `^` produces `1`.

#### 4. Bitwise NOT (`!`): Opposite Day
Whatever you hand to the NOT operator, it produces the exact electrical opposite. Every `1` becomes a `0`, and every `0` becomes a `1`.

> [!WARNING] The C vs. Rust Syntax Trap!
> If you have programmed in C, C++, Java, or Python, you are used to writing `~x` for bitwise NOT. 
> **Rust does NOT use `~`!** In Rust, the bitwise NOT operator for integers is the exclamation point: **`!x`**. 
> If `x` is a boolean, `!x` is logical NOT. If `x` is an integer (`u8`, `u32`), `!x` flips every single bit!

#### 5. Left Shift (`<<`) & Right Shift (`>>`): The Conveyor Belt
Imagine 8 boxes lined up on a conveyor belt with a bottomless trash bin at both ends:
* **Left Shift (`<< N`):** The belt moves $N$ steps to the left. The boxes on the far left drop into the trash bin and vanish forever. Empty spaces opening up on the right are filled with fresh **`0`**s.
* **Right Shift (`>> N`):** The belt moves $N$ steps to the right. The boxes on the far right fall off and vanish. Empty spaces on the left are filled.

---

### 2. Preemptive Syntax Training: Isolating Every Operator in Rust

Before combining these into robot device drivers, let's deconstruct the exact syntax with isolated micro-examples.

#### 1. Bitwise AND (`&`)
```rust
let input:  u8 = 0b1100_1010;
let filter: u8 = 0b1111_0000;
let output: u8 = input & filter; // Result: 0b1100_0000
```
* **Notice:** Only positions where **both** `input` and `filter` had a `1` remained `1`. The lower 4 bits were completely wiped to `0`!

#### 2. Bitwise OR (`|`)
```rust
let current_state: u8 = 0b1000_0000;
let new_flag:      u8 = 0b0000_0101;
let combined:      u8 = current_state | new_flag; // Result: 0b1000_0101
```
* **Notice:** The `1` from `current_state` and the `1`s from `new_flag` were merged together into one byte without altering each other.

#### 3. Bitwise XOR (`^`)
```rust
let state:  u8 = 0b1010_1010;
let toggle: u8 = 0b0000_1111;
let result: u8 = state ^ toggle; // Result: 0b1010_0101
```
* **Notice:** The top 4 bits (where `toggle` had `0`) were left completely untouched. The bottom 4 bits (where `toggle` had `1`) were cleanly inverted!

#### 4. Bitwise NOT (`!`)
```rust
let original: u8 = 0b1111_0000;
let inverted: u8 = !original; // Result: 0b0000_1111
```
* **Notice:** Every single bit flipped state.

#### 5. Shift Operators (`<<` and `>>`)
```rust
let val: u8 = 0b0000_0001; // Decimal 1

let shifted_left = val << 3; // Shift 3 positions left: 0b0000_1000 (Decimal 8)
let shifted_back = shifted_left >> 2; // Shift 2 positions right: 0b0000_0010 (Decimal 2)
```

#### 6. Compound Assignment Operators (`&=`, `|=`, `^=`, `<<=`, `>>=`)
Just like `x += 1`, Rust provides shorthand bitwise mutation when a variable is declared `mut`:

```rust
let mut control_register: u8 = 0b0000_0000;

control_register |= 0b0000_0001; // Turn ON bit 0
control_register <<= 4;          // Shift the whole register left by 4
```

---

### 3. Silicon-Level Ground Truth: Inside the ALU

What actually happens physically inside an ARM Cortex-M or RISC-V microcontroller when you execute a bitwise operation?

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                 ALU (ARITHMETIC LOGIC UNIT) SILICON PIPELINE                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   Register R0: [ 1 1 0 0 1 0 1 0 ] ───┐                                     │
│                                       ├──► [ 8 Microscopic Parallel ] ────►  Register R2:
│   Register R1: [ 1 1 1 1 0 0 0 0 ] ───┘    [    AND Logic Gates     ]        [ 1 1 0 0 0 0 0 0 ]
│                                                                             │
│   Execution Time: Exactly 1 CPU Clock Cycle!                                │
│   At 168 MHz (STM32F4): 1 cycle = 1 / 168,000,000 = 5.95 nanoseconds!       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Parallel Silicon Gates
Unlike high-level loops in Python or Java that inspect elements one by one, your microcontroller contains **physical parallel transistor arrays**:
* An 8-bit CPU has 8 physical AND gates sitting side-by-side.
* A 32-bit ARM Cortex-M CPU has **32 physical AND gates** sitting side-by-side.
* When the processor executes an instruction like `ands r2, r0, r1`, the electrical voltages on all 32 lines pass through all 32 transistor gates simultaneously in **a single clock cycle**.

#### The Math Magic of Shifts: Ultrafast Multiplication & Division
In mathematics:
* Shifting left by 1 (`x << 1`) is physically identical to **multiplying by 2** ($x \times 2^1$).
* Shifting left by $N$ (`x << N`) is physically identical to **multiplying by $2^N$**.
* Shifting right by $N$ (`x >> N`) is physically identical to **dividing by $2^N$** (integer division, discarding the remainder).

```text
Binary:       0b0000_0011  =  3 in decimal
Left Shift 1: 0b0000_0110  =  6 in decimal  (3 x 2)
Left Shift 2: 0b0000_1100  = 12 in decimal  (3 x 4)
Left Shift 3: 0b0001_1000  = 24 in decimal  (3 x 8)
```

> [!TIP] Why Roboticists Love Shifts
> Microcontroller hardware division units (if present at all) can take anywhere from **2 to 12 clock cycles** to execute. A shift instruction (`lsl` or `lsr`) takes **exactly 1 cycle**. Whenever you need to multiply or divide sensor readings by powers of two ($2, 4, 8, 16, 256$), bit shifts deliver instantaneous execution!

---

### 4. Silicon Architecture: Logical vs. Arithmetic Right Shift

Here lies one of the most critical hardware traps in all of embedded systems engineering: **How does the CPU handle a Right Shift?**

When bits move to the right, what gets injected into the empty spots on the left?
The answer depends entirely on whether your integer is **Unsigned (`u8`, `u16`, `u32`)** or **Signed (`i8`, `i16`, `i32`)**!

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                 LOGICAL SHIFT VS. ARITHMETIC RIGHT SHIFT                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  CASE 1: LOGICAL RIGHT SHIFT (Unsigned Types: `u8`, `u16`, `u32`)            │
│  The CPU always injects ZEROES from the left:                               │
│                                                                             │
│  Initial `u8`:        [ 1 1 0 1 0 0 0 0 ]  (Decimal 208)                    │
│                             │ │ │ │ │ │                                     │
│  After `>> 2`:        [ 0 0 1 1 0 1 0 0 ]  (Decimal 52 = 208 / 4)           │
│                         ▲ ▲                                                 │
│                         └── Fresh 0s always injected!                       │
│                                                                             │
│  ─────────────────────────────────────────────────────────────────────────  │
│                                                                             │
│  CASE 2: ARITHMETIC RIGHT SHIFT (Signed Types: `i8`, `i16`, `i32`)          │
│  The Most Significant Bit (MSB) is the SIGN BIT (1 = Negative, 0 = Positive).│
│  To preserve negative numbers, the CPU REPLICATES THE SIGN BIT!             │
│                                                                             │
│  Initial `i8`:        [ 1 1 0 1 0 0 0 0 ]  (Decimal -48 in Two's Complement)│
│                         │   │ │ │ │ │ │                                     │
│  After `>> 2`:        [ 1 1 1 1 0 1 0 0 ]  (Decimal -12 = -48 / 4)          │
│                         ▲ ▲                                                 │
│                         └── Sign bit (1) was copied to keep number negative!│
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

* In ARM Assembly, **Logical Shift Right** uses the instruction: `lsr`.
* In ARM Assembly, **Arithmetic Shift Right** uses the instruction: `asr`.

> [!CAUTION] Hardware Driver Warning
> When working with hardware register masks, raw byte packets, or communication buses (I2C/SPI), **ALWAYS use unsigned integers (`u8`, `u16`, `u32`)**. If you accidentally use a signed integer (`i8`), a right shift can duplicate the leading `1` bit and corrupt your entire register mask!

---

### 5. Practical Robotics Case Study: Combining Two 8-bit Sensor Bytes into a 16-bit Measurement

Almost all robotics sensors (such as the MPU-6050 Accelerometer/Gyroscope, VL53L0X Time-of-Flight distance sensor, or magnetic wheel encoders) measure physical quantities in **16-bit resolution**. 

However, standard buses like I2C and SPI only transmit **8 bits (1 byte) at a time**. 

Therefore, the sensor splits the 16-bit measurement into two separate bytes:
1. **High Byte (`msb` - Most Significant Byte):** Contains bits 15 through 8.
2. **Low Byte (`lsb` - Least Significant Byte):** Contains bits 7 through 0.

How do we assemble these two separate 8-bit pieces back into a single 16-bit integer? **Using Left Shift (`<<`) and Bitwise OR (`|`)!**

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                 ASSEMBLING A 16-BIT SENSOR VALUE FROM TWO BYTES             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   Step 1: Read Two 8-bit Bytes from I2C Bus:                                │
│   High Byte (`msb`):            [ 0x3F ]  ──►  0b0011_1111                  │
│   Low Byte  (`lsb`):            [ 0x9A ]  ──►  0b1001_1010                  │
│                                                                             │
│   Step 2: Cast High Byte to 16 bits:                                        │
│   `msb as u16`                  ──►  0b0000_0000_0011_1111                  │
│                                                                             │
│   Step 3: Shift High Byte Left by 8 positions (`<< 8`):                     │
│   `(msb as u16) << 8`           ──►  0b0011_1111_0000_0000                  │
│                                                                             │
│   Step 4: Combine with Low Byte using Bitwise OR (`|`):                     │
│                                      0b0011_1111_0000_0000  (Shifted High)  │
│                                   |  0b0000_0000_1001_1010  (Low Byte)      │
│                                   ────────────────────────                  │
│   Final 16-Bit Reading:              0b0011_1111_1001_1010  (Hex: 0x3F9A)   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### The Rust Implementation:

```rust
#![no_std]

/// Combines an incoming high byte (MSB) and low byte (LSB) 
/// received over an I2C sensor bus into a single 16-bit unsigned distance reading.
#[inline(always)]
pub fn combine_sensor_bytes(msb: u8, lsb: u8) -> u16 {
    // 1. Widen msb to 16 bits so the shift does not overflow
    let high_part = (msb as u16) << 8;
    
    // 2. Widen lsb to 16 bits to match the target type
    let low_part = lsb as u16;
    
    // 3. Merge both halves using bitwise OR
    high_part | low_part
}

/// Decodes raw accelerometer readings into signed real-world forces.
/// Sensors report negative acceleration (e.g. falling) as Two's Complement.
pub fn parse_accelerometer_axis(msb: u8, lsb: u8) -> i16 {
    let raw_unsigned = combine_sensor_bytes(msb, lsb);
    
    // Cast to signed integer without any memory copying
    raw_unsigned as i16
}

// Example usage:
pub fn test_fusion() {
    let high: u8 = 0x1A;
    let low:  u8 = 0xF4;
    
    let combined_distance_mm = combine_sensor_bytes(high, low);
    // combined_distance_mm is now 0x1AF4 (Decimal: 6,900 mm or 6.9 meters)
}
```

---

### 6. Line-by-Line Code & Silicon Execution Breakdown

| Code Expression | Plain English Meaning | Hardware & Silicon Execution |
| :--- | :--- | :--- |
| `(msb as u16)` | Expands the 8-bit byte into a 16-bit register. | CPU executes zero-extension instruction: `uxtb r0, r0`. Takes 1 clock cycle. |
| `<< 8` | Moves the byte into the upper half of the 16-bit register. | CPU executes logical shift left: `lsl r0, r0, #8`. The lower 8 bits become `0`. Takes 1 clock cycle. |
| `high_part \| low_part` | Fills the lower 8 zeroes with the bits of `lsb`. | CPU executes bitwise OR: `orr r0, r0, r1`. Takes 1 clock cycle. |
| `raw_unsigned as i16` | Reinterprets the identical 16 electrical bits as a signed integer. | **Zero CPU instructions!** In silicon, the bit pattern does not change at all—only the compiler's interpretation changes. |

---

### 7. Multi-Language Contrast Matrix: Bitwise Operations

| Feature | Rust (`no_std`) | C (Bare Metal) | Java | Python |
| :--- | :--- | :--- | :--- | :--- |
| **Bitwise NOT Syntax** | **`!val`** | `~val` | `~val` | `~val` |
| **Shift Overflow ($\ge$ bit width)** | ✅ **Caught!** Panics in debug; masks in release. | ❌ **Undefined Behavior:** CPU may execute random behavior or crash. | ⚠️ **Silent Masking:** Java masks shift amount by 31 or 63 silently. | ⚠️ **Arbitrary Precision:** Python auto-expands into large integers on heap! |
| **Logical Right Shift on Signed** | Explicit: Cast to unsigned first (`x as u32 >> n`) | Requires manual cast: `((uint32_t)x) >> n` | Dedicated operator: `>>>` | ❌ Complex bitmasking required due to infinite sign extension |
| **Type Strictness** | 🛡️ **Absolute:** Compiler refuses to mix `u8` and `u16` without explicit `as` cast. | ⚠️ **Weak:** Silently promotes `uint8_t` to signed `int`, causing subtle sign bugs. | ⚠️ Promotes `byte` to `int` silently on any bitwise operation. | ⚠️ Dynamically typed; no fixed register widths without `ctypes`. |

---

### 8. Exhaustive Traps, Caveats & Compiler Diagnostics

#### Trap 1: The Invalid Unary Operator Error (`error[E0600]`)

Developers coming from C or Python instinctively type `~` when attempting to flip bits:

```rust
let mask: u8 = 0b0000_1111;
let inverted = ~mask; // ❌ COMPILATION FAILS!
```

```text
error[E0600]: cannot apply unary operator `~` to type `u8`
 --> src/main.rs:2:20
  |
2 |     let inverted = ~mask;
  |                    ^^^^^
  |
help: did you mean `!`?
  |
2 |     let inverted = !mask;
  |                    +
```
* **Why it happens:** In Rust's design, `~` was completely retired from the language syntax. The unary negation operator for all primitive integers is `!`.
* **The Fix:** Use `!mask`.

---

#### Trap 2: The Operator Precedence Landmine (A Classic Robotics Bug!)

In standard math, multiplication happens before addition. But how does the compiler prioritize bitwise operators versus addition and comparisons?

```rust
// Intent: Shift 1 left by 2 positions, then add 1 (Expected: (1 << 2) + 1 = 5)
let value = 1 << 2 + 1; // 😱 BUG! Evaluates to 8, NOT 5!
```

* **Why it happens:** In Rust (and in C), **arithmetic addition (`+`) has HIGHER precedence than bit shifts (`<<`)**!
  The compiler evaluates `2 + 1` first (which is 3), and then evaluates `1 << 3`, resulting in **8**!
* **The Ironclad Embedded Rule:** **ALWAYS USE EXPLICIT PARENTHESES `( )` WITH BITWISE OPERATIONS!**

```rust
let value = (1 << 2) + 1; // ✅ Correct: Evaluates to 5
```

---

#### Trap 3: Shift Count Overflow Panic

What happens if you shift an 8-bit integer by 8 or more positions?

```rust
let byte: u8 = 0b0000_0001;
let broken = byte << 8; // ❌ COMPILATION FAILS (or RUNTIME PANIC)!
```

If evaluated at compile time, the Rust compiler immediately halts:

```text
error: this arithmetic operation will overflow
 --> src/main.rs:2:18
  |
2 |     let broken = byte << 8;
  |                  ^^^^^^^^^ attempt to shift left by `8_i32`, which would overflow
```

* **The Silicon Ground Truth:** An 8-bit register only has positions `0` through `7`. Shifting by 8 completely clears every bit out of the byte. In C, shifting by $\ge \text{width}$ is **Undefined Behavior**. In Rust, the compiler protects you by rejecting it at compile time or panicking at runtime.
* **The Fix:** If you need to shift by 8 bits, widen the integer first: `(byte as u16) << 8`.

---

#### Trap 4: Boolean AND (`&&`) vs. Bitwise AND (`&`)

Beginners frequently confuse logical double-operators with single bitwise operators:

```rust
let status: u8 = 0b0000_0010;

// ❌ COMPILATION ERROR:
if status && 0b0000_0010 { }
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:8
  |
2 |     if status && 0b0000_0010 { }
  |        ^^^^^^ expected `bool`, found `u8`
```

* **Why it happens:** `&&` and `||` operate strictly on `bool` values (`true` / `false`) and perform short-circuiting. They cannot be used on integers.
* **The Fix:** Use the single bitwise operator `&` and explicitly compare the result against zero:

```rust
if (status & 0b0000_0010) != 0 {
    // Bit 1 is active!
}
```

---

### 9. Section 3.2 Self-Check & Mental Model Review

Before moving forward to Section 3.3 (*Register Bitmasking: Setting, Clearing, and Toggling Individual Bits*), make sure you can answer these three questions:

1. **Why does Rust use `!x` instead of `~x` for bitwise NOT on integers?**
   * *Answer:* Rust unified the logical negation and bitwise inversion operators into a single symbol (`!`), eliminating `~` from the language grammar.
2. **What is the difference in silicon between right-shifting a `u8` vs. an `i8`?**
   * *Answer:* Shifting a `u8` performs a Logical Shift Right (`lsr`), always inserting `0`s on the left. Shifting an `i8` performs an Arithmetic Shift Right (`asr`), replicating the sign bit (1 for negative, 0 for positive) to preserve the sign.
3. **What is the result of `(0x0F as u16) << 8 | 0xAA`?**
   * *Answer:* `0x0FAA` (in binary: `0b0000_1111_1010_1010`).
