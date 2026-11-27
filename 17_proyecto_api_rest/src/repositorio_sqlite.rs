// =============================================================================
// ADAPTADOR SQLite con SQL puro (crate rusqlite).
// =============================================================================
//
// rusqlite es un envoltorio fino y síncrono sobre la librería C de SQLite.
// Con la feature `bundled` (ver Cargo.toml del workspace) compila SQLite junto
// al programa: no hace falta tenerlo instalado, y el binario no depende de
// ninguna libsqlite3 del sistema.
//
// Una `Connection` es Send pero NO Sync (no se puede usar desde dos hilos a la
// vez), así que la protegemos con un Mutex. Para mucha concurrencia usarías un
// pool (crate r2d2 + r2d2_sqlite) o un driver async (sqlx).

use crate::modelo::{ErrorDominio, Repositorio, Resultado, Tarea, ahora, normalizar_titulo};
use rusqlite::{Connection, OptionalExtension, Row, params};
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

pub struct RepositorioSqlite {
    conexion: Mutex<Connection>,
}

// Cualquier error de rusqlite que no sea "no hay filas" es un fallo interno.
// Implementar From permite usar `?` con errores de rusqlite en este archivo.
impl From<rusqlite::Error> for ErrorDominio {
    fn from(e: rusqlite::Error) -> Self {
        ErrorDominio::Interno(e.to_string())
    }
}

const ESQUEMA: &str = "
    CREATE TABLE IF NOT EXISTS tareas (
        id         INTEGER PRIMARY KEY AUTOINCREMENT,
        titulo     TEXT    NOT NULL,
        completada INTEGER NOT NULL DEFAULT 0 CHECK (completada IN (0, 1)),
        creada_en  INTEGER NOT NULL
    );
";

const COLUMNAS: &str = "id, titulo, completada, creada_en";

impl RepositorioSqlite {
    /// Abre (o crea) la base de datos en `ruta` y aplica el esquema.
    pub fn abrir(ruta: impl AsRef<Path>) -> Resultado<Self> {
        Self::desde_conexion(Connection::open(ruta)?)
    }

    /// Base de datos solo en memoria (útil en tests).
    pub fn en_memoria() -> Resultado<Self> {
        Self::desde_conexion(Connection::open_in_memory()?)
    }

    fn desde_conexion(conexion: Connection) -> Resultado<Self> {
        // WAL: los lectores no bloquean al escritor. busy_timeout: si la BD está
        // bloqueada por otro proceso, esperar hasta 5 s en vez de fallar al momento.
        conexion.pragma_update(None, "journal_mode", "WAL")?;
        conexion.busy_timeout(std::time::Duration::from_secs(5))?;
        // "Migración" mínima al arrancar
        conexion.execute_batch(ESQUEMA)?;
        Ok(Self {
            conexion: Mutex::new(conexion),
        })
    }

    fn conexion(&self) -> MutexGuard<'_, Connection> {
        self.conexion.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Solo para tests: rompe la BD a propósito para probar los errores 500.
    #[doc(hidden)]
    pub fn romper_para_tests(&self) {
        self.conexion().execute_batch("DROP TABLE tareas").unwrap();
    }
}

// Convierte una fila en Tarea. Lo usan tanto query_row como query_map.
fn fila_a_tarea(fila: &Row) -> rusqlite::Result<Tarea> {
    Ok(Tarea {
        id: fila.get("id")?,
        titulo: fila.get("titulo")?,
        completada: fila.get("completada")?, // rusqlite convierte INTEGER 0/1 en bool
        creada_en: fila.get("creada_en")?,
    })
}

impl Repositorio for RepositorioSqlite {
    fn listar(&self) -> Resultado<Vec<Tarea>> {
        let con = self.conexion();
        // prepare_cached reutiliza la sentencia compilada en llamadas siguientes
        let mut sentencia =
            con.prepare_cached(&format!("SELECT {COLUMNAS} FROM tareas ORDER BY id"))?;
        let filas = sentencia.query_map([], fila_a_tarea)?;
        // collect sobre Result: se para en el primer error de fila
        Ok(filas.collect::<Result<Vec<_>, _>>()?)
    }

    fn obtener(&self, id: i64) -> Resultado<Tarea> {
        self.conexion()
            .query_row(
                &format!("SELECT {COLUMNAS} FROM tareas WHERE id = ?1"),
                params![id],
                fila_a_tarea,
            )
            .optional()? // Ok(None) si no hay filas, en vez de Err(QueryReturnedNoRows)
            .ok_or(ErrorDominio::NoEncontrada)
    }

    fn crear(&self, titulo: &str) -> Resultado<Tarea> {
        let titulo = normalizar_titulo(titulo)?;
        let creada_en = ahora();
        let con = self.conexion();
        // Placeholders ?1, ?2: los datos NUNCA se concatenan en el SQL → sin inyección SQL
        con.execute(
            "INSERT INTO tareas (titulo, completada, creada_en) VALUES (?1, 0, ?2)",
            params![titulo, creada_en],
        )?;
        Ok(Tarea {
            id: con.last_insert_rowid(),
            titulo,
            completada: false,
            creada_en,
        })
    }

    fn actualizar(&self, id: i64, completada: bool) -> Resultado<Tarea> {
        // RETURNING: actualizar y leer el resultado en UNA sola consulta
        self.conexion()
            .query_row(
                &format!("UPDATE tareas SET completada = ?1 WHERE id = ?2 RETURNING {COLUMNAS}"),
                params![completada, id],
                fila_a_tarea,
            )
            .optional()?
            .ok_or(ErrorDominio::NoEncontrada)
    }

    fn eliminar(&self, id: i64) -> Resultado<()> {
        // execute devuelve el número de filas afectadas
        match self
            .conexion()
            .execute("DELETE FROM tareas WHERE id = ?1", params![id])?
        {
            0 => Err(ErrorDominio::NoEncontrada),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ruta_temporal(nombre: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("tareas-api-{nombre}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir.join("tareas.db")
    }

    #[test]
    fn los_datos_persisten_al_reabrir() {
        let ruta = ruta_temporal("persistencia");
        {
            let repo = RepositorioSqlite::abrir(&ruta).unwrap();
            repo.crear("sobrevivir al reinicio").unwrap();
            repo.actualizar(1, true).unwrap();
        } // aquí se cierra la conexión (Drop)
        let repo = RepositorioSqlite::abrir(&ruta).unwrap();
        let t = repo.obtener(1).unwrap();
        assert_eq!(
            (t.titulo.as_str(), t.completada),
            ("sobrevivir al reinicio", true)
        );
        // AUTOINCREMENT: los ids no se reutilizan tras borrar
        repo.eliminar(1).unwrap();
        assert_eq!(repo.crear("otra").unwrap().id, 2);
        let _ = std::fs::remove_dir_all(ruta.parent().unwrap());
    }

    #[test]
    fn traduce_los_errores_al_dominio() {
        let repo = RepositorioSqlite::en_memoria().unwrap();
        assert_eq!(repo.obtener(99), Err(ErrorDominio::NoEncontrada));
        assert_eq!(repo.actualizar(99, true), Err(ErrorDominio::NoEncontrada));
        assert_eq!(repo.eliminar(99), Err(ErrorDominio::NoEncontrada));
        assert_eq!(repo.crear("  "), Err(ErrorDominio::TituloVacio));
    }

    #[test]
    fn no_es_vulnerable_a_inyeccion_sql() {
        let repo = RepositorioSqlite::en_memoria().unwrap();
        let malicioso = "x'); DROP TABLE tareas; --";
        let t = repo.crear(malicioso).unwrap();
        assert_eq!(repo.obtener(t.id).unwrap().titulo, malicioso);
        assert_eq!(repo.listar().unwrap().len(), 1); // la tabla sigue ahí
    }

    #[test]
    fn un_fallo_de_la_bd_es_error_interno() {
        let repo = RepositorioSqlite::en_memoria().unwrap();
        repo.romper_para_tests();
        assert!(matches!(repo.listar(), Err(ErrorDominio::Interno(_))));
    }
}
