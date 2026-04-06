use sysinfo::{CpuRefreshKind, RefreshKind, System};

use super::InfoSource;
pub struct CpuInfo {
    name: String,
    cores: usize,
}

impl CpuInfo {
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        Self {
            name: Self::fetch_name(&sys),
            cores: Self::fetch_cores(&sys),
        }
    }

    fn fetch_cores(sys: &System) -> usize {
        sys.cpus().len()
    }

    fn fetch_name(sys: &System) -> String {
        sys.cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

impl InfoSource for CpuInfo {
    fn label(&self) -> &str {
        "CPU"
    }

    fn value(&self) -> String {
        format!("{} ({} cores)", self.name, self.cores)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fetch_cpu_name_not_unknown() {
        let sys = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        let name = CpuInfo::fetch_name(&sys);
        assert_ne!(name, "Unknown");
    }
    #[test]
    fn fetch_cores_greater_than_zero() {
        let sys = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        let cores = CpuInfo::fetch_cores(&sys);
        assert!(cores > 0);
    }
    #[test]
    fn cpu_label() {
        assert_eq!(CpuInfo::new().label(), "CPU");
    }
}
