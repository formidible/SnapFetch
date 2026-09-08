#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: ./installer.sh [options]

Build and install snapfetch. By default it uses /usr/local/bin when that
directory is writable (or can be reached with sudo), and ~/.local/bin otherwise.

Options:
  --system             Install to /usr/local/bin (use sudo when needed)
  --user               Install to the user's XDG bin directory
  --prefix DIR         Install below DIR/bin
  --bin-dir DIR        Install the executable directly into DIR
  --uninstall          Remove the installed snapfetch executable
  -h, --help           Show this help
EOF
}

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
prefix=""
bin_dir=""
mode="auto"
uninstall=false

while (($#)); do
    case "$1" in
        --system) mode="system"; shift ;;
        --user) mode="user"; shift ;;
        --prefix)
            [[ $# -ge 2 ]] || { printf '%s\n' 'Error: --prefix needs a directory.' >&2; exit 2; }
            prefix="$2"; shift 2 ;;
        --bin-dir)
            [[ $# -ge 2 ]] || { printf '%s\n' 'Error: --bin-dir needs a directory.' >&2; exit 2; }
            bin_dir="$2"; shift 2 ;;
        --uninstall) uninstall=true; shift ;;
        -h|--help) usage; exit 0 ;;
        *) printf 'Error: unknown option: %s\n\n' "$1" >&2; usage >&2; exit 2 ;;
    esac
done

user_bin_dir="${XDG_BIN_HOME:-${HOME:-}/.local/bin}"
if [[ -z "$user_bin_dir" || "$user_bin_dir" == "/.local/bin" ]]; then
    user_bin_dir="${TMPDIR:-/tmp}/snapfetch-bin"
fi

if [[ -z "$bin_dir" ]]; then
    case "$mode" in
        system) bin_dir="/usr/local/bin" ;;
        user) bin_dir="$user_bin_dir" ;;
        auto)
            if [[ "${EUID:-$(id -u)}" -eq 0 ]]; then
                bin_dir="/usr/local/bin"
            elif [[ -d /usr/local/bin && -w /usr/local/bin ]]; then
                bin_dir="/usr/local/bin"
            else
                bin_dir="$user_bin_dir"
            fi
            ;;
    esac
fi

if [[ -n "$prefix" ]]; then
    bin_dir="${prefix%/}/bin"
fi

# Resolve custom relative paths before any sudo command runs. This keeps the
# destination stable even when privilege escalation changes the working dir.
if [[ "$bin_dir" != /* ]]; then
    bin_dir="$(pwd -P)/${bin_dir#./}"
fi

if [[ "$uninstall" == true ]]; then
    target="${bin_dir%/}/snapfetch"
    if [[ -e "$target" ]]; then
        if [[ -w "$target" || -w "$(dirname -- "$target")" ]]; then
            rm -- "$target"
        elif command -v sudo >/dev/null 2>&1; then
            sudo rm -- "$target"
        else
            printf 'Error: cannot remove %s (try sudo or choose --user).\n' "$target" >&2
            exit 1
        fi
        printf 'Removed %s\n' "$target"
    else
        printf 'snapfetch is not installed at %s\n' "$target"
    fi
    exit 0
fi

load_cargo() {
    if ! command -v cargo >/dev/null 2>&1 && [[ -f "${HOME:-}/.cargo/env" ]]; then
        # shellcheck disable=SC1090
        . "${HOME}/.cargo/env"
    fi
}

install_rust() {
    if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
        printf '%s\n' 'Error: Cargo is missing and neither curl nor wget is available to install Rust.' >&2
        printf '%s\n' 'Install Rust from https://rustup.rs/ and run this installer again.' >&2
        exit 1
    fi
    printf '%s\n' 'Cargo was not found; installing Rust with rustup...'
    if command -v curl >/dev/null 2>&1; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    else
        wget -qO- https://sh.rustup.rs | sh -s -- -y
    fi
    load_cargo
}

load_cargo
command -v cargo >/dev/null 2>&1 || install_rust
command -v cargo >/dev/null 2>&1 || { printf '%s\n' 'Error: Rust installed, but Cargo is still unavailable in PATH.' >&2; exit 1; }

printf '%s\n' 'Fetching locked dependencies...'
if ! cargo fetch --locked --manifest-path "${script_dir}/Cargo.toml"; then
    printf '%s\n' 'Warning: not all optional platform dependencies could be fetched; continuing with the current target.' >&2
fi
printf '%s\n' 'Building snapfetch in release mode...'
cargo build --release --locked --manifest-path "${script_dir}/Cargo.toml"

target="${bin_dir%/}/snapfetch"
if [[ -d "$bin_dir" && -w "$bin_dir" ]]; then
    install -m 755 "${script_dir}/target/release/snapfetch" "$target"
elif mkdir -p "$bin_dir" 2>/dev/null && [[ -w "$bin_dir" ]]; then
    install -m 755 "${script_dir}/target/release/snapfetch" "$target"
elif command -v sudo >/dev/null 2>&1; then
    sudo mkdir -p "$bin_dir"
    sudo install -m 755 "${script_dir}/target/release/snapfetch" "$target"
else
    printf 'Error: %s is not writable and sudo is unavailable.\n' "$bin_dir" >&2
    exit 1
fi

printf 'Installed snapfetch to %s\n' "$target"
case ":${PATH:-}:" in
    *":${bin_dir}:"*) ;;
    *)
        printf 'Note: %s is not currently on PATH. Add this to your shell profile:\n' "$bin_dir"
        printf '  export PATH="%s:\$PATH"\n' "$bin_dir"
        ;;
esac
