use colored::Colorize;

use crate::system::SystemInfo;

const DEFAULT_FIELDS: &[&str] = &[
    "os", "kernel", "cpu", "cores", "cpu_usage", "memory", "disk", "uptime", "shell",
    "network", "desktop", "terminal", "processes", "swap",
];

pub fn render(info: &SystemInfo, use_color: bool, fields: Option<&str>) {
    colored::control::set_override(use_color);
    let terminal = terminal_width();
    let value_width = terminal.saturating_sub(40).max(20);
    let fields = fields.map(|value| value.split(',').map(|field| field.trim().to_ascii_lowercase()).collect::<Vec<_>>());
    let wanted = |name: &str| fields.as_ref().map(|items| items.iter().any(|item| item == name)).unwrap_or(true);
    let rows = info.fields().into_iter().filter(|(name, _)| wanted(name) && (fields.is_some() || DEFAULT_FIELDS.contains(name))).collect::<Vec<_>>();
    let username = whoami::username().unwrap_or_else(|_| "user".to_string());
    let mut right = vec![format!("{}@{}", username, info.hostname).bold().to_string(), "----------------".dimmed().to_string()];
    right.extend(rows.iter().map(|(label, value)| {
        let label = format!("{:<12}", format!("{}:", label.replace('_', " ")));
        format!("{}{}", label.cyan(), clip(value, value_width))
    }));

    let info_width = right.iter().map(|line| strip_ansi(line).chars().count()).max().unwrap_or(0);
    let info_offset = terminal.saturating_sub(info_width) / 2;
    for line in right { println!("{}{}", " ".repeat(info_offset), line); }
}

fn clip(value: &str, width: usize) -> String {
    let mut chars = value.chars();
    let clipped = chars.by_ref().take(width).collect::<String>();
    if chars.next().is_some() && width > 1 {
        format!("{}…", clipped.chars().take(width - 1).collect::<String>())
    } else {
        clipped
    }
}

pub fn render_minimal(info: &SystemInfo, use_color: bool) {
    colored::control::set_override(use_color);
    println!("{} | {} | {} | {}", info.os.cyan(), info.cpu, info.memory, info.uptime);
}

pub fn render_json(info: &SystemInfo, fields: Option<&str>) {
    let requested = fields.map(|value| value.split(',').map(|field| field.trim().to_ascii_lowercase()).collect::<Vec<_>>());
    let values = info.fields().into_iter()
        .filter(|(key, _)| requested.as_ref().map(|items| items.iter().any(|item| item == key)).unwrap_or(true))
        .map(|(key, value)| format!("\"{}\":\"{}\"", escape_json(key), escape_json(value)))
        .collect::<Vec<_>>();
    println!("{{{}}}", values.join(","));
}

fn escape_json(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}

fn strip_ansi(value: &str) -> String {
    let mut plain = String::with_capacity(value.len());
    let mut in_escape = false;
    for character in value.chars() {
        if character == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if character == 'm' { in_escape = false; }
        } else {
            plain.push(character);
        }
    }
    plain
}

fn terminal_width() -> usize {
    std::env::var("COLUMNS").ok().and_then(|value| value.parse().ok()).unwrap_or(80)
}
