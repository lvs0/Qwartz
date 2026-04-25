# ⬡ QWARTZ

> *Educational/Research Cryptographic Learning Tool*
>
> ⚠️ **NOT FOR PRODUCTION USE** — This is a learning/educational tool only.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange)](https://rust-lang.org)

---

## What is QWARTZ?

QWARTZ is an **educational and research cryptographic learning tool** that implements:

- **ZAB-R** (Adaptive Mode): AES-256-GCM encryption using environment-derived keys
- **ZAB-K** (Key-Based Mode): Key-based AES-256-GCM encryption with auto-mutability

### ⚠️ Important Disclaimer

> **This is NOT a production cryptographic system.**
> Do NOT use for any real security-sensitive applications.
> For educational purposes only.
> This codebase has NOT been audited by professional cryptographers.

---

## Quick Start

```bash
# Build
cargo build

# Run tests
cargo test

# Run CLI
cargo run -- --help
```

### CLI Usage

```bash
# Generate adaptive key (ZAB-R)
cargo run -- keygen --mode adaptive --output my.key

# Encrypt file
cargo run -- encrypt --key my.key --input secret.txt --output secret.qw

# Decrypt file
cargo run -- decrypt --key my.key --input secret.qw --output decrypted.txt
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        QWARTZ CORE                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    │
│  │   ZAB-R     │    │   ZAB-K     │    │   ZAB-Sync  │    │
│  │  (Adaptive) │    │ (Key-Based) │    │ (Protocol)  │    │
│  └─────────────┘    └─────────────┘    └─────────────┘    │
│         │                  │                  │             │
│         └──────────────────┼──────────────────┘             │
│                            ▼                                │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              ENTROPY ENGINE                          │   │
│  │  • CPU micro-timing                                  │   │
│  │  • Software fingerprints                             │   │
│  │  • Energy signature (local)                         │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## ZAB-R (Adaptive Mode)

ZAB-R derives encryption keys from environment entropy (CPU timing, OS fingerprint, etc.).

**Implementation:** AES-256-GCM (via `aes-gcm` crate, NOT XOR)

```rust
use qwartz::zab_r::{encrypt_adaptive, decrypt_adaptive};

let ciphertext = encrypt_adaptive(b"Secret message", b"purpose")?;
let plaintext = decrypt_adaptive(&ciphertext, b"purpose")?;
```

---

## ZAB-K (Key-Based Mode)

ZAB-K uses stored keys with AES-256-GCM encryption and auto-mutability.

---

## Security Considerations

- This is an **educational/research tool** — NOT for production
- XOR has been replaced with AES-256-GCM
- This codebase has NOT been professionally audited
- Do NOT trust this for real security-sensitive applications
- For production cryptography, use well-established libraries like
  `ring`, `rustls`, or `dalek-cryptography`

---

## Project R-Labs

QWARTZ is part of the **R-Labs Cryptographic Initiative**:

- [Polygone](https://github.com/lvs0/Polygone) - Ephemeral privacy network
- [Qwartz](https://github.com/lvs0/Qwartz) - Educational cryptography
- [Polygone-Server](https://github.com/lvs0/Polygone-Server) - Resource management

---

## License

MIT License - R-Labs © 2026

---

## Contact

- **R-Labs:** https://github.com/lvs0
- **Email:** lvs0@proton.me