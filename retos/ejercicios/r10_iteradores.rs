// =============================================================================
// RETOS 10 — ITERADORES                          (repasa: 10_iteradores_y_closures)
// Comprueba tu solución:   cargo test -p retos --test r10
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.
// Regla del módulo: sin `for`, `while` ni `loop`. Solo adaptadores y consumidores.

use std::collections::BTreeMap;

/// RETO 1 — Suma de los cuadrados de los pares
fn suma_cuadrados_pares(v: &[i64]) -> i64 {
    todo!()
}

#[test]
fn reto_1_suma_cuadrados_pares() {
    assert_eq!(suma_cuadrados_pares(&[1, 2, 3, 4]), 20);
    assert_eq!(suma_cuadrados_pares(&[-2, 5]), 4);
    assert_eq!(suma_cuadrados_pares(&[]), 0);
}

/// RETO 2 — Capitalizar palabras
/// "hola   mundo cruel" → "Hola Mundo Cruel" (las palabras se unen con UN espacio)
/// Pista: `split_whitespace`, `chars`, `to_uppercase`, `chain`, `collect`, `join`.
fn capitalizar(texto: &str) -> String {
    todo!()
}

#[test]
fn reto_2_capitalizar() {
    assert_eq!(capitalizar("hola   mundo cruel"), "Hola Mundo Cruel");
    assert_eq!(capitalizar("ñandú éxito"), "Ñandú Éxito");
    assert_eq!(capitalizar(""), "");
}

/// RETO 3 — Un iterador propio: la secuencia de Collatz
/// Empieza en n; si es par, el siguiente es n/2; si es impar, 3n+1.
/// Termina DESPUÉS de producir el 1.   Collatz::new(6) → 6 3 10 5 16 8 4 2 1
/// (Aquí sí puedes usar if/match dentro de `next`.)
struct Collatz {
    actual: Option<u64>,
}

impl Collatz {
    fn new(n: u64) -> Self {
        todo!()
    }
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        todo!()
    }
}

#[test]
fn reto_3_collatz() {
    assert_eq!(
        Collatz::new(6).collect::<Vec<_>>(),
        [6, 3, 10, 5, 16, 8, 4, 2, 1]
    );
    assert_eq!(Collatz::new(1).collect::<Vec<_>>(), [1]);
    assert_eq!(Collatz::new(27).count(), 112);
    assert_eq!(Collatz::new(27).max(), Some(9232)); // ¡gratis por implementar Iterator!
}

/// RETO 4 — Media por categoría
/// Devuelve (categoría, media) ordenado por categoría.
/// Pista: agrupa en un BTreeMap<&str, (suma, cuenta)> con `fold`.
fn media_por_categoria(datos: &[(&str, f64)]) -> Vec<(String, f64)> {
    todo!()
}

#[test]
fn reto_4_media_por_categoria() {
    let datos = [
        ("fruta", 2.0),
        ("pan", 1.0),
        ("fruta", 4.0),
        ("leche", 0.9),
        ("pan", 2.0),
    ];
    assert_eq!(
        media_por_categoria(&datos),
        [
            ("fruta".to_string(), 3.0),
            ("leche".to_string(), 0.9),
            ("pan".to_string(), 1.5)
        ]
    );
}

/// RETO 5 — Contar subidas
/// ¿Cuántas veces un elemento es mayor que el anterior?  [1, 3, 2, 5, 6] → 3
/// Pista: `windows(2)`.
fn contar_subidas(v: &[i32]) -> usize {
    todo!()
}

#[test]
fn reto_5_contar_subidas() {
    assert_eq!(contar_subidas(&[1, 3, 2, 5, 6]), 3);
    assert_eq!(contar_subidas(&[5, 4, 3]), 0);
    assert_eq!(contar_subidas(&[7]), 0);
}

/// RETO 6 — Iteradores infinitos y perezosos
/// Devuelve los `n` primeros números primos.
/// Pista: `(2..)` es infinito; `filter` + `take` lo vuelven finito.
fn primeros_primos(n: usize) -> Vec<u64> {
    todo!()
}

#[test]
fn reto_6_primeros_primos() {
    assert_eq!(primeros_primos(10), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    assert_eq!(primeros_primos(1000).last(), Some(&7919));
    assert!(primeros_primos(0).is_empty());
}
