#!/usr/bin/env bash
set -euo pipefail

prefix="${PREFIX:-${HOME}/.local}"
bin_dir="${prefix}/bin"

if ! command -v cargo >/dev/null 2>&1; then
    printf '%s\n' 'Error: Cargo is required. Install Rust from https://rustup.rs/.' >&2
    exit 1
fi

printf '%s\n' 'Building snapfetch in release mode...'
cargo build --release

mkdir -p "$bin_dir"
install -m 755 target/release/snapfetch "$bin_dir/snapfetch"

printf 'Installed snapfetch to %s\n' "$bin_dir/snapfetch"
case ":${PATH}:" in
    *":${bin_dir}:"*) ;;
    *) printf 'Add %s to PATH if it is not already there.\n' "$bin_dir" ;;
esac
