# 🗃️ my_archive

A fast, lightweight CLI tool written in Rust to pack multiple files into a single `.arch` archive with optional XOR encryption.

---

## ✨ Features

- 📦 Pack multiple files into a single `.arch` archive
- 🔐 Optional password-based XOR encryption/decryption
- ⚡ Stream-based processing — memory efficient for large files
- 🛡️ Input validation and descriptive error messages
- 🖥️ Simple CLI interface powered by [clap](https://github.com/clap-rs/clap)

---

## 🚀 Installation

```bash
git clone https://github.com/koktel141/my_archive.git
cd my_archive
cargo build --release
```

The binary will be at `target/release/archive`.

---

## 📖 Usage

### Pack files into an archive

```bash
# Without encryption
./archive pack file1.txt file2.txt image.png

# With password encryption
./archive pack --password mysecret file1.txt file2.txt image.png
```

### Unpack an archive

```bash
# Without encryption
./archive unpack output.arch

# With password decryption
./archive unpack --password mysecret output.arch
```

---

## 🏗️ How It Works

The `.arch` format uses a simple binary layout:

```
┌─────────────────────────────────────┐
│         Header                      │
│  [num_files : u32]                  │
│  ┌─────────────────────────────┐    │
│  │ [name_len : u32]            │ ×N │
│  │ [filename : bytes]          │    │
│  │ [file_size : u64]           │    │
│  └─────────────────────────────┘    │
├─────────────────────────────────────┤
│         Data                        │
│  [file_data : bytes] × N            │
└─────────────────────────────────────┘
```
{
##Tip for quick run:
    cargo run -- pack file1.txt file2.txt --password "secret123"  [password is optional]
    cargo run -- unpack output.arch --password "secret123" [if you didn't set any password just skip the --passwoord]
}

The entire stream is optionally XOR-encrypted using a repeating key derived from the password — applied transparently via custom `XorWriter` and `XorReader` wrappers that implement Rust's standard `Write` and `Read` traits.

---

## 📁 Project Structure

```
src/
├── main.rs      # CLI entry point — argument parsing with clap
├── crypto.rs    # XorWriter / XorReader streaming wrappers
├── pack.rs      # Archive creation logic
└── unpack.rs    # Archive extraction logic
```

---

## ⚠️ Security Note

XOR cipher is **not** cryptographically secure and is intended for **educational purposes only**. Do not use this tool to protect sensitive data.

---

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/) 🦀
- [clap 4.4](https://github.com/clap-rs/clap) — CLI argument parsing
