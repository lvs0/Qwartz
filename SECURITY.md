# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | ✅ Development     |

## Reporting a Vulnerability

If you discover a security vulnerability in QWARTZ, please report it privately:

1. **Email:** lvs0@proton.me
2. **Subject:** [SECURITY] QWARTZ Vulnerability Report

Please include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

## Security Features

### Zero-Dependency Crypto
- Pure Rust implementation
- No external crypto library dependencies
- All algorithms implemented from scratch

### Memory Protection
- `zeroize` crate for sensitive data
- Automatic key wiping on drop
- No key material in logs or errors

### Post-Quantum Resistance
- Lattice-based algorithms
- Hash-based signatures
- Hybrid key exchange

## Threat Model

### Protected Against
- ✅ Classical computer attacks
- ✅ Quantum computer attacks (NIST Level 5)
- ✅ AI-powered attacks
- ✅ Side-channel attacks (timing)
- ✅ Key extraction attacks

### Not Protected Against
- ❌ Physical access to machine
- ❌ Compromised RNG hardware
- ❌ Social engineering

## Best Practices

1. Use ZAB-R for maximum security
2. Keep keys backed up securely
3. Use different keys for different purposes
4. Enable auto-mutation for long-term keys

## Audit

This code has not been audited by a third party. Use at your own risk.
