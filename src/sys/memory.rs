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
        let total_ram = Self::fetch_total_ram(&sys);
        let used_ram = Self::fetch_used_ram(&sys);
        Self {
            total_ram,
            used_ram,
        }
    }

    fn fetch_used_ram(sys: &System) -> f64 {
        sys.used_memory() as f64 / 1_000_000_000.0
    }

    fn fetch_total_ram(sys: &System) -> f64 {
        sys.total_memory() as f64 / 1_000_000_000.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_ram_not_zero() {
        let sys = System::new_all();
        assert!(MemoryInfo::fetch_total_ram(&sys) > 0.0);
    }
    #[test]
    fn used_ram_never_exceeds_total() {
        let sys = System::new_all();
        let total = MemoryInfo::fetch_total_ram(&sys);
        let used = MemoryInfo::fetch_used_ram(&sys);
        assert!(used <= total);
    }
}
