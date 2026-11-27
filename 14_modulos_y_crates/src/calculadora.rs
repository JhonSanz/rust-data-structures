// =============================================================================
// src/calculadora.rs — el módulo `calculadora`
// =============================================================================
//! Operaciones aritméticas con historial.

// Submódulo en src/calculadora/historial.rs
mod historial;

// Lo re-exportamos para que se use como `calculadora::Historial`
pub use historial::Historial;

/// Error de las operaciones de la calculadora.
#[derive(Debug, PartialEq)]
pub enum ErrorCalculo {
    DivisionPorCero,
}

/// Una calculadora que recuerda las operaciones realizadas.
///
/// Los campos de un struct son PRIVADOS por defecto, aunque el struct sea `pub`:
/// desde fuera no se puede escribir `Calculadora { historial: ... }`, así que el
/// usuario está obligado a usar `Calculadora::new()`. Así protegemos invariantes.
#[derive(Debug, Default)]
pub struct Calculadora {
    historial: Historial,
    /// Este campo sí es público
    pub nombre: String,
}

impl Calculadora {
    pub fn new(nombre: &str) -> Self {
        Self {
            historial: Historial::default(),
            nombre: nombre.to_string(),
        }
    }

    pub fn sumar(&mut self, a: f64, b: f64) -> f64 {
        self.registrar(format!("{a} + {b}"), a + b)
    }

    pub fn dividir(&mut self, a: f64, b: f64) -> Result<f64, ErrorCalculo> {
        if b == 0.0 {
            return Err(ErrorCalculo::DivisionPorCero);
        }
        Ok(self.registrar(format!("{a} / {b}"), a / b))
    }

    // Compilación condicional: este método SOLO existe con la feature "cientifica"
    #[cfg(feature = "cientifica")]
    pub fn potencia(&mut self, base: f64, exp: f64) -> f64 {
        self.registrar(format!("{base} ^ {exp}"), base.powf(exp))
    }

    pub fn historial(&self) -> &Historial {
        &self.historial
    }

    // Método privado (sin `pub`): solo visible dentro de este módulo y sus hijos
    fn registrar(&mut self, op: String, resultado: f64) -> f64 {
        self.historial.anadir(format!("{op} = {resultado}"));
        resultado
    }
}

// Los tests unitarios suelen ir en el mismo archivo, en un módulo `tests`
// que solo se compila con `cargo test` (más en el módulo 15).
#[cfg(test)]
mod tests {
    use super::*; // `super` = el módulo padre

    #[test]
    fn dividir_por_cero_es_error() {
        let mut c = Calculadora::new("test");
        assert_eq!(c.dividir(1.0, 0.0), Err(ErrorCalculo::DivisionPorCero));
        // Los tests del propio módulo pueden ver lo privado:
        assert_eq!(c.historial.len(), 0);
    }
}
