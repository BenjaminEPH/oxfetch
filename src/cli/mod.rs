use clap::Parser;
#[derive(Parser, Debug)]
#[command(
    name = "oxfetch",
    version,
    about = "A system info tool written in Rust."
)]
pub struct Cli {
    #[arg(long, value_name = "DISTRO")]
    pub logo: Option<String>,

    #[arg(long, value_name = "FIELD")]
    pub hide: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub config: Option<String>,

    #[arg(long, value_name = "THEME")]
    pub theme: Option<String>,
}
