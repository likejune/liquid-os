//! What the installer actually does on screen 7, as a list of commands of
//! ready-made tools. Executed by the privileged installer helper.

use crate::flow::Choices;

/// Steps shown in the progress list (screen 7).
pub const STEPS: [&str; 5] = [
    "Подготовка файлов",
    "Копирование системы",
    "Установка компонентов",
    "Настройка системы",
    "Завершение",
];

/// One command: program and arguments, plus the progress step it belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    /// Index into `STEPS`.
    pub step: usize,
    /// Program.
    pub program: &'static str,
    /// Arguments.
    pub args: Vec<String>,
}

fn cmd(step: usize, program: &'static str, args: &[&str]) -> Command {
    Command {
        step,
        program,
        args: args.iter().map(|s| s.to_string()).collect(),
    }
}

/// Build the install plan. `image_dir` is the repart definitions directory
/// (`/usr/lib/liquid/install.repart.d`). Returns None until a disk is chosen.
pub fn install_plan(c: &Choices, image_dir: &str) -> Option<Vec<Command>> {
    let disk = c.disk.as_ref()?;
    let dev = disk.device.as_str();
    let mut v = vec![
        cmd(0, "udevadm", &["settle"]),
        // Partitions the disk and block-copies the verified system from the
        // live medium into slot A (CopyBlocks=auto), creates empty slot B.
        cmd(
            1,
            "systemd-repart",
            &[
                "--dry-run=no",
                "--empty=force",
                "--definitions",
                image_dir,
                dev,
            ],
        ),
        cmd(1, "bootctl", &["install", "--esp-path=/mnt/efi"]),
    ];
    let mut flatpaks: Vec<&str> = Vec::new();
    if c.components.office {
        flatpaks.push("org.libreoffice.LibreOffice");
    }
    if c.components.extras {
        flatpaks.extend(["org.videolan.VLC", "org.gnome.Music"]);
    }
    if c.components.developer {
        flatpaks.extend(["org.gnome.Builder", "com.visualstudio.code"]);
    }
    for app in flatpaks {
        v.push(cmd(
            2,
            "flatpak",
            &["install", "--system", "--noninteractive", "flathub", app],
        ));
    }
    let locale = format!("LANG={}.UTF-8", c.language);
    v.push(Command {
        step: 3,
        program: "systemd-firstboot",
        args: vec![
            "--root=/mnt".into(),
            format!("--locale={}.UTF-8", c.language),
            format!("--keymap={}", c.layout),
        ],
    });
    v.push(Command {
        step: 3,
        program: "liquid-write-env",
        args: vec![locale],
    });
    v.push(cmd(4, "sync", &[]));
    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flow::Disk;

    #[test]
    fn plan_needs_disk() {
        assert!(install_plan(&Choices::default(), "/usr/lib/liquid/install.repart.d").is_none());
    }

    #[test]
    fn plan_partitions_first() {
        let c = Choices {
            disk: Some(Disk {
                device: "/dev/vda".into(),
                label: "VM".into(),
                size: 64 << 30,
                free: 64 << 30,
            }),
            ..Choices::default()
        };
        let p = install_plan(&c, "/usr/lib/liquid/install.repart.d").unwrap();
        assert_eq!(p[1].program, "systemd-repart");
        assert!(p
            .iter()
            .any(|x| x.args.iter().any(|a| a == "org.libreoffice.LibreOffice")));
        assert!(p.windows(2).all(|w| w[0].step <= w[1].step));
    }
}
