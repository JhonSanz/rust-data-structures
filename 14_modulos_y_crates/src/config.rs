// =============================================================================
// src/config.rs — módulo PRIVADO `config`
// =============================================================================
// El módulo no es `pub` (ver lib.rs), pero `Config` se re-exporta con `pub use`.
// Es el equivalente al directorio `internal/` de Go, con mucho más control.

use std::env;

/// Configuración leída del entorno.
#[derive(Debug)]
pub struct Config {
    pub entorno: String,
    pub decimales: usize,
}

impl Config {
    /// Lee `APP_ENV` (por defecto "desarrollo").
    pub fn desde_entorno() -> Self {
        let entorno = env::var("APP_ENV").unwrap_or_else(|_| "desarrollo".to_string());
        let decimales = decimales_para(&entorno);
        Config { entorno, decimales }
    }
}

// pub(crate): visible en TODO el crate, pero no fuera de él.
pub(crate) fn decimales_para(entorno: &str) -> usize {
    if entorno == "produccion" { 2 } else { 4 }
}
