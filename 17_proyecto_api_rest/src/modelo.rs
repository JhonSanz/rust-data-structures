// =============================================================================
// DOMINIO: la entidad, sus reglas y el "puerto" (trait) de persistencia.
//
// Este archivo NO sabe nada de HTTP ni de SQL. Los ADAPTADORES implementan el
// trait Repositorio:
//
//   repositorio_memoria.rs → BTreeMap + RwLock (rápido, se pierde al reiniciar)
//   repositorio_sqlite.rs  → SQLite con SQL puro vía rusqlite
//
// Los handlers solo dependen del trait (como `Arc<dyn Repositorio>`), así que el
// adaptador se elige al arrancar (--db) sin tocar una línea de los handlers.
// Es la arquitectura de "puertos y adaptadores" (hexagonal).
// =============================================================================

use serde::Serialize;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// El recurso que expone la API.
///
/// `#[derive(Serialize)]` (del crate serde) genera la conversión a JSON en
/// compilación, sin reflexión: es el equivalente a los struct tags de Go.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Tarea {
    pub id: i64,
    pub titulo: String,
    pub completada: bool,
    /// Segundos desde 1970 (UTC). Para fechas legibles usarías `jiff` o `chrono`.
    pub creada_en: i64,
}

/// Errores del dominio: los handlers los traducen a códigos HTTP.
/// Cada adaptador debe traducir SUS errores (p. ej. QueryReturnedNoRows) a estos.
#[derive(Debug, PartialEq)]
pub enum ErrorDominio {
    NoEncontrada,
    TituloVacio,
    TituloLargo,
    /// Fallo de infraestructura (BD caída, disco lleno...). El detalle se
    /// registra en el log pero NUNCA se envía al cliente.
    Interno(String),
}

impl fmt::Display for ErrorDominio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorDominio::NoEncontrada => write!(f, "tarea no encontrada"),
            ErrorDominio::TituloVacio => write!(f, "el título es obligatorio"),
            ErrorDominio::TituloLargo => {
                write!(f, "el título no puede superar {MAX_TITULO} caracteres")
            }
            ErrorDominio::Interno(detalle) => write!(f, "error interno: {detalle}"),
        }
    }
}

impl std::error::Error for ErrorDominio {}

pub type Resultado<T> = Result<T, ErrorDominio>;

/// El PUERTO de persistencia que necesitan los handlers.
///
/// Decisiones de diseño:
///   - `Send + Sync`: se comparte entre las tareas de Tokio (varios hilos).
///   - Métodos SÍNCRONOS: rusqlite es síncrono, así que los handlers los
///     llaman dentro de `spawn_blocking` para no bloquear el runtime async.
///     Además, un trait síncrono se puede usar como `dyn Repositorio`.
///   - Todo devuelve Result: una BD puede fallar, algo que un map nunca hace.
pub trait Repositorio: Send + Sync {
    fn listar(&self) -> Resultado<Vec<Tarea>>;
    fn obtener(&self, id: i64) -> Resultado<Tarea>;
    fn crear(&self, titulo: &str) -> Resultado<Tarea>;
    fn actualizar(&self, id: i64, completada: bool) -> Resultado<Tarea>;
    fn eliminar(&self, id: i64) -> Resultado<()>;
}

pub const MAX_TITULO: usize = 100;

/// Reglas de negocio del título. Viven en el dominio para que TODOS los
/// adaptadores validen exactamente igual.
pub fn normalizar_titulo(titulo: &str) -> Resultado<String> {
    let titulo = titulo.trim();
    if titulo.is_empty() {
        Err(ErrorDominio::TituloVacio)
    } else if titulo.chars().count() > MAX_TITULO {
        // contar chars, no bytes (UTF-8)
        Err(ErrorDominio::TituloLargo)
    } else {
        Ok(titulo.to_string())
    }
}

/// Instante actual en segundos Unix.
pub fn ahora() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizar_titulo_aplica_las_reglas() {
        assert_eq!(normalizar_titulo("  Hola  "), Ok("Hola".to_string()));
        assert_eq!(normalizar_titulo("   "), Err(ErrorDominio::TituloVacio));
        assert_eq!(
            normalizar_titulo(&"ñ".repeat(100)).map(|t| t.chars().count()),
            Ok(100)
        );
        assert_eq!(
            normalizar_titulo(&"ñ".repeat(101)),
            Err(ErrorDominio::TituloLargo)
        );
    }
}
