// =============================================================================
// src/calculadora/historial.rs — submódulo `calculadora::historial`
// =============================================================================

/// Lista de operaciones realizadas.
#[derive(Debug, Default)]
pub struct Historial {
    entradas: Vec<String>,
}

impl Historial {
    // pub(super): visible SOLO para el módulo padre (calculadora).
    // El usuario del crate no puede añadir entradas falsas al historial.
    pub(super) fn anadir(&mut self, entrada: String) {
        self.entradas.push(entrada);
    }

    pub fn len(&self) -> usize {
        self.entradas.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entradas.is_empty()
    }

    /// Devuelve las entradas como un iterador de `&str`.
    pub fn entradas(&self) -> impl Iterator<Item = &str> {
        self.entradas.iter().map(String::as_str)
    }
}
