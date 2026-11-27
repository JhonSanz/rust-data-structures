// =============================================================================
// RETOS 04 — OWNERSHIP Y BORROWING               (repasa: 04_ownership_y_borrowing)
// Comprueba tu solución:   cargo test -p retos --test r04
// =============================================================================
// ⚠️ Este archivo NO COMPILA a propósito: los retos 1 y 3 son errores del
// borrow checker que tienes que arreglar. Hasta que lo hagas, no podrás ver el
// resultado del resto. Lee los errores con:  cargo test -p retos --test r04
//
// En los retos "ARREGLA" puedes cambiar el código de la función; en los demás,
// sustituye `todo!()`. NO modifiques los tests. Prohibido usar `.clone()`.

/// RETO 1 — ARREGLA: prestar en vez de mover
/// Esta función se queda con el String que recibe, así que el test no compila.
/// Cambia SOLO la firma para que tome el texto prestado y acepte también literales.
fn longitud(texto: String) -> usize {
    texto.chars().count()
}

#[test]
fn reto_1_prestar_en_vez_de_mover() {
    let nombre = String::from("Ferris 🦀");
    assert_eq!(longitud(&nombre), 8);
    assert_eq!(longitud("hola"), 4);
    assert_eq!(nombre, "Ferris 🦀"); // `nombre` sigue siendo nuestro
}

/// RETO 2 — Modificar a través de &mut
/// Añade `sufijo` al final de `texto`, salvo que ya termine en él.
/// Ejemplo: "foto" + ".jpg" → "foto.jpg";  "foto.jpg" + ".jpg" → "foto.jpg"
fn anadir_sufijo(texto: &mut String, sufijo: &str) {
    todo!()
}

#[test]
fn reto_2_anadir_sufijo() {
    let mut archivo = String::from("foto");
    anadir_sufijo(&mut archivo, ".jpg");
    assert_eq!(archivo, "foto.jpg");
    anadir_sufijo(&mut archivo, ".jpg");
    assert_eq!(archivo, "foto.jpg");
}

/// RETO 3 — ARREGLA: préstamo inmutable y mutable a la vez
/// Debe añadir al final el doble del primer elemento y devolver ese primer
/// elemento. El borrow checker lo rechaza: entiende por qué y arréglalo.
fn duplicar_primero(v: &mut Vec<i32>) -> i32 {
    let primero = &v[0];
    v.push(*primero * 2);
    *primero
}

#[test]
fn reto_3_duplicar_primero() {
    let mut v = vec![3, 1];
    assert_eq!(duplicar_primero(&mut v), 3);
    assert_eq!(v, [3, 1, 6]);
}

/// RETO 4 — Devolver un slice, sin copiar
/// Devuelve la palabra más larga (en caracteres) como un trozo del texto
/// original. Si hay empate, la primera. Si no hay palabras, "".
/// Requisito: no crees ningún String: devuelve un &str que apunte a `texto`.
fn palabra_mas_larga(texto: &str) -> &str {
    todo!()
}

#[test]
fn reto_4_palabra_mas_larga() {
    let frase = String::from("el murciélago come kiwis");
    let larga = palabra_mas_larga(&frase);
    assert_eq!(larga, "murciélago");
    // Comprobamos que es un trozo de `frase` y no una copia
    let rango = frase.as_bytes().as_ptr_range();
    assert!(rango.contains(&larga.as_ptr()));
    assert_eq!(palabra_mas_larga("uno dos"), "uno");
    assert_eq!(palabra_mas_larga("   "), "");
}

/// RETO 5 — Quitarle el contenido a una referencia mutable
/// Devuelve todos los elementos de `v` y lo deja VACÍO, sin clonar nada.
/// Pista: no puedes mover un valor fuera de una `&mut`... salvo que dejes otro
/// en su lugar. Mira `std::mem::take`.
fn vaciar(v: &mut Vec<String>) -> Vec<String> {
    todo!()
}

#[test]
fn reto_5_vaciar() {
    let mut nombres = vec![String::from("Ana"), String::from("Luis")];
    let sacados = vaciar(&mut nombres);
    assert_eq!(sacados, ["Ana", "Luis"]);
    assert!(nombres.is_empty());
}
