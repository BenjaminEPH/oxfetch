use super::InfoSource;
use sysinfo::System;
pub struct KernelInfo {
    name: String,
}

impl KernelInfo {
    pub fn new() -> Self {
        let name = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        Self { name }
    }
}
impl InfoSource for KernelInfo {
    fn label(&self) -> &str {
        "Kernel"
    }
    fn value(&self) -> String {
        self.name.clone()
    }
}
