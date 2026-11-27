// =============================================================================
// RETOS 02 — CONTROL DE FLUJO                    (repasa: 02_control_de_flujo)
// Comprueba tu solución:   cargo test -p retos --test r02
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.

/// RETO 1 — FizzBuzz con match
/// Devuelve "FizzBuzz" si n es múltiplo de 3 y de 5, "Fizz" si solo de 3,
/// "Buzz" si solo de 5, y el número como texto en otro caso.
/// Requisito: usa un `match` sobre la tupla `(n % 3, n % 5)`.
fn fizzbuzz(n: u32) -> String {
    todo!()
}

#[test]
fn reto_1_fizzbuzz() {
    let r: Vec<String> = (1..=15).map(fizzbuzz).collect();
    assert_eq!(
        r,
        [
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
    );
}

/// RETO 2 — Calificaciones
/// Clasifica una nota de 0 a 10:
///   0–4 "suspenso" · 5–6 "aprobado" · 7–8 "notable" · 9–10 "sobresaliente" · otra "inválida"
/// Requisito: un único `match` con rangos (`a..=b`).
fn clasificar_nota(nota: u8) -> &'static str {
    todo!()
}

#[test]
fn reto_2_clasificar_nota() {
    let casos = [
        (0, "suspenso"),
        (4, "suspenso"),
        (5, "aprobado"),
        (6, "aprobado"),
        (7, "notable"),
    ];
    for (nota, esperado) in casos {
        assert_eq!(clasificar_nota(nota), esperado, "nota {nota}");
    }
    assert_eq!(clasificar_nota(10), "sobresaliente");
    assert_eq!(clasificar_nota(11), "inválida");
}

/// RETO 3 — Mínimo común múltiplo con `loop`
/// Devuelve el menor número positivo divisible por a y por b (a, b ≥ 1).
/// Requisito: usa `loop` y devuelve el resultado con `break valor`.
fn mcm(a: u64, b: u64) -> u64 {
    todo!()
}

#[test]
fn reto_3_mcm() {
    assert_eq!(mcm(4, 6), 12);
    assert_eq!(mcm(7, 3), 21);
    assert_eq!(mcm(5, 5), 5);
    assert_eq!(mcm(1, 9), 9);
    assert_eq!(mcm(12, 18), 36);
}

/// RETO 4 — Dos números que suman un objetivo
/// Devuelve los índices (i, j), con i < j, del PRIMER par cuyos valores suman
/// `objetivo` (el de menor i; a igual i, el de menor j). None si no hay.
/// Requisito: dos `for` anidados y un `break` con etiqueta (o `return`).
fn par_que_suma(nums: &[i32], objetivo: i32) -> Option<(usize, usize)> {
    todo!()
}

#[test]
fn reto_4_par_que_suma() {
    assert_eq!(par_que_suma(&[2, 7, 11, 15], 9), Some((0, 1)));
    assert_eq!(par_que_suma(&[3, 2, 4], 6), Some((1, 2)));
    assert_eq!(par_que_suma(&[1, 5, 1, 5], 6), Some((0, 1)));
    assert_eq!(par_que_suma(&[1, 2], 10), None);
    assert_eq!(par_que_suma(&[], 0), None);
}

/// RETO 5 — Parsear un comando con `let else`
/// Una línea válida tiene EXACTAMENTE dos palabras: un comando y un entero.
///   "mover 10" → Some(("mover", 10))     "  girar   -90 " → Some(("girar", -90))
///   "saltar" → None   "mover diez" → None   "a 1 2" → None
/// Requisito: usa `let ... else { return None; }` al menos una vez.
fn parsear_comando(linea: &str) -> Option<(String, i32)> {
    todo!()
}

#[test]
fn reto_5_parsear_comando() {
    assert_eq!(parsear_comando("mover 10"), Some(("mover".to_string(), 10)));
    assert_eq!(
        parsear_comando("  girar   -90 "),
        Some(("girar".to_string(), -90))
    );
    assert_eq!(parsear_comando("saltar"), None);
    assert_eq!(parsear_comando("mover diez"), None);
    assert_eq!(parsear_comando("a 1 2"), None);
    assert_eq!(parsear_comando(""), None);
}
