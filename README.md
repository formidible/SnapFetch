# Snapfetch

A fast, lightweight system information fetcher written in Rust.

Snapfetch displays useful information about your machine directly in your terminal, including your operating system, kernel version, CPU, memory usage, uptime, and more.

Inspired by tools like neofetch and fastfetch, but built from scratch with speed, simplicity, and customization in mind.

## Preview

```
        /\_/\
       ( o.o )
        > ^ <

dfja@linux

OS:        Arch Linux
Kernel:    6.16.4
CPU:       Ryzen 7 5700X
Memory:    5.4GB / 16GB
Uptime:    4h 32m
Shell:     zsh
```

## Features

- Fast startup time
- Written entirely in Rust
- Clean terminal output
- Operating system detection
- Kernel information
- CPU information
- Memory usage
- Disk information
- System uptime
- Custom ASCII logos
- Configurable output (planned)

## Installation

### Build from source

Requirements:

- Rust
- Cargo

Clone the repository:

```bash
git clone https://github.com/yourusername/snapfetch.git
cd snapfetch
```

Build:

```bash
cargo build --release
```

Run:

```bash
./target/release/snapfetch
```

## Usage

Run:

```bash
snapfetch
```

Example output:

```
        /\_/\
       ( o.o )
        > ^ <

user@machine

OS:        Arch Linux
Kernel:    6.16.4
CPU:       AMD Ryzen 7
Memory:    8GB / 16GB
Disk:      120GB / 512GB
Uptime:    2 days
```

## Development

Clone the repository:

```bash
git clone https://github.com/yourusername/snapfetch.git
cd snapfetch
```

Run locally:

```bash
cargo run
```

Format:

```bash
cargo fmt
```

Check:

```bash
cargo check
```


## License

MIT License
