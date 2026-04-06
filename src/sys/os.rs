use super::InfoSource;
pub struct OsInfo {
    name: String,
}

impl OsInfo {
    pub fn new() -> Self {
        let info = os_info::get();
        Self {
            name: format!("{} {}", info.os_type(), info.version()),
        }
    }
}
impl InfoSource for OsInfo {
    fn label(&self) -> &str {
        "OS"
    }
    fn value(&self) -> String {
        self.name.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn os_not_empty() {
        assert!(!OsInfo::new().value().is_empty());
    }
}
