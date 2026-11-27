// =============================================================================
// MÓDULO 14 — MÓDULOS Y CRATES: la raíz de la BIBLIOTECA (src/lib.rs)
// =============================================================================
//
// Un PAQUETE (lo que define un Cargo.toml) puede contener:
//   - como mucho una biblioteca:  src/lib.rs   → crate `modulos`
//   - cualquier nº de binarios:   src/main.rs, src/bin/*.rs
// El binario (main.rs) usa la biblioteca como si fuera una dependencia externa:
// `use modulos::calculadora;` — y solo ve lo que sea `pub`.
//
// Los MÓDULOS organizan el código dentro de un crate. `mod nombre;` busca:
//   src/nombre.rs   (o el estilo antiguo src/nombre/mod.rs)
// y los submódulos de `nombre` van en src/nombre/sub.rs.

//! Este comentario con `//!` documenta el crate entero (sale en `cargo doc`).
//!
//! Una mini biblioteca de calculadora para el tutorial de Rust.

// Módulo público: los usuarios del crate pueden acceder a él.
pub mod calculadora;

// Módulo PRIVADO: solo accesible dentro de este crate.
mod config;

// Re-exportar: el usuario puede escribir `modulos::Config` en vez de
// `modulos::config::Config` (y sin que el módulo `config` sea público).
pub use config::Config;

// Las macros declarativas (`macro_rules!`) generan código en compilación.
// `#[macro_export]` la pone en la raíz del crate: `modulos::lista_cuadrados!`.
/// Crea un `Vec` con los cuadrados de los números dados.
///
/// Los bloques de código de la documentación son TESTS (doc-tests): `cargo
/// test` los compila y ejecuta, así que la documentación nunca queda obsoleta.
///
/// ```
/// let v = modulos::lista_cuadrados![1, 2, 3];
/// assert_eq!(v, vec![1, 4, 9]);
/// ```
#[macro_export]
macro_rules! lista_cuadrados {
    // $( ... ),* repite el patrón para cada elemento separado por comas
    ( $( $x:expr ),* $(,)? ) => {
        vec![ $( $x * $x ),* ]
    };
}

/// Versión de la biblioteca, tomada de Cargo.toml en tiempo de compilación.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Devuelve qué features se compilaron.
pub fn features_activas() -> Vec<&'static str> {
    // `cfg!(...)` evalúa una condición de compilación como bool
    let mut v = vec![];
    if cfg!(feature = "cientifica") {
        v.push("cientifica");
    }
    if cfg!(debug_assertions) {
        v.push("(modo debug)");
    }
    v
}
