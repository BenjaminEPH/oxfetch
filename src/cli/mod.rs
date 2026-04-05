use clap::Parser;
#[derive(Parser, Debug)]
#[command(
    name = "oxfetch",
    version,
    about = "A system info tool written in Rust."
)]
pub struct Cli {
    // Overrride the logo from the scanned distro
    // Sobreescribir el logo de la distribucion
    #[arg(long, value_name = "DISTRO")]
    pub logo: Option<String>,
    // Hide especific fields (e.g. ---hide cpu or --hide memory)
    // Esoonder campos en especifico (ej. --hide cpu o --hide memory)
    #[arg(long, value_name = "FIELD")]
    pub hide: Vec<String>,
    // Path to a custom config file
    // Ruta a un archivo de configuracion personalizado
    #[arg(long, value_name = "PATH")]
    pub config: Option<String>,
    // Color theme to use (default, dracula, nord, gruvbox)
    // Esquema de color a usar (default, dracula, nord, gruvbox)
    #[arg(long, value_name = "THEME")]
    pub theme: Option<String>,
}
