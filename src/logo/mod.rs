use std::fs::read_to_string;

pub mod logos;

pub fn get_logo(distro_override: Option<&str>) -> &'static str {
    let distro = match distro_override {
        Some(d) => d.to_lowercase(),
        None => detect_distro(),
    };
    match distro.as_str() {
        "void" | "void linux" => logos::VOID,
        "arch" | "arch linux" => logos::ARCH,
        "ubuntu" | "debian" => logos::UBUNTU,
        "fedora" => logos::FEDORA,
        _ => logos::GENERIC,
    }
}

fn detect_distro() -> String {
    read_to_string("/etc/os-release")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("ID="))
        .map(|l| l.trim_start_matches("ID=").trim_matches('"').to_lowercase())
        .unwrap_or_else(|| "Unknown".to_string())
}
