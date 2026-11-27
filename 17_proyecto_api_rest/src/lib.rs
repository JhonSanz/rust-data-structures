// =============================================================================
// MÓDULO 17 — PROYECTO FINAL: API REST DE TAREAS (biblioteca)
// =============================================================================
//
// La lógica vive en la biblioteca para que los tests de integración
// (tests/api.rs) puedan usarla. main.rs solo arranca el programa.
//
//   modelo.rs              → dominio: Tarea, errores y el trait Repositorio (puerto)
//   repositorio_memoria.rs → adaptador en memoria (BTreeMap + RwLock)
//   repositorio_sqlite.rs  → adaptador SQLite (rusqlite + SQL puro)
//   handlers.rs            → router de axum, handlers, middleware, errores HTTP

pub mod handlers;
pub mod modelo;
pub mod repositorio_memoria;
pub mod repositorio_sqlite;

pub use handlers::app;
pub use modelo::{ErrorDominio, Repositorio, Tarea};
pub use repositorio_memoria::RepositorioMemoria;
pub use repositorio_sqlite::RepositorioSqlite;
