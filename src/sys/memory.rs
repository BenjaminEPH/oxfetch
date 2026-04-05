use sysinfo::System;

use super::InfoSource;
pub struct MemoryInfo {
    total_ram: f64,
    used_ram: f64,
}

impl InfoSource for MemoryInfo {
    fn label(&self) -> &str {
        "MEMORY"
    }

    fn value(&self) -> String {
        format!("{:.1} GB / {:.1} GB", self.used_ram, self.total_ram)
    }
}

impl MemoryInfo {
    pub fn new() -> Self {
        let sys = System::new_all();
        let total_ram = sys.total_memory() as f64 / 1_000_000_000.0;
        let used_ram = sys.used_memory() as f64 / 1_000_000_000.0;
        Self {
            total_ram,
            used_ram,
        }
    }
}
