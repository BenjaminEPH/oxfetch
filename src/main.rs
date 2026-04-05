mod cli;
mod config;
mod logo;
mod sys;
mod theme;
use clap::Parser;
use sys::SysInfo;

use cli::Cli;

use crate::config::Config;
fn main() {
    let cli = Cli::parse();
    // Load config file, then the the cli flags get priority
    let config = Config::load(cli.config.as_deref());
    // cli has priority over config.toml
    let logo_name = cli.logo.as_deref().or(config.logo.as_deref());
    let theme_name = cli.theme.as_deref().or(config.theme.as_deref());

    let hidden: Vec<String> = {
        let mut h = cli.hide.clone();
        for item in &config.hide {
            h.push(item.clone());
        }
        h.iter().map(|s| s.to_lowercase()).collect()
    };
    let theme = theme::get_theme(theme_name);
    let logo = logo::get_logo(logo_name);
    let info = SysInfo::collect();

    let logo_lines: Vec<&str> = logo.lines().collect();
    let info_lines: Vec<String> = info
        .fields
        .iter()
        .filter(|f| !hidden.contains(&f.label().to_lowercase()))
        .map(|f| {
            format!(
                "{}{:<10}{} {}{}{}",
                theme.label_color,
                f.label(),
                theme.reset,
                theme.value_color,
                f.value(),
                theme.reset,
            )
        })
        .collect();

    let max_lines = logo_lines.len().max(info_lines.len());
    let logo_width = logo_lines.iter().map(|l| l.len()).max().unwrap_or(0);

    for i in 0..max_lines {
        let logo_col = logo_lines.get(i).unwrap_or(&"");
        let info_col = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        println!(
            "{}{:<width$}{}   {}",
            theme.logo_color,
            logo_col,
            theme.reset,
            info_col,
            width = logo_width
        );
    }
}
