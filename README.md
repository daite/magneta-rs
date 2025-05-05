![build](https://github.com/daite/magneta/workflows/Rust/badge.svg)
# Magneta

A fast, extensible CLI tool for searching torrents across multiple torrent sites with failover support and pluggable site parsers.

## Features

- 🔍 Search torrents by keyword
- 🌐 Supports multiple torrent sites with **failover fallback** (Multiplexer N→1)
- 🧩 Pluggable site parsers (e.g., TorrentTop, TorrentSome, TorrentRJ)
- 🛠️ Built with pure Rust, 100% Rust project
- 🧪 Full offline unit testing (HTML samples, GitHub Actions CI)
- 📦 Pretty table CLI output
- 🩺 `doctor` subcommand to check site health
- 🪵 Integrated logging (`RUST_LOG=warn` for errors)

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

# Run diagnostics
make run doctor

# Run all tests
make test
```

Or using Cargo directly:

```bash
cargo run -- search "keyword"
cargo run -- doctor
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
├── config.toml
├── src
│   ├── lib.rs
│   ├── main.rs
│   ├── site_registry.rs
│   └── sites
│       ├── mod.rs
│       ├── torrenttop.rs
│       ├── torrentsome.rs
│       └── torrentrj.rs
└── tests
```

## Site Diagnostics

Use the `doctor` subcommand to check which torrent sites are currently reachable:

```bash
cargo run -- doctor
```

**Example Output:**

```
🔍 Running diagnostics for torrent sites:
+-------------+----------------------------+----------------------+
| Site Name   | Site URL                   | Status               |
+-------------+----------------------------+----------------------+
| torrenttop  | https://torrenttop151.com  | ❌ connection failed |
| torrentrj   | https://torrentrj199.com   | ✅ 200 OK            |
| torrentsome | https://torrentsome192.com | ❌ connection failed |
+-------------+----------------------------+----------------------+
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
