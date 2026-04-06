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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_uptime() {
        let uptime = UptimeInfo::new();
        assert!(uptime.seconds > 0);
    }
    #[test]
    fn valid_uptime_format() {
        let info = UptimeInfo::new();
        assert!(info.value().contains('h'));
        assert!(info.value().contains('m'));
    }
}
