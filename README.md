# esp_steno

This project uses Rust with nightly **only for this project** (no global override).
Follow these steps to set up your development environment and get started.

---

## 🚀 Quick Start

### 1. Install Rust (if you don’t have it)

Go to https://rustup.rs/ and download the installer.  
Run it and follow the instructions.

After install, verify in a new terminal or command prompt:
```
rustc --version
cargo --version
```

---

### 2. Clone this Repository

```
git clone <repo-url>
cd <repo-folder>
```

---

### 3. Use Nightly Rust Toolchain (Project Only)

This repository includes a `rust-toolchain.toml` file which pins the Rust toolchain to nightly for this project.

If you ever need to set or update manually (from inside this folder):
```
rustup override set nightly
```

---

### 4. Install ESP Toolchain Prerequisites

```
cargo install espup
espup install
```
> After running `espup install`, restart your terminal or command prompt.

---

### 5. Add the ESP Target

For ESP32:
```
rustup target add xtensa-esp32-none-elf
```

For ESP32-C3/S2/S3:
```
rustup target add riscv32imc-esp-espidf
```

---

### 6. Install Project Template Generator (if needed)

```
cargo install cargo-generate
```

---

### 7. Build the Project

```
cargo build
```
Or, if needed:
```
cargo build --release --target xtensa-esp32-none-elf
```

---

### 8. Flash the Device

First time only, install flasher:
```
cargo install espflash
```

Flash (replace COM3 with your serial port and <bin-name> with your binary):
```
espflash COM3 target\xtensa-esp32-none-elf\release\<bin-name>
```

---

### 9. Monitor Serial Output

```
espflash serial-monitor COM3
```

---

## Notes

- All Rust and cargo tools are per-user; no admin required.
- Using nightly is project-specific and does not affect other projects or the system default toolchain.
- For more, see: https://esp-rs.github.io/book/

