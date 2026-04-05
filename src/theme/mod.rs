pub struct Theme {
    pub label_color: &'static str, // color ANSI para la etiqueta
    pub value_color: &'static str, // color ANSI para el valor
    pub logo_color: &'static str,  // color ANSI para el logo
    pub reset: &'static str,
}

pub const RESET: &str = "\x1b[0m";

pub const DEFAULT: Theme = Theme {
    label_color: "\x1b[1;36m", // cyan bold
    value_color: "\x1b[0;37m", // gris claro
    logo_color: "\x1b[1;32m",  // verde bold
    reset: RESET,
};

pub const DRACULA: Theme = Theme {
    label_color: "\x1b[1;35m", // purple bold
    value_color: "\x1b[0;97m", // blanco
    logo_color: "\x1b[1;35m",  // purple bold
    reset: RESET,
};

pub const NORD: Theme = Theme {
    label_color: "\x1b[1;34m", // azul bold
    value_color: "\x1b[0;96m", // cyan claro
    logo_color: "\x1b[1;34m",  // azul bold
    reset: RESET,
};

pub const GRUVBOX: Theme = Theme {
    label_color: "\x1b[1;33m", // amarillo bold
    value_color: "\x1b[0;37m", // gris
    logo_color: "\x1b[1;31m",  // rojo bold
    reset: RESET,
};

pub const TOKYO_NIGHT_STORM: Theme = Theme {
    label_color: "\x1b[1;38;2;187;154;247m", // purple  #BB9AF7
    value_color: "\x1b[0;38;2;169;177;214m", // fg      #A9B1D6
    logo_color: "\x1b[1;38;2;122;162;247m",  // blue    #7AA2F7
    reset: RESET,
};

pub fn get_theme(name: Option<&str>) -> &'static Theme {
    match name.map(|s| s.to_lowercase()).as_deref() {
        Some("dracula") => &DRACULA,
        Some("nord") => &NORD,
        Some("gruvbox") => &GRUVBOX,
        Some("tokyo-night-storm") => &TOKYO_NIGHT_STORM,
        _ => &DEFAULT,
    }
}
