# Contributing to QWARTZ

Thank you for your interest in contributing to QWARTZ!

## Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/lvs0/Qwartz
cd Qwartz

# Build
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## Code Standards

- Use `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Write tests for new features
- Update documentation when needed

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Security Considerations

- Never commit key material or secrets
- Use `zeroize` for sensitive data
- Report security issues via email, not GitHub

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
