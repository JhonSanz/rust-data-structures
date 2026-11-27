// =============================================================================
// RETOS 01 — FUNDAMENTOS                         (repasa: 01_fundamentos)
// Comprueba tu solución:   cargo test -p retos --test r01
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.

/// RETO 1 — Temperatura
/// Convierte grados Celsius a Fahrenheit:  F = C × 9/5 + 32
fn celsius_a_fahrenheit(c: f64) -> f64 {
    todo!()
}

#[test]
fn reto_1_celsius_a_fahrenheit() {
    assert_eq!(celsius_a_fahrenheit(0.0), 32.0);
    assert_eq!(celsius_a_fahrenheit(100.0), 212.0);
    assert!((celsius_a_fahrenheit(-40.0) - -40.0).abs() < 1e-9);
    assert!((celsius_a_fahrenheit(36.6) - 97.88).abs() < 1e-9);
}

/// RETO 2 — Horas, minutos y segundos
/// Convierte una cantidad de segundos en una tupla (horas, minutos, segundos).
/// Ejemplo: 3661 → (1, 1, 1)
/// Pista: división entera `/` y resto `%`.
fn segundos_a_hms(total: u64) -> (u64, u64, u64) {
    todo!()
}

#[test]
fn reto_2_segundos_a_hms() {
    assert_eq!(segundos_a_hms(0), (0, 0, 0));
    assert_eq!(segundos_a_hms(3661), (1, 1, 1));
    assert_eq!(segundos_a_hms(86_399), (23, 59, 59));
    assert_eq!(segundos_a_hms(90_000), (25, 0, 0));
}

/// RETO 3 — Iniciales
/// Devuelve las iniciales en mayúscula, cada una seguida de un punto.
/// Ejemplo: "ada lovelace" → "A.L."   Los espacios extra se ignoran.
/// Pista: `split_whitespace()`, `chars().next()` y `to_uppercase()`
///        (este último devuelve un iterador: `String::extend` lo acepta).
fn iniciales(nombre: &str) -> String {
    todo!()
}

#[test]
fn reto_3_iniciales() {
    assert_eq!(iniciales("ada lovelace"), "A.L.");
    assert_eq!(iniciales("  grace   brewster hopper "), "G.B.H.");
    assert_eq!(iniciales("émile"), "É.");
    assert_eq!(iniciales(""), "");
}

/// RETO 4 — Conversión segura
/// Convierte un i64 a u8 SOLO si cabe (0..=255); si no, devuelve None.
/// Prohibido usar `as` (perdería información en silencio).
/// Pista: `u8::try_from(...)` devuelve un Result; `.ok()` lo convierte en Option.
fn a_byte(n: i64) -> Option<u8> {
    todo!()
}

#[test]
fn reto_4_a_byte() {
    assert_eq!(a_byte(0), Some(0));
    assert_eq!(a_byte(255), Some(255));
    assert_eq!(a_byte(256), None);
    assert_eq!(a_byte(-1), None);
}

/// RETO 5 — Formato de precios
/// Recibe céntimos y devuelve el precio con 2 decimales y el símbolo €.
/// Ejemplo: 123456 → "1234.56 €",  5 → "0.05 €"
/// Pista: `format!` con `{:02}` rellena con ceros a 2 dígitos. Sin usar f64.
fn formatear_precio(centimos: u64) -> String {
    todo!()
}

#[test]
fn reto_5_formatear_precio() {
    assert_eq!(formatear_precio(123_456), "1234.56 €");
    assert_eq!(formatear_precio(5), "0.05 €");
    assert_eq!(formatear_precio(100), "1.00 €");
    assert_eq!(formatear_precio(0), "0.00 €");
}

/// RETO 6 — Desbordamiento bajo control
/// Devuelve una tupla con la suma de dos u8 de tres formas:
///   (comprobada → None si se desborda, con vuelta → da la vuelta, saturada → se queda en 255)
/// Pista: `checked_add`, `wrapping_add`, `saturating_add`.
fn sumas_u8(a: u8, b: u8) -> (Option<u8>, u8, u8) {
    todo!()
}

#[test]
fn reto_6_sumas_u8() {
    assert_eq!(sumas_u8(100, 50), (Some(150), 150, 150));
    assert_eq!(sumas_u8(250, 10), (None, 4, 255));
    assert_eq!(sumas_u8(255, 1), (None, 0, 255));
}
