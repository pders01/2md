# 2md

Convert HTML to Markdown from stdin.

## Usage

```bash
curl -s https://example.com | 2md
```

## Options

- `-q, --quiet` - Suppress error messages
- `-v` - Print version
- `-h, --help` - Print help

## Build

```bash
cd 2md
cargo build --release
```

The binary will be at `target/release/2md`.
