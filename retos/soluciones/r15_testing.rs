// =============================================================================
// RETOS 15 — TESTING                             (repasa: 15_testing)
// Comprueba tu solución:   cargo test -p retos --test r15
// =============================================================================
// Aquí los papeles se invierten: en varios retos eres TÚ quien escribe los
// tests. Un test con `todo!()` falla, así que el reto cuenta como pendiente
// hasta que lo escribas. Sé honesto: un test que pasa sin comprobar nada no
// demuestra que hayas aprendido.

use std::num::ParseIntError;

/// Año bisiesto: divisible por 4, salvo los divisibles por 100,
/// excepto los divisibles por 400. (Esta función es CORRECTA.)
fn es_bisiesto(anio: u32) -> bool {
    (anio % 4 == 0 && anio % 100 != 0) || anio % 400 == 0
}

/// RETO 1 — Escribe un test table-driven
/// Comprueba `es_bisiesto` con al menos 6 casos, incluyendo los cuatro tipos:
/// no divisible por 4, divisible por 4, divisible por 100 y divisible por 400.
/// Usa un array de tuplas y un mensaje en el assert que diga qué año falló.
#[test]
fn reto_1_tabla_bisiestos() {
    let casos = [
        (2023, false),
        (2024, true),
        (1900, false),
        (2100, false),
        (2000, true),
        (1600, true),
    ];
    for (anio, esperado) in casos {
        assert_eq!(es_bisiesto(anio), esperado, "año {anio}");
    }
}

/// RETO 2 — Encuentra y arregla el bug
/// `mediana` tiene un error con las listas de longitud PAR (la mediana de
/// [1, 2, 3, 4] es 2.5). Primero escribe un test que lo demuestre en
/// `reto_2_tu_test_de_mediana` y comprueba que falla; luego arregla la función.
fn mediana(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    let m = v.len() / 2;
    if v.len() % 2 == 0 {
        Some((v[m - 1] + v[m]) / 2.0)
    } else {
        Some(v[m])
    }
}

#[test]
fn reto_2_tu_test_de_mediana() {
    assert_eq!(mediana(&mut [4.0, 1.0, 3.0, 2.0]), Some(2.5));
    assert_eq!(mediana(&mut [5.0, 1.0, 3.0]), Some(3.0));
    assert_eq!(mediana(&mut []), None);
}

#[test]
fn reto_2_mediana_arreglada() {
    assert_eq!(mediana(&mut [3.0, 1.0, 2.0]), Some(2.0));
    assert_eq!(mediana(&mut [10.0, 1.0, 3.0, 2.0]), Some(2.5));
    assert_eq!(mediana(&mut [7.0, 3.0]), Some(5.0));
    assert_eq!(mediana(&mut []), None);
}

/// Calcula qué porcentaje es `parte` de `total`.
/// Hace panic con "el total no puede ser 0" si total == 0.
fn porcentaje(parte: u32, total: u32) -> f64 {
    assert!(total != 0, "el total no puede ser 0");
    parte as f64 * 100.0 / total as f64
}

/// RETO 3 — Probar que algo hace panic
/// Escribe un test que compruebe que `porcentaje(1, 0)` hace panic CON ESE
/// MENSAJE. Tendrás que añadir un atributo a la función de test (y quitar el todo!).
#[test]
#[should_panic(expected = "el total no puede ser 0")]
fn reto_3_porcentaje_con_total_cero() {
    porcentaje(1, 0);
}

/// RETO 4 — Tests que devuelven Result
/// Cambia este test para que devuelva `Result<(), ParseIntError>` y usa `?` para
/// parsear "25" y "75" y comprobar que porcentaje(25, 100) + porcentaje(75, 100) == 100.
/// Sin unwrap.
#[test]
fn reto_4_test_con_interrogacion() -> Result<(), ParseIntError> {
    let a: u32 = "25".parse()?;
    let b: u32 = "75".parse()?;
    assert_eq!(porcentaje(a, 100) + porcentaje(b, 100), 100.0);
    Ok(())
}

/// RETO 5 — Un helper de aserción
/// Implementa `assert_aprox`: hace panic con un mensaje que incluya ambos
/// valores si |a - b| > 1e-9. Márcalo con `#[track_caller]` para que, si falla,
/// el error señale la línea del test que lo llamó.
#[track_caller]
fn assert_aprox(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-9,
        "{a} y {b} no son aproximadamente iguales"
    );
}

#[test]
fn reto_5_assert_aprox() {
    assert_aprox(0.1 + 0.2, 0.3); // con assert_eq! fallaría: 0.30000000000000004
    assert_aprox(porcentaje(1, 3), 33.333333333333336);
    let fallo = std::panic::catch_unwind(|| assert_aprox(1.0, 1.1));
    let payload = fallo.expect_err("assert_aprox(1.0, 1.1) debería fallar");
    let msg = payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default();
    assert!(
        msg.contains('1') && msg.contains("1.1"),
        "el mensaje debe incluir los valores: {msg:?}"
    );
}
