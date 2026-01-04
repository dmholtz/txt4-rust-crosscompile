# TXT 4.0 — Rust Cross-Compile Template

This repository provides a ready-to-use development container and build setup for cross-compiling Rust applications for the **fischertechnik TXT 4.0 controller**.

It includes:

- a preconfigured Rust toolchain for ARM (armv7)
- cross-compilation target support
- example code demonstrating LED control
- a reproducible dev-container environment

## 🚀 Cross-compiling

Build an example which changes the color of the LED button for the TXT 4.0:

```bash
cargo build --release --example rgb --target armv7-unknown-linux-gnueabi
```

The resulting binary will be located at:

```bash
target/armv7-unknown-linux-gnueabi/release/examples/rgb
```

## ▶️ Running on the TXT 4.0

Copy the binary to the controller (adjust hostname / path as needed):

```bash
scp target/armv7-unknown-linux-gnueabi/release/examples/rgb <TXT4-HOSTNAME>:/home/ft/
```

Run it on the device:

```bash
ssh <TXT4-HOSTNAME> ./rgb
```