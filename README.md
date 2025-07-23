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
sh
rustc --version
cargo --version
```

---

### 2. Clone this Repository

```
sh
git clone <repo-url>
cd <repo-folder>
```

---

### 3. Use Nightly Rust Toolchain (Project Only)

This repository includes a `rust-toolchain.toml` file which pins the Rust toolchain to nightly for this project.

If you ever need to set or update manually (from inside this folder):

```
sh
rustup override set nightly
```

---

### 4. Install ESP Toolchain Prerequisites

```
sh
cargo install espup
espup install
```
> After running `espup install`, restart your terminal or command prompt.

---

### 5. Build the Project

**For ESP32 device firmware:**

```
sh
cargo build --release --features device
```

**For host/desktop frontend (testing chord processing on your PC):**

```
sh
cargo run --features host
```

---

### 6. Flash the Device

First time only, install flasher:

```
sh
cargo install espflash
```

Flash (replace COM3 with your serial port and `<bin-name>` with your binary):

```
sh
espflash COM3 target/xtensa-esp32-none-elf/release/<bin-name>
```

---

### 7. Monitor Serial Output

```
sh
espflash serial-monitor COM3
```

---

## Notes

- Do **not** use `rustup target add xtensa-esp32-none-elf`—`espup` handles the toolchain and targets automatically.
- All Rust and cargo tools are per-user; no admin required.
- Using nightly is project-specific and does not affect other projects or the system default toolchain.
- For more, see: https://esp-rs.github.io/book/
- To add a GUI to the host frontend in the future, enable the `winit` dependency and update `host.rs`.
