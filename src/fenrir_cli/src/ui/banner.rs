//! # Banner ASCII do Fenrir
//! 
//! O lobo mitológico que devora os deuses.
//! Banner editável separado para fácil customização.

use colored::Colorize;

/// Banner ASCII do Fenrir - lobo nórdico
const FENRIR_BANNER: &str = r#"
                                  ▄▄▄▄▄
                              ▄▓████████▓▄
                           ▄▓██▀▀      ▀▀██▓▄
                         ▄██▀    ▄▄████▄▄   ▀██▄
                        ▓█▀   ▄██████████▓▄   ▀█▓
                       ▓█▀  ▄████▀▀▀  ▀▀████▄  ▀█▓
                      ▐█▌  ▓███▀  ▄▓▓▄   ▀███▓  ▐█▌
                      █▓  ▐███▌  ▐████▌   ███▌  ▓█
                     ▐█▌  ████   ▐████▌   ████  ▐█▌
                     ▓█  ▐████    ▀▓▓▀    ████▌  █▓
                     █▓  ▓████▄          ▄████▓  ▓█
                    ▐██  ██████▓▄▄▄▄▄▄▄▓██████  ██▌
                    ▓██▄▄█████████████████████▄▄██▓
                    ███████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀███████
                   ▐███▀▀                    ▀▀███▌
                   ███   ▄▓████████████████▓▄   ███
                  ▐██▌ ▄███▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀███▄ ▐██▌
                  ▓██ ▐██▌                  ▐██▌ ██▓
                  ▓█▓ ▓██                    ██▓ ▓█▓
                  ▀█▓ ▓██   F E N R I R      ██▓ ▓█▀
                   ▀█▄▐██▌                  ▐██▌▄█▀
                    ▀█▄███▄                ▄███▄█▀
                     ▀▓████▓▄            ▄▓████▓▀
                       ▀▓█████▓▄▄▄▄▄▄▄▓█████▓▀
                          ▀▀▓████████████▓▀▀
                               ▀▀▀▀▀▀▀▀
"#;

/// Banner alternativo mais simples (para terminais menores)
const FENRIR_BANNER_SIMPLE: &str = r#"
      ╔═══════════════════════════════════════╗
      ║                                       ║
      ║     ▄▄▄  ▄▄▄ ▄▄  ▄ ▄▄▄  ▄ ▄▄▄        ║
      ║     █▄▄  █▄▄ █ █ █ █▄▄▀ █ █▄▄▀       ║
      ║     █    █▄▄ █  ██ █  █ █ █  █       ║
      ║                                       ║
      ║        🐺 THE WOLF THAT BITES         ║
      ╚═══════════════════════════════════════╝
"#;

/// Imprime o banner do Fenrir com cores
pub fn print_banner() {
    // Detecta tamanho do terminal para escolher banner
    let use_simple = std::env::var("FENRIR_SIMPLE_BANNER").is_ok() 
        || console::Term::stdout().size().1 < 60;

    let banner = if use_simple {
        FENRIR_BANNER_SIMPLE
    } else {
        FENRIR_BANNER
    };

    // Imprime banner com gradiente de cores
    for (i, line) in banner.lines().enumerate() {
        let colored_line = match i % 6 {
            0 => line.bright_red(),
            1 => line.red(),
            2 => line.bright_yellow(),
            3 => line.yellow(),
            4 => line.bright_white(),
            _ => line.white(),
        };
        println!("{}", colored_line);
    }
}

/// Retorna o banner como string (para testes ou outros usos)
pub fn get_banner() -> &'static str {
    FENRIR_BANNER
}

/// Retorna o banner simples
pub fn get_simple_banner() -> &'static str {
    FENRIR_BANNER_SIMPLE
}
