// =============================================================================
// RETOS 03 — FUNCIONES Y CLOSURES                (repasa: 03_funciones)
// Comprueba tu solución:   cargo test -p retos --test r03
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.

/// RETO 1 — Aplicar una función n veces
/// aplicar_n_veces(|x| x * 2, 3, 1) == 8   (1 → 2 → 4 → 8).  Con n = 0 devuelve x.
fn aplicar_n_veces(f: impl Fn(i32) -> i32, n: usize, x: i32) -> i32 {
    todo!()
}

#[test]
fn reto_1_aplicar_n_veces() {
    assert_eq!(aplicar_n_veces(|x| x * 2, 3, 1), 8);
    assert_eq!(aplicar_n_veces(|x| x + 10, 0, 5), 5);
    let incremento = 7; // el closure captura una variable del entorno
    assert_eq!(aplicar_n_veces(|x| x + incremento, 4, 0), 28);
}

/// RETO 2 — Componer funciones
/// Devuelve una función nueva que aplica primero `f` y luego `g`:  x → g(f(x))
/// Pista: tendrás que devolver un closure con `move`.
fn componer(f: impl Fn(i32) -> i32, g: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    |_x: i32| -> i32 { todo!() } // ← sustituye este closure por el tuyo
}

#[test]
fn reto_2_componer() {
    let doble_mas_uno = componer(|x| x * 2, |x| x + 1);
    assert_eq!(doble_mas_uno(5), 11);
    let mas_uno_doble = componer(|x| x + 1, |x| x * 2);
    assert_eq!(mas_uno_doble(5), 12);
}

/// RETO 3 — Un acumulador con estado
/// Devuelve un closure que suma cada valor recibido a un total (que empieza en
/// `inicial`) y devuelve el total acumulado tras cada llamada.
/// Pista: el closure debe ser FnMut y llevarse (`move`) su estado.
fn crear_acumulador(inicial: i32) -> impl FnMut(i32) -> i32 {
    |_x: i32| -> i32 { todo!() } // ← sustituye este closure por el tuyo
}

#[test]
fn reto_3_crear_acumulador() {
    let mut acc = crear_acumulador(10);
    assert_eq!(acc(5), 15);
    assert_eq!(acc(-20), -5);
    assert_eq!(acc(0), -5);
    let mut otro = crear_acumulador(0); // cada acumulador tiene su propio estado
    assert_eq!(otro(1), 1);
    assert_eq!(acc(1), -4);
}

/// RETO 4 — Mínimo y máximo de una sola pasada
/// Devuelve Some((mínimo, máximo)) o None si el slice está vacío.
/// Requisito: recorre el slice UNA vez (sin `iter().min()` ni `iter().max()`).
fn min_max(numeros: &[i32]) -> Option<(i32, i32)> {
    todo!()
}

#[test]
fn reto_4_min_max() {
    assert_eq!(min_max(&[3, -1, 7, 0]), Some((-1, 7)));
    assert_eq!(min_max(&[42]), Some((42, 42)));
    assert_eq!(min_max(&[]), None);
}

/// RETO 5 — Torres de Hanói (recursión)
/// Devuelve la lista de movimientos (desde, hasta) para mover `n` discos de la
/// torre `desde` a la torre `hasta` usando `aux` como auxiliar.
///   hanoi(1, 1, 3, 2) == [(1, 3)]
///   hanoi(2, 1, 3, 2) == [(1, 2), (1, 3), (2, 3)]
/// Pista: mover n discos = mover n-1 a la auxiliar, mover 1, mover n-1 encima.
fn hanoi(n: u32, desde: u8, hasta: u8, aux: u8) -> Vec<(u8, u8)> {
    todo!()
}

#[test]
fn reto_5_hanoi() {
    assert_eq!(hanoi(0, 1, 3, 2), vec![]);
    assert_eq!(hanoi(1, 1, 3, 2), vec![(1, 3)]);
    assert_eq!(hanoi(2, 1, 3, 2), vec![(1, 2), (1, 3), (2, 3)]);
    // Simulamos los 10 discos para comprobar que todos los movimientos son legales
    let movimientos = hanoi(10, 1, 3, 2);
    assert_eq!(movimientos.len(), 1023);
    let mut torres: [Vec<u32>; 3] = [(1..=10).rev().collect(), vec![], vec![]];
    for (d, h) in movimientos {
        let disco = torres[d as usize - 1]
            .pop()
            .expect("mover desde una torre vacía");
        if let Some(&arriba) = torres[h as usize - 1].last() {
            assert!(disco < arriba, "disco {disco} encima de {arriba}");
        }
        torres[h as usize - 1].push(disco);
    }
    assert_eq!(torres[2], (1..=10).rev().collect::<Vec<_>>());
}
