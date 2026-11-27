// =============================================================================
// ADAPTADOR en memoria: BTreeMap protegido por un RwLock.
// =============================================================================

use crate::modelo::{ErrorDominio, Repositorio, Resultado, Tarea, ahora, normalizar_titulo};
use std::collections::BTreeMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Default)]
struct Estado {
    // BTreeMap mantiene las tareas ordenadas por id: listar sale ordenado gratis
    tareas: BTreeMap<i64, Tarea>,
    siguiente_id: i64,
}

/// Repositorio en memoria. Los datos se pierden al reiniciar.
///
/// El RwLock deja que muchas peticiones LEAN a la vez y serializa las escrituras.
/// El Mutex/RwLock CONTIENE los datos: es imposible tocarlos sin el lock.
#[derive(Default)]
pub struct RepositorioMemoria {
    estado: RwLock<Estado>,
}

impl RepositorioMemoria {
    pub fn new() -> Self {
        Self::default()
    }

    // Si un hilo hace panic con el lock tomado, el lock queda "envenenado".
    // Aquí decidimos recuperar los datos igualmente (into_inner), porque cada
    // operación deja el estado consistente antes de poder hacer panic.
    fn leer(&self) -> RwLockReadGuard<'_, Estado> {
        self.estado.read().unwrap_or_else(|e| e.into_inner())
    }
    fn escribir(&self) -> RwLockWriteGuard<'_, Estado> {
        self.estado.write().unwrap_or_else(|e| e.into_inner())
    }
}

impl Repositorio for RepositorioMemoria {
    fn listar(&self) -> Resultado<Vec<Tarea>> {
        Ok(self.leer().tareas.values().cloned().collect())
    }

    fn obtener(&self, id: i64) -> Resultado<Tarea> {
        self.leer()
            .tareas
            .get(&id)
            .cloned()
            .ok_or(ErrorDominio::NoEncontrada)
    }

    fn crear(&self, titulo: &str) -> Resultado<Tarea> {
        let titulo = normalizar_titulo(titulo)?;
        let mut estado = self.escribir();
        estado.siguiente_id += 1;
        let tarea = Tarea {
            id: estado.siguiente_id,
            titulo,
            completada: false,
            creada_en: ahora(),
        };
        estado.tareas.insert(tarea.id, tarea.clone());
        Ok(tarea)
    }

    fn actualizar(&self, id: i64, completada: bool) -> Resultado<Tarea> {
        let mut estado = self.escribir();
        let tarea = estado
            .tareas
            .get_mut(&id)
            .ok_or(ErrorDominio::NoEncontrada)?;
        tarea.completada = completada;
        Ok(tarea.clone())
    }

    fn eliminar(&self, id: i64) -> Resultado<()> {
        self.escribir()
            .tareas
            .remove(&id)
            .map(|_| ())
            .ok_or(ErrorDominio::NoEncontrada)
    }
}
