use std::env;

use super::InfoSource;
pub struct ShellInfo {
    name: String,
}

impl InfoSource for ShellInfo {
    fn label(&self) -> &str {
        "SHELL"
    }

    fn value(&self) -> String {
        self.name.clone()
    }
}

impl ShellInfo {
    pub fn new() -> Self {
        let name = Self::fetch_shell();

        Self { name }
    }

    fn fetch_shell() -> String {
        env::var("SHELL")
            .map(|path| path.split('/').last().unwrap_or("Unknown").to_string())
            .unwrap_or_else(|_| "Unknown".to_string())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fetch_shell_not_unknown() {
        let shell = ShellInfo::fetch_shell();
        assert_ne!(shell, "Unknown");
    }
}
