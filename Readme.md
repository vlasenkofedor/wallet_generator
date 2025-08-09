# TRON Wallet Address Generator

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This Rust tool generates secure TRON addresses until it finds one matching your specified suffix. It supports both case-sensitive and case-insensitive searches, with parallel processing for maximum speed.

## Features

- 🚀 Multi-threaded search using Rayon for parallel processing
- 🔑 Secure key generation with `secp256k1` and OS-level RNG
- ⚙️ Case-sensitive or case-insensitive search modes
- 📦 Compiled binary optimized with `strip` and `upx`
- 🌐 Generates standard TRON addresses (starting with `T`)
- 🔒 Self-contained executable with no runtime dependencies

## Prerequisites

- Rust toolchain (latest stable version)
- `upx` (for binary compression - optional)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install UPX (Ubuntu/Debian)
sudo apt install upx -y

# For other systems see: https://upx.github.io
```
# Installation & Compilation
```bash
# 1. Clone repository
git clone https://github.com/vlasenkofedor/wallet_generator.git
cd wallet_generator-generator
```
```bash
# 2. Build with optimizations (automatically strips and compresses binary)
chmod +x build.sh
./build.sh
```
```bash
# 3. Run directly from build directory
./target/release/wallet_generator <suffix>
```
```bash
# Or install system-wide (optional)
sudo cp target/release/wallet_generator /usr/local/bin
```
## Alternative Manual Build
```bash
cargo build --release
strip target/release/wallet_generator
upx --best target/release/wallet_generator
```
# Usage
```bash
# Example:
./wallet_generator <suffix> [case-sensitive]
```
- suffix: The ending pattern you want to find (alphanumeric)
- case-sensitive (optional): Set to true for exact case matching

## Examples
```bash
# Case-insensitive search (matches ABC, Abc, abc, etc.):
./wallet_generator Abc true
```
```bash
# Search for special patterns (must be at end of address):
./wallet_generator 123
./wallet_generator Wallet
./wallet_generator FaNcY
```
## Sample Output
```text
Searching for TRON address ending with: 'Test'
Case sensitivity: disabled

Found matching address!
Address: TYvmc4kfZ4E3w2aFgKcQ9qj8dR7sTest
Private Key: 3d8f1b0e7ac0c4b2e8d9a6c5f3e1d8a7b0e4f2c1a9d3b6e8f7c4a5d2e1b0f3e
```

# Security
Private keys are generated using cryptographically secure RNG (OS entropy source)

Keys are never stored, transmitted, or logged

Generated keys should be immediately imported to a secure wallet

Always verify addresses on testnet before mainnet use

# License
License: MIT - [see](https://opensource.org/licenses/MIT) LICENSE file for details