// =============================================================================
// BENCHMARK MÍNIMO SIN DEPENDENCIAS
//   cargo bench -p testing
// =============================================================================
// El atributo #[bench] solo existe en Rust nightly. En stable lo habitual es
// usar el crate `criterion` (o `divan`), que hace estadística, calentamiento y
// comparación con la ejecución anterior. Aquí medimos "a mano" para ver la idea.
//
// `harness = false` en Cargo.toml: este archivo tiene su propio main().

use std::hint::black_box;
use std::time::Instant;
use testing::{es_palindromo, invertir};

fn medir(nombre: &str, iteraciones: u32, mut f: impl FnMut()) {
    // calentamiento
    for _ in 0..iteraciones / 10 {
        f();
    }
    let inicio = Instant::now();
    for _ in 0..iteraciones {
        f();
    }
    let por_iter = inicio.elapsed() / iteraciones;
    println!("{nombre:<30} {por_iter:>12?}/iter  ({iteraciones} iteraciones)");
}

fn main() {
    let corto = "Anita lava la tina";
    let largo = "Anita lava la tina ".repeat(1000);

    // black_box impide que el optimizador elimine el cálculo (porque su
    // resultado "no se usa") o lo precalcule en compilación.
    medir("es_palindromo(corto)", 1_000_000, || {
        black_box(es_palindromo(black_box(corto)));
    });
    medir("es_palindromo(largo)", 2_000, || {
        black_box(es_palindromo(black_box(&largo)));
    });
    medir("invertir(largo)", 2_000, || {
        black_box(invertir(black_box(&largo)));
    });
}
