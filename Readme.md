# TRON Wallet Generator (CPU Optimized) ⚡

High-performance TRON vanity address generator optimized for modern multi-core CPUs.
This tool generates TRON addresses and searches for a specific **suffix** (ending characters).

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🚀 Performance

- **Speed**: Uses `secp256k1` C-library bindings for maximum elliptic curve performance.
- **Parallelism**: Automatically detects and utilizes all available CPU cores.
- **Efficiency**: Minimized memory allocations and atomic contention.
- **Reporting**: Provides status updates every 1 minute.

## 📋 Prerequisites

- **Rust**: Latest stable version ([Install Rust](https://rustup.rs/))
- **OS**: macOS, Linux, or Windows (Optimized for macOS Apple Silicon)

## 🔧 Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/vlasenkofedor/wallet_generator.git
   cd wallet_generator
   ```

2. **Build for Maximum Performance:**
   Use the `target-cpu=native` flag to enable CPU-specific optimizations (AVX2, NEON, etc.).

   ```bash
   RUSTFLAGS="-C target-cpu=native" cargo build --release
   ```

## 📖 Usage

The tool accepts two arguments:

1. `SUFFIX`: The string you want your address to end with.
2. `CASE_SENSITIVE` (Optional): `true` or `false` (default: `false`).

### Basic Command

```bash
./target/release/wallet_generator <SUFFIX> [CASE_SENSITIVE]
```

### Examples

#### 1. Simple Suffix Search (Case-Insensitive)

Find an address ending with "888". This will match `...888`, `...888`, etc.

```bash
./target/release/wallet_generator 888
```

#### 2. Case-Sensitive Suffix

Find an address ending exactly with "Love". Matches `...Love` but NOT `...love` or `...LOVE`.

```bash
./target/release/wallet_generator Love true
```

#### 3. Long Suffix (Harder to find)

Find an address ending with "Crypto".

```bash
./target/release/wallet_generator Crypto
```

## 💡 How it Works

1. **Key Generation**: Generates a random private key.
2. **Public Key Derivation**: Derives the public key using Secp256k1.
3. **Hashing**: Applies Keccak-256 to the public key.
4. **Address Formation**: Adds the `0x41` prefix and calculates the double SHA-256 checksum.
5. **Base58 Encoding**: Encodes the result to get the final TRON address (e.g., `T...`).
6. **Suffix Check**: Checks if the generated address ends with your desired suffix.

## ⚠️ Important Notes

- **Prefix vs Suffix**: This tool currently searches for a **SUFFIX** (the end of the address). All TRON addresses start with `T`.
- **Performance**: The search time increases exponentially with the length of the suffix.
  - 3 chars: Instant
  - 4 chars: Seconds
  - 5 chars: Minutes
  - 6 chars: Hours/Days
- **Security**: Private keys are generated securely using OS entropy and are **never** stored or transmitted. They are only displayed when a match is found.

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## 📄 License

MIT License
