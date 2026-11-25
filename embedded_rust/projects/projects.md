# **Projects**

## ESP32-C3 SuperMini

### Step-1: Install the Silicon Inspection Tool (`esptool`)

```bash
sudo apt update && sudo pip install esptool
```

### Query the chip Identity

```bash
esptool --port /dev/ttyACM0 read-mac
```

If error come:
```bash
sudo usermod -aG dialout $USER
```

### Query the on-board flash memory

```bash
esptool --port /dev/ttyACM0 flash-id
```

## Set up the machine 

### 1. Install System Native Build Dependencies

```bash
sudo apt install -y build-essential pkg-config libudev-dev
```

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3. Add the RISC-V Silicon Target 

```bash
rustup target add riscv32imc-unknown-none-elf
```

### 4. Install `espflash` (The flasher & Serial Monitor)

```bash
cargo install espflash --locked
```



Step 2: The Color Number Code
  
   Color                                       │                Number Value
  ─────────────────────────────────────────────┼─────────────────────────────────────────────
   Black                                       │                      0
   Brown                                       │                      1
   Red                                         │                      2
   Orange                                      │                      3
   Yellow                                      │                      4
   Green                                       │                      5
   Blue                                        │                      6
   Violet                                      │                      7
   Grey                                        │                      8
   White                                       │                      9



