# Hashcheck-rs
A small CLI tool to hash files and verify checksums. Built to practice Rust, `clap`, and cryptographic hashing.

## Install

```bash
git clone https://github.com/M4KSS1S/Hashcheck-rs
cd Hashcheck-rs/
cargo build --release
```
The binary will be at ./target/release/hashcheck.
# Usage
## Hash a file with SHA-256 (default)
```bash
hashcheck file.txt
```

## Hash with BLAKE3
```bash
hashcheck file.txt --algo blake3
```

## Verify a checksum
```bash
hashcheck file.txt --verify a3f5c2...
```

## Hash from stdin
```bash
echo "hello world" | hashcheck --stdin
```

---
