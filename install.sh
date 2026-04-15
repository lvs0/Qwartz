#!/bin/bash

# QWARTZ One-Line Installer
# R-Labs Next-Generation Cryptographic System

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${CYAN}"
echo "╔═══════════════════════════════════════════════════════╗"
echo "║   ⬡ QWARTZ - R-Labs Cryptographic System           ║"
echo "║   Beyond keys. Beyond quantum. Beyond limits.        ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo -e "${CYAN}Installing QWARTZ...${NC}"

# Build
cargo build --release 2>/dev/null || cargo build --release

# Install
BINARY="./target/release/qwartz"
if [ -f "$BINARY" ]; then
    sudo cp "$BINARY" /usr/local/bin/qwartz
    chmod +x /usr/local/bin/qwartz
    echo -e "${GREEN}✅ QWARTZ installed to /usr/local/bin/qwartz${NC}"
else
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}⬡ QWARTZ Ready!${NC}"
echo ""
echo "Usage:"
echo "  qwartz keygen --mode pq --output my.key"
echo "  qwartz encrypt --key my.key -i file.txt -o file.qw"
echo "  qwartz decrypt --key my.key -i file.qw -o file.txt"
echo ""
