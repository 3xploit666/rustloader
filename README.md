<div align="center">

# RustLoader

**Advanced Shellcode Loader with Anti-Debugging & Evasion**

<img src="assets/rustloader.jpg" width="600">

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)](https://www.microsoft.com/windows)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

*A sophisticated shellcode loader built in Rust with multiple evasion techniques*

</div>

---

## Demo

![POC Demo](assets/poc.gif)

## Overview

**RustLoader** is a shellcode loader written in Rust, designed for security research and demonstrating advanced evasion and execution techniques. It focuses on direct memory manipulation using low-level APIs to avoid high-level functions that are easily monitored by EDR/AV solutions.

## Features

| Feature | Description |
|---------|-------------|
| **Anti-Debugging** | Detects debuggers and prevents execution in monitored environments |
| **Human Interaction Simulation** | Requires mouse clicks to simulate real user presence before execution |
| **Low-Level Memory Management** | Direct memory allocation and cleanup via low-level calls |
| **Encrypted Shellcode Execution** | Decrypts and executes shellcode directly from memory at runtime |
| **XOR Encryption** | Built-in encoder with random key generation |

## Project Structure

```
src/
├── main.rs        — Entry point, security checks and loader initialization
├── patch.rs       — Runtime process patching for persistence
├── shellcode.rs   — Shellcode loading and execution
├── utils.rs       — Utilities (click simulation, delays)
└── cipher.rs      — XOR payload encoder with random key generation
```

## Prerequisites

- **Rust** — Latest stable toolchain
- **Microsoft Visual C++ Build Tools** — Required for Windows compilation

## Usage

### 1. Encode the Payload

```bash
cargo run --bin encoding demon.x64.bin
```

```
Random XOR key generated: 0x60
Successfully read shellcode from '.\demon.x64.bin'
Successfully encrypted shellcode with key '0x60'
Successfully wrote encrypted shellcode to 'encrypted.bin'
```

### 2. Run the Loader

```bash
cargo run --bin loader
```

### 3. Build for Release

```bash
cargo build --release
```

## Legal Disclaimer

> **This software is intended exclusively for educational and security research purposes.** It is not suitable for production use or illegal activities. Unauthorized use against systems you do not own or have explicit permission to test is illegal. The author assumes no liability for misuse of this software.

## Author

**[@3xploit666](https://github.com/3xploit666)**

---

<div align="center">

*For educational and authorized security testing purposes only.*

</div>
