![build](https://github.com/daite/magneta/workflows/Rust/badge.svg)
# Magneta

A fast, extensible CLI tool for searching torrents across multiple torrent sites.

## Features

- 🔍 Search torrents by keyword
- 🌐 Supports multiple torrent sites (extensible architecture)
- 🧩 Pluggable site parsers for easy expansion
- 🛠️ Built with pure Rust, 100% Rust project
- 🧪 Full offline unit testing (HTML samples, GitHub Actions CI)
- 📦 Simple CLI usage with pretty table output


## Requirements

- Rust (stable)
- Cargo

Install Rust:  
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build and Run

```bash
# Build the project
make build

# Run the project
make run search "keyword"

# Example
make run search "슬기로울"

# Run all tests
make test
```

Or using Cargo directly:

```bash
cargo run -- search "keyword"
cargo test
```

## Project Structure

```text
magneta/
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── Makefile
├── README.md
├── src
│   ├── lib.rs
│   ├── main.rs
│   └── sites
│       ├── mod.rs
│       └── torrenttop.rs
└── tests
    ├── data
    │   └── torrenttop
    │       ├── magnet_sample.html
    │       └── search_sample.html
    └── torrenttop_test.rs
```
## Testing

This project uses **offline HTML samples** for testing:

```bash
make test
```

No external internet access required.  
Tests are fully reproducible even in CI environments.

## Contributing

Pull requests are welcome!  
Please follow Rust's code style and run:

```bash
make fmt
make lint
```
before submitting a PR.

## License

This project is licensed under the MIT License.
