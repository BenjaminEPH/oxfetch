use sysinfo::System;

use super::InfoSource;
pub struct UptimeInfo {
    seconds: u64,
}

impl InfoSource for UptimeInfo {
    fn label(&self) -> &str {
        "UPTIME"
    }

    fn value(&self) -> String {
        let h = self.seconds / 3600;
        let m = (self.seconds % 3600) / 60;
        format!("{}h {}m", h, m)
    }
}

impl UptimeInfo {
    pub fn new() -> Self {
        Self {
            seconds: System::uptime(),
        }
    }
}
