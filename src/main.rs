mod display;
mod system;

use clap::Parser;
use std::{thread, time::Duration};

#[derive(Debug, Parser)]
#[command(author, version, about = "A fast, lightweight system information fetcher")]
struct Args {
    /// Disable colored output.
    #[arg(long)]
    plain: bool,
    /// Comma-separated fields to display.
    #[arg(long, value_name = "FIELDS")]
    fields: Option<String>,
    /// Print compact one-line output.
    #[arg(long)]
    minimal: bool,
    /// Print machine-readable JSON.
    #[arg(long)]
    json: bool,
    /// Refresh output every N seconds.
    #[arg(long, value_name = "SECONDS")]
    watch: Option<u64>,
}

#[derive(Default)]
struct Config {
    fields: Option<String>,
    plain: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let config = load_config();
    let fields = args.fields.as_deref().or(config.fields.as_deref());
    let plain = args.plain || config.plain.unwrap_or(false);
    let interval = args.watch.map(|seconds| seconds.max(1));

    loop {
        if interval.is_some() { print!("\x1b[2J\x1b[H"); }
        let info = system::get_system_info();
        if args.json { display::render_json(&info, fields); }
        else if args.minimal { display::render_minimal(&info, !plain); }
        else { display::render(&info, !plain, fields); }
        if let Some(seconds) = interval { thread::sleep(Duration::from_secs(seconds)); } else { break; }
    }
}

fn load_config() -> Config {
    let Some(home) = std::env::var_os("HOME") else { return Config::default() };
    let path = std::path::PathBuf::from(home).join(".config/snapfetch/config.toml");
    let Ok(contents) = std::fs::read_to_string(path) else { return Config::default() };
    let mut config = Config::default();
    for line in contents.lines().map(str::trim).filter(|line| !line.is_empty() && !line.starts_with('#')) {
        let Some((key, raw)) = line.split_once('=') else { continue };
        let value = raw.trim().trim_matches('"').trim_matches('\'');
        match key.trim() {
            "fields" => config.fields = Some(value.to_string()),
            "plain" => config.plain = Some(value == "true"),
            _ => {}
        }
    }
    config
}
