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
        let name = sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        let cores = sys.cpus().len();
        Self { name, cores }
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
