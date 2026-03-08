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

**RustLoader** is a shellcode loader written in Rust for security research. It implements multiple defense-evasion techniques to avoid detection by EDR/AV solutions, including debugger detection, ETW patching, human interaction simulation, and encrypted shellcode execution from heap memory.

## Evasion Techniques

| Technique | Implementation | File |
|-----------|---------------|------|
| **Anti-Debugging** | `IsDebuggerPresent` check at startup | `main.rs` |
| **Sandbox Evasion** | Requires 5 real mouse clicks before execution | `main.rs` |
| **ETW Blinding** | Patches `EtwEventWrite` via PEB walk + inline ASM | `patch.rs` |
| **XOR Encryption** | Shellcode encrypted at rest, decrypted in-memory | `shellcode.rs` |
| **Heap Execution** | Allocates executable heap, no `VirtualAlloc` calls | `shellcode.rs` |
| **Binary Stripping** | LTO, symbol stripping, size optimization | `Cargo.toml` |

## Architecture

```
src/
├── main.rs        — Entry point: anti-debug, click gate, orchestration
├── patch.rs       — ETW patch via PEB traversal (inline x64 ASM)
├── shellcode.rs   — Heap allocation, XOR decryption, shellcode execution
├── utils.rs       — XOR cipher, sleep utilities
└── cipher.rs      — Standalone encoder binary (random key generation)
```

### Execution Flow

```
main() → IsDebuggerPresent check
       → Wait for 5 mouse clicks (sandbox evasion)
       → patch_etw()     → PEB walk → resolve ntdll → patch EtwEventWrite
       → execute()       → HeapCreate(EXECUTABLE) → HeapAlloc
                         → copy encrypted shellcode → XOR decrypt in-place
                         → transmute to fn() → execute
```

## Prerequisites

- **Rust** stable toolchain (x86_64-pc-windows-msvc)
- **Windows 10/11 x64**

## Usage

### 1. Encode the Payload

```bash
cargo run --bin encoding -- shellcode.bin
```

```
[*] Random XOR key: 0x60
[!] Set XOR_KEY = 0x60 in src/shellcode.rs before building the loader
[+] Read 51200 bytes from 'shellcode.bin'
[+] Encrypted with key 0x60
[+] Written to 'encrypted.bin'
```

### 2. Update the XOR Key

Set the generated key in `src/shellcode.rs`:

```rust
const XOR_KEY: u8 = 0x60; // Match the encoder output
```

### 3. Build the Loader

```bash
cargo build --release --bin loader
```

The optimized binary will be at `target/release/loader.exe`.

### Build Profile

| Setting | Value | Purpose |
|---------|-------|---------|
| `opt-level` | `"z"` | Minimize binary size |
| `lto` | `true` | Link-time optimization |
| `codegen-units` | `1` | Maximum optimization |
| `panic` | `"abort"` | No unwinding overhead |
| `strip` | `true` | Remove all symbols |

## Technical Details

- **ETW Patch**: `xor rax, rax; ret` (`48 33 C0 C3`) written to `EtwEventWrite` prologue
- **PEB Walk**: Inline x64 ASM resolves `ntdll.dll` base via `gs:[0x60] → PEB → Ldr → InMemoryOrderModuleList`
- **Heap Execution**: `HeapCreate(HEAP_CREATE_ENABLE_EXECUTE)` + `HeapAlloc` avoids `VirtualAlloc` hooks
- **No Debug Output**: Release builds produce zero console output for stealth

## Legal Disclaimer

> **This software is intended exclusively for educational and security research purposes.** Unauthorized use against systems you do not own or have explicit permission to test is illegal. The author assumes no liability for misuse of this software.

## Author

**[@3xploit666](https://github.com/3xploit666)**

---

<div align="center">

*For educational and authorized security testing purposes only.*

</div>
