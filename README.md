![build](https://github.com/daite/magneta/workflows/Rust/badge.svg)
# Magneta

A fast, extensible CLI tool for searching torrents across multiple torrent sites with failover support and pluggable site parsers.

## Background

> Korean torrent sites are often filled with aggressive advertisements, popups, and scripts.  
> Searching for a single show can take many minutes and multiple clicks.  

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

### Example

```bash
make run search "전공의"
```

<details>
<summary>⠏ ✅ Search completed! (click to expand)</summary>

<pre>
+-----------------------------------------------------+--------------------------------------------------------------+
| Title                                               | Magnet                                                       |
+-----------------------------------------------------+--------------------------------------------------------------+
| 언젠가는 슬기로울 전공의생활.E08.250504.1080p-F1RST        | magnet:?xt=urn:btih:a2ecde5ebdd06a486541559ab024c5b734af10aa |
| 언젠가는 슬기로울 전공의생활.E07.250503.1080p-F1RST        | magnet:?xt=urn:btih:3046d9446f0abf746bdb1b19ebc368873b84a8e5 |
| 언젠가는 슬기로울 전공의생활.E06.250427.1080p-F1RST        | magnet:?xt=urn:btih:cff190e74667137cd4c0aa84c4f3df70b1fd7183 |
| 언젠가는 슬기로울 전공의생활.E05.250426.1080p-F1RST        | magnet:?xt=urn:btih:03ad63f4cd3dcdb7cc62733f0ba0bdd754202ec0 |
| 언젠가는 슬기로울 전공의생활.E04.250420.1080p-F1RST        | magnet:?xt=urn:btih:76eed1fede165614410ca66483e9ba3cec1b1653 |
| 언젠가는 슬기로울 전공의생활.E03.250419.1080p-F1RST        | magnet:?xt=urn:btih:c15eccdd1feb967c11863c164ea949cb639ad560 |
| 언젠가는 슬기로울 전공의생활.E02.250413.1080p-F1RST        | magnet:?xt=urn:btih:90792e2be9aee17ca522aae8770503a7d5260415 |
| 언젠가는 슬기로울 전공의생활.E01.250412.1080p-F1RST        | magnet:?xt=urn:btih:ee8ea649250f1abdda5ba3765757ca0f544bf058 |
+-----------------------------------------------------+--------------------------------------------------------------+
</pre>

</details>

```bash
# Build the project
make build

# Run the project
make run search "keyword"

# Example
make run search "전공의"

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
    ├── data
    │   ├── torrenttop
    │   └── torrentsome
    └── torrenttop_test.rs
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
