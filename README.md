# Snapfetch

Snapfetch is a fast, lightweight system information fetcher written in Rust. It displays useful details about your machine directly in the terminal with clean, readable output.

## Features

- Operating system and kernel detection
- CPU, memory, disk, uptime, shell, user, and hostname information
- CPU core count and usage, swap, process count, network interfaces, desktop, and terminal information
- Colored, minimal, JSON, filtered, and live-refresh output modes
- Small dependency footprint and quick startup
- Cross-platform system information through `sysinfo`

## Installation

You need Rust and Cargo installed.

```sh
# From a checkout of this repository:
./installer.sh
```

Alternatively, build and run it without installing:

```sh
cargo build --release
./target/release/snapfetch
```

## Usage

```text
snapfetch              # show system information
snapfetch --plain      # disable colors
snapfetch --minimal    # compact one-line output
snapfetch --json       # machine-readable output
snapfetch --fields os,cpu,memory,disk
snapfetch --watch 2    # refresh every two seconds
snapfetch --help       # show all options
```

Snapfetch also reads an optional lightweight configuration file from
`~/.config/snapfetch/config.toml`:

```toml
fields = "os,cpu,memory,disk,uptime"
plain = false
```

Command-line options override configuration values.

## Development

```sh
cargo run
cargo check
cargo test

# Install to ~/.local/bin (or set PREFIX=/some/path)
./installer.sh
```

## License

MIT License
