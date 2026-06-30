use quick_os_core::{ensure_kvm, ensure_path_exists, AppConfig, QuickOsError};

pub struct EnvironmentReport {
    pub kvm: bool,
    pub firecracker_binary: bool,
    pub kernel: bool,
    pub rootfs: bool,
}

impl EnvironmentReport {
    pub fn all_ok(&self) -> bool {
        self.kvm && self.firecracker_binary && self.kernel && self.rootfs
    }
}

pub fn check_environment(config: &AppConfig) -> EnvironmentReport {
    EnvironmentReport {
        kvm: ensure_kvm().is_ok(),
        firecracker_binary: config.firecracker.binary.exists(),
        kernel: config.guest.kernel_path.exists(),
        rootfs: config.guest.rootfs_path.exists(),
    }
}

pub fn require_environment(config: &AppConfig) -> Result<(), QuickOsError> {
    ensure_kvm()?;
    ensure_path_exists(&config.firecracker.binary, "firecracker binary")?;
    ensure_path_exists(&config.guest.kernel_path, "guest kernel")?;
    ensure_path_exists(&config.guest.rootfs_path, "guest rootfs")?;
    std::fs::create_dir_all(&config.firecracker.data_dir)?;
    std::fs::create_dir_all(config.agents_dir())?;
    Ok(())
}

pub fn print_report(report: &EnvironmentReport, config: &AppConfig) {
    let mark = |ok: bool| if ok { "ok" } else { "MISSING" };

    println!("quick-os environment check");
    println!("  /dev/kvm:              {}", mark(report.kvm));
    println!(
        "  firecracker binary:    {} ({})",
        mark(report.firecracker_binary),
        config.firecracker.binary.display()
    );
    println!(
        "  guest kernel:          {} ({})",
        mark(report.kernel),
        config.guest.kernel_path.display()
    );
    println!(
        "  guest rootfs:          {} ({})",
        mark(report.rootfs),
        config.guest.rootfs_path.display()
    );
    println!("  data dir:              {}", config.firecracker.data_dir.display());
}
