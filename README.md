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

```sh
# From a checkout of this repository:
./installer.sh
```

The installer is safe to run from any working directory. It fetches the locked
Rust dependencies, builds a release binary, and installs it system-wide in
`/usr/local/bin` when possible. Without permission for that directory it uses
`${XDG_BIN_HOME:-~/.local/bin}` instead. If Cargo is missing, it automatically
installs Rust with Rustup when `curl` or `wget` is available.

Useful installation options:

```sh
./installer.sh --system             # force /usr/local/bin
./installer.sh --user               # install for the current user
./installer.sh --prefix /opt/tools  # install to /opt/tools/bin
./installer.sh --uninstall          # remove from the selected bin directory
```

If the chosen user directory is not already on `PATH`, the installer prints
the exact export command to add to your shell profile.

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

# Install with the same behavior as above
./installer.sh
```

## License

MIT License
