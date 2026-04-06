use super::InfoSource;
use sysinfo::System;
pub struct KernelInfo {
    name: String,
}

impl KernelInfo {
    pub fn new() -> Self {
        let name = Self::fetch_kernel();
        Self { name }
    }

    fn fetch_kernel() -> String {
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    }
}

impl InfoSource for KernelInfo {
    fn label(&self) -> &str {
        "KERNEL"
    }
    fn value(&self) -> String {
        self.name.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fetch_kernel_name_not_unknown() {
        let name = KernelInfo::fetch_kernel();
        assert_ne!(name, "Unknown");
    }
}
