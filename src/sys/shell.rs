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
        let name = env::var("SHELL")
            .map(|path| {
                // /bin/zsh → zsh
                path.split('/').last().unwrap_or("Unknown").to_string()
            })
            .unwrap_or_else(|_| "Unknown".to_string());

        Self { name }
    }
}
