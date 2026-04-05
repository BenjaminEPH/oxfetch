use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default)]
    pub logo: Option<String>,

    #[serde(default)]
    pub theme: Option<String>,

    #[serde(default)]
    pub hide: Vec<String>,
}

impl Config {
    pub fn load(path: Option<&str>) -> Self {
        let config_path = match path {
            Some(p) => PathBuf::from(p),
            None => default_config_path(),
        };
        std::fs::read_to_string(&config_path)
            .ok()
            .and_then(|contents| toml::from_str(&contents).ok())
            .unwrap_or_default()
    }
}

fn default_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("oxfetch")
        .join("config.toml")
}
