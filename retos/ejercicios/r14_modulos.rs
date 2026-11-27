// =============================================================================
// RETOS 14 — MÓDULOS, VISIBILIDAD Y MACROS       (repasa: 14_modulos_y_crates)
// Comprueba tu solución:   cargo test -p retos --test r14
// =============================================================================
// ⚠️ Este archivo NO COMPILA a propósito: el reto 1 es un problema de
// visibilidad. En el reto 1 puedes modificar el módulo `biblioteca`; en los
// demás, sustituye `todo!()`. NO modifiques los tests.

// -----------------------------------------------------------------------------
/// RETO 1 — ARREGLA la visibilidad
/// Los tests no compilan porque casi todo es privado. Haz públicos SOLO los
/// elementos que el test necesita, siguiendo estas reglas:
///   - Los tests deben poder escribir `biblioteca::Libro` (re-exporta con `pub use`
///     en vez de hacer público el módulo `catalogo`).
///   - El campo `titulo` debe seguir siendo PRIVADO (se lee con el método).
///   - `comision_interna` no debe ser accesible desde fuera de `biblioteca`
///     (pista: `pub(super)` o `pub(crate)` para que `utilidades` la use).
mod biblioteca {
    use catalogo::Libro;

    mod catalogo {
        pub struct Libro {
            titulo: String,
            prestado: bool,
        }

        impl Libro {
            fn nuevo(titulo: &str) -> Libro {
                Libro {
                    titulo: titulo.to_string(),
                    prestado: false,
                }
            }
            fn titulo(&self) -> &str {
                &self.titulo
            }
            fn prestado(&self) -> bool {
                self.prestado
            }
            fn prestar(&mut self) -> Result<(), String> {
                if self.prestado {
                    return Err(format!("'{}' ya está prestado", self.titulo));
                }
                self.prestado = true;
                Ok(())
            }
        }

        fn comision_interna() -> u32 {
            2
        }
    }

    mod utilidades {
        use super::Libro;

        fn formatear(libro: &Libro) -> String {
            let estado = if libro.prestado() {
                "prestado"
            } else {
                "disponible"
            };
            format!(
                "{} ({estado}, comisión {}€)",
                libro.titulo(),
                super::catalogo::comision_interna()
            )
        }
    }
}

#[test]
fn reto_1_visibilidad() {
    use biblioteca::Libro;
    use biblioteca::utilidades::formatear;
    let mut libro = Libro::nuevo("Dune");
    assert_eq!(libro.titulo(), "Dune");
    assert_eq!(formatear(&libro), "Dune (disponible, comisión 2€)");
    assert_eq!(libro.prestar(), Ok(()));
    assert_eq!(libro.prestar(), Err("'Dune' ya está prestado".to_string()));
    assert_eq!(formatear(&libro), "Dune (prestado, comisión 2€)");
}

// -----------------------------------------------------------------------------
/// RETO 2 — Una macro declarativa: `mapa!`
/// mapa!{"a" => 1, "b" => 2} crea un HashMap con esos pares. Debe aceptar
/// una coma final opcional y también la forma vacía `mapa!{}`.
/// Pista: el patrón `$( $k:expr => $v:expr ),* $(,)?` y un bloque `{ ... }` que
/// cree el mapa, haga los insert repetidos con `$( ... )*` y lo devuelva.
macro_rules! mapa {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {
        todo!()
    };
}

#[test]
fn reto_2_macro_mapa() {
    use std::collections::HashMap;
    let m: HashMap<&str, i32> = mapa! { "uno" => 1, "dos" => 2, };
    assert_eq!(m.len(), 2);
    assert_eq!(m["dos"], 2);
    let vacio: HashMap<String, u8> = mapa! {};
    assert!(vacio.is_empty());
    let calculado: HashMap<i32, i32> = mapa! { 1 + 1 => 2 * 10 };
    assert_eq!(calculado[&2], 20);
}

/// RETO 3 — Una macro recursiva: `maximo!`
/// maximo!(3) == 3;  maximo!(1, 5, 2) == 5. Acepta cualquier número de expresiones ≥ 1.
/// Pista: dos reglas. Una para un solo elemento y otra para "primero, resto...",
/// que se llama a sí misma con el resto.
macro_rules! maximo {
    ( $x:expr ) => {
        todo!()
    };
    ( $x:expr, $( $resto:expr ),+ ) => {
        todo!()
    };
}

#[test]
fn reto_3_macro_maximo() {
    let a: i32 = maximo!(3);
    let b: i32 = maximo!(1, 5, 2);
    let c: f64 = maximo!(0.5, -1.0, 2.25, 2.0);
    let d: &str = maximo!("pera", "uva", "kiwi");
    assert_eq!((a, b, c, d), (3, 5, 2.25, "uva"));
}

/// RETO 4 — Compilación condicional
/// Devuelve "debug" si el código se compila con debug_assertions activadas
/// (cargo test normal) y "release" en otro caso (cargo test --release).
/// Requisito: usa `cfg!(...)`.
fn modo_compilacion() -> &'static str {
    todo!()
}

#[test]
fn reto_4_modo_compilacion() {
    let esperado = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    assert_eq!(modo_compilacion(), esperado);
}
