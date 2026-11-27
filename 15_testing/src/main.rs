// =============================================================================
// MÓDULO 15 — TESTING
// Ejecuta el programa:  cargo run -p testing
// Ejecuta los tests:    cargo test -p testing
// (Lo importante está en src/lib.rs, tests/integracion.rs y benches/.)
// =============================================================================

use testing::{contar_palabras, es_palindromo, invertir};

fn main() {
    let frases = ["Anita lava la tina", "Hola mundo", "¿Acaso hubo búhos acá?"];
    for f in frases {
        println!(
            "{:<26} palíndromo={:<5} invertido={:?}",
            format!("{f:?}"),
            es_palindromo(f),
            invertir(f)
        );
    }

    let n = contar_palabras("Rust es seguro y rápido").unwrap();
    println!("palabras: {n}");

    println!(
        "
Ahora prueba los tests:
  cargo test -p testing                          # todos: unitarios, integración y doc-tests
  cargo test -p testing palindromo               # filtrar por nombre (subcadena)
  cargo test -p testing -- --nocapture           # ver los println! de los tests
  cargo test -p testing -- --ignored             # solo los #[ignore]
  cargo test -p testing --test integracion       # solo tests/integracion.rs
  cargo test -p testing --doc                    # solo los doc-tests
  cargo test -p testing -- --test-threads=1      # sin paralelismo
  cargo bench -p testing                         # benchmark (benches/rendimiento.rs)"
    );
}
