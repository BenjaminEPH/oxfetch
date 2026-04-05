mod cli;
mod config;
mod logo;
mod sys;
use sys::SysInfo;
fn main() {
    let logo = logo::get_logo(None);
    let info = SysInfo::collect();

    let logo_lines: Vec<&str> = logo.lines().collect();
    let info_lines: Vec<String> = info
        .fields
        .iter()
        .map(|f| format!("{:<12} {}", f.label(), f.value()))
        .collect();

    let max_lines = logo_lines.len().max(info_lines.len());
    let logo_width = logo_lines.iter().map(|l| l.len()).max().unwrap_or(0);

    for i in 0..max_lines {
        let logo_col = logo_lines.get(i).unwrap_or(&"");
        let info_col = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        println!("{:<width$}   {}", logo_col, info_col, width = logo_width);
    }
}
