//! "О системе" shows real data of the computer (not the mockup values):
//! device, CPU, memory, graphics, storage, OS version.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::fs;

/// Values shown in the About window rows.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct About {
    /// "Версия 1.0 • Предварительный выпуск" line.
    pub version: String,
    /// "Устройство".
    pub device: String,
    /// "Процессор".
    pub cpu: String,
    /// "Память".
    pub memory: String,
    /// "Графика".
    pub graphics: String,
    /// "Накопитель".
    pub storage: String,
}

/// Value of a key in os-release text.
pub fn os_release_value(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|l| {
        let (k, v) = l.split_once('=')?;
        (k.trim() == key).then(|| v.trim().trim_matches('"').to_string())
    })
}

/// "model name" from /proc/cpuinfo.
pub fn cpu_model(cpuinfo: &str) -> Option<String> {
    cpuinfo.lines().find_map(|l| {
        let (k, v) = l.split_once(':')?;
        (k.trim() == "model name").then(|| v.split_whitespace().collect::<Vec<_>>().join(" "))
    })
}

/// MemTotal from /proc/meminfo, rounded to whole GB as on stickers ("16 ГБ").
pub fn memory_gb(meminfo: &str) -> Option<u64> {
    let kb: u64 = meminfo
        .lines()
        .find(|l| l.starts_with("MemTotal:"))?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()?;
    let gib = kb as f64 / (1024.0 * 1024.0);
    // Usable RAM is a bit below installed RAM; round up to the next power of two step.
    let steps = [1u64, 2, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256];
    Some(
        steps
            .iter()
            .copied()
            .find(|s| (*s as f64) >= gib)
            .unwrap_or(gib.ceil() as u64),
    )
}

/// GPU vendor name from a PCI vendor id ("0x10de").
pub fn gpu_vendor(vendor_id: &str) -> &'static str {
    match vendor_id.trim().to_lowercase().as_str() {
        "0x10de" => "NVIDIA",
        "0x1002" => "AMD",
        "0x8086" => "Intel",
        "0x1af4" | "0x1234" | "0x15ad" | "0x80ee" => "Виртуальная видеокарта",
        _ => "Видеокарта",
    }
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

/// Collect About data from the running system.
pub fn collect() -> About {
    let os = read("/usr/lib/os-release");
    let device = [
        "/sys/class/dmi/id/product_name",
        "/sys/class/dmi/id/board_name",
    ]
    .iter()
    .map(|p| read(p).trim().to_string())
    .find(|s| !s.is_empty())
    .unwrap_or_else(|| "Компьютер".into());
    let mut gpus: Vec<&str> = Vec::new();
    if let Ok(dir) = fs::read_dir("/sys/class/drm") {
        for e in dir.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if name.starts_with("card") && !name.contains('-') {
                let v = gpu_vendor(&read(&format!("/sys/class/drm/{name}/device/vendor")));
                if !gpus.contains(&v) {
                    gpus.push(v);
                }
            }
        }
    }
    let mut storage = Vec::new();
    if let Ok(dir) = fs::read_dir("/sys/block") {
        for e in dir.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if ["loop", "ram", "zram", "dm-", "sr"]
                .iter()
                .any(|p| n.starts_with(p))
                || read(&format!("/sys/block/{n}/removable")).trim() == "1"
            {
                continue;
            }
            let sectors: u64 = read(&format!("/sys/block/{n}/size"))
                .trim()
                .parse()
                .unwrap_or(0);
            if sectors == 0 {
                continue;
            }
            let ssd = read(&format!("/sys/block/{n}/queue/rotational")).trim() == "0";
            storage.push(format!(
                "{}{}",
                liquid_format::size(sectors * 512),
                if ssd { " SSD" } else { " HDD" }
            ));
        }
    }
    About {
        version: os_release_value(&os, "VERSION").unwrap_or_else(|| "—".into()),
        device,
        cpu: cpu_model(&read("/proc/cpuinfo")).unwrap_or_else(|| "Процессор".into()),
        memory: memory_gb(&read("/proc/meminfo"))
            .map(|g| format!("{g} ГБ"))
            .unwrap_or_default(),
        graphics: if gpus.is_empty() {
            "—".into()
        } else {
            gpus.join(", ")
        },
        storage: storage.join(", "),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_os_release() {
        let t = "NAME=\"Liquid OS\"\nVERSION=\"0.4.0 (pre-alpha)\"\n";
        assert_eq!(
            os_release_value(t, "VERSION").as_deref(),
            Some("0.4.0 (pre-alpha)")
        );
    }

    #[test]
    fn parses_cpu_and_memory() {
        let cpu = "processor\t: 0\nmodel name\t: AMD Ryzen 7  7840U w/ Radeon 780M Graphics\n";
        assert_eq!(
            cpu_model(cpu).as_deref(),
            Some("AMD Ryzen 7 7840U w/ Radeon 780M Graphics")
        );
        assert_eq!(memory_gb("MemTotal:       15728640 kB\n"), Some(16));
    }

    #[test]
    fn vendors() {
        assert_eq!(gpu_vendor("0x10de\n"), "NVIDIA");
    }
}
