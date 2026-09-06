use sysinfo::{Disks, Networks, ProcessesToUpdate, System};

pub struct SystemInfo {
    pub os: String,
    pub kernel: String,
    pub cpu: String,
    pub memory: String,
    pub disk: String,
    pub uptime: String,
    pub shell: String,
    pub hostname: String,
    pub cpu_cores: String,
    pub cpu_usage: String,
    pub swap: String,
    pub processes: String,
    pub network: String,
    pub desktop: String,
    pub terminal: String,
}

impl SystemInfo {
    pub fn fields(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("os", &self.os), ("kernel", &self.kernel), ("hostname", &self.hostname), ("cpu", &self.cpu),
            ("cores", &self.cpu_cores), ("cpu_usage", &self.cpu_usage), ("memory", &self.memory),
            ("disk", &self.disk), ("uptime", &self.uptime), ("shell", &self.shell),
            ("network", &self.network),
            ("desktop", &self.desktop), ("terminal", &self.terminal),
            ("processes", &self.processes), ("swap", &self.swap),
        ]
    }
}

pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new();
    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_millis(100));
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let os = System::name().unwrap_or_else(|| "Unknown".to_string());

    let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

    let cpu = sys.cpus().first().map(|cpu| cpu.brand().to_string()).unwrap_or_else(|| "Unknown".to_string());
    let cpu_cores = sys.cpus().len().to_string();
    let cpu_usage = read_proc_cpu_usage().unwrap_or_else(|| {
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len().max(1) as f32
    });
    let memory = format_bytes_pair(sys.used_memory(), sys.total_memory());

    let disks = Disks::new_with_refreshed_list();
    let disk = disks
        .list()
        .iter()
        .find(|disk| disk.mount_point() == std::path::Path::new("/"))
        .or_else(|| disks.list().first())
        .map(|disk| format_bytes_pair(disk.total_space().saturating_sub(disk.available_space()), disk.total_space()))
        .unwrap_or_else(|| "Unknown".to_string());

    let uptime = format_uptime(System::uptime());

    let swap = format_bytes_pair(sys.used_swap(), sys.total_swap());
    let processes = sys.processes().len().to_string();
    let networks = Networks::new_with_refreshed_list();
    let network = networks
        .list()
        .keys()
        .filter(|name| name.as_str() != "lo")
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");

    let hostname = whoami::hostname().unwrap_or_else(|_| "machine".to_string());
    let shell = std::env::var("SHELL")
        .ok()
        .and_then(|path| std::path::PathBuf::from(path).file_name().map(|name| name.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "Unknown".to_string());
    let desktop = first_env(&["XDG_CURRENT_DESKTOP", "XDG_SESSION_DESKTOP", "DESKTOP_SESSION"]);
    let terminal = first_env(&["TERM_PROGRAM", "COLORTERM"]);

    SystemInfo {
        os,
        kernel,
        cpu,
        memory,
        disk,
        uptime,
        shell,
        hostname,
        cpu_cores,
        cpu_usage: format!("{:.0}%", cpu_usage),
        swap,
        processes,
        network: if network.is_empty() { "Unknown".to_string() } else { network },
        desktop,
        terminal,
    }
}

fn first_env(names: &[&str]) -> String {
    names.iter().find_map(|name| std::env::var(name).ok().filter(|value| !value.is_empty())).unwrap_or_else(|| "Unknown".to_string())
}

fn read_proc_cpu_usage() -> Option<f32> {
    fn sample() -> Option<(u64, u64)> {
        let line = std::fs::read_to_string("/proc/stat").ok()?.lines().next()?.to_string();
        let mut values = line.split_whitespace().skip(1).map(str::parse::<u64>);
        let user = values.next()?.ok()?;
        let nice = values.next()?.ok()?;
        let system = values.next()?.ok()?;
        let idle = values.next()?.ok()?;
        let iowait = values.next().and_then(Result::ok).unwrap_or(0);
        let total = user + nice + system + idle + iowait + values.sum::<Result<u64, _>>().ok()?;
        Some((total, idle + iowait))
    }

    let (total_before, idle_before) = sample()?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    let (total_after, idle_after) = sample()?;
    let total_delta = total_after.saturating_sub(total_before);
    let idle_delta = idle_after.saturating_sub(idle_before);
    if total_delta == 0 { None } else { Some((100.0 * (total_delta - idle_delta) as f32 / total_delta as f32).clamp(0.0, 100.0)) }
}

fn format_bytes_pair(used: u64, total: u64) -> String {
    format!("{} / {}", format_bytes(used), format_bytes(total))
}

fn format_bytes(bytes: u64) -> String {
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;

    if bytes >= GIB as u64 {
        format!("{:.1} GiB", bytes as f64 / GIB)
    } else {
        format!("{:.0} MiB", bytes as f64 / MIB)
    }
}

pub(crate) fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else {
        format!("{}h {}m", hours, minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::format_uptime;

    #[test]
    fn formats_short_uptime() {
        assert_eq!(format_uptime(4 * 3600 + 32 * 60), "4h 32m");
    }

    #[test]
    fn formats_multi_day_uptime() {
        assert_eq!(format_uptime(2 * 86_400 + 3 * 3600 + 5 * 60), "2d 3h 5m");
    }
}
