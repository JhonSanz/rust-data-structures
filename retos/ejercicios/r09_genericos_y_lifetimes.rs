// =============================================================================
// RETOS 09 — GENÉRICOS Y LIFETIMES               (repasa: 09_genericos_y_lifetimes)
// Comprueba tu solución:   cargo test -p retos --test r09
// =============================================================================
// ⚠️ Este archivo NO COMPILA a propósito: los retos 3 y 4 tienen errores de
// lifetimes, y en el 5 falta una restricción sin la cual el compilador no
// puede deducir el tipo de los closures del test. Arréglalos para ver el resto.
//
// En los retos 1, 5 y 6 tendrás que AÑADIR restricciones (trait bounds) a las
// firmas: puedes modificar las firmas, pero no los tests.

use std::fmt::Display;

/// RETO 1 — Función genérica con restricción
/// Devuelve una referencia al mayor elemento del slice (None si está vacío).
/// Si hay varios iguales, cualquiera vale.
/// La firma está incompleta: el compilador no sabe si T se puede comparar.
fn maximo<T>(lista: &[T]) -> Option<&T> {
    todo!()
}

#[test]
fn reto_1_maximo() {
    assert_eq!(maximo(&[3, 9, 2]), Some(&9));
    assert_eq!(maximo(&["pera", "uva", "kiwi"]), Some(&"uva"));
    assert_eq!(maximo(&[1.5, -2.0]), Some(&1.5));
    assert_eq!(maximo::<char>(&[]), None);
}

/// RETO 2 — Un tipo genérico: una pila
struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    fn new() -> Self {
        todo!()
    }
    fn apilar(&mut self, x: T) {
        todo!()
    }
    fn desapilar(&mut self) -> Option<T> {
        todo!()
    }
    /// Mira el elemento de arriba sin sacarlo.
    fn cima(&self) -> Option<&T> {
        todo!()
    }
    fn len(&self) -> usize {
        todo!()
    }
    fn esta_vacia(&self) -> bool {
        todo!()
    }
}

#[test]
fn reto_2_pila() {
    let mut p = Pila::new();
    assert!(p.esta_vacia());
    p.apilar("a".to_string());
    p.apilar("b".to_string());
    assert_eq!(p.cima().map(String::as_str), Some("b"));
    assert_eq!(p.len(), 2);
    assert_eq!(p.desapilar(), Some("b".to_string()));
    assert_eq!(p.desapilar(), Some("a".to_string()));
    assert_eq!(p.desapilar(), None);
    let mut numeros: Pila<i32> = Pila::new();
    numeros.apilar(1);
    assert_eq!(numeros.cima(), Some(&1));
}

/// RETO 3 — ARREGLA: lifetimes en una función
/// El compilador no sabe de cuál de las dos entradas sale la referencia
/// devuelta. Añade un lifetime a la firma.
fn mas_larga(a: &str, b: &str) -> &str {
    if a.chars().count() >= b.chars().count() {
        a
    } else {
        b
    }
}

#[test]
fn reto_3_mas_larga() {
    let a = String::from("larguísima");
    let r;
    {
        let b = String::from("corta");
        r = mas_larga(&a, &b);
        assert_eq!(r, "larguísima");
    }
    assert_eq!(mas_larga("ab", "cd"), "ab");
}

/// RETO 4 — ARREGLA: un struct que guarda una referencia
/// Un struct con un campo &str necesita un parámetro de lifetime.
/// Arregla la definición del struct y su bloque impl.
struct PrimeraPalabra {
    palabra: &str,
}

impl PrimeraPalabra {
    fn de(texto: &str) -> PrimeraPalabra {
        PrimeraPalabra {
            palabra: texto.split_whitespace().next().unwrap_or(""),
        }
    }
}

#[test]
fn reto_4_struct_con_referencia() {
    let texto = String::from("  hola mundo");
    let p = PrimeraPalabra::de(&texto);
    assert_eq!(p.palabra, "hola");
    assert_eq!(PrimeraPalabra::de("").palabra, "");
}

/// RETO 5 — Genéricos con where y closures
/// Cuenta los elementos que cumplen el predicado.
/// La firma está incompleta: añade una cláusula `where` que diga que F es
/// una función que recibe &T y devuelve bool.
fn contar_donde<T, F>(items: &[T], pred: F) -> usize {
    todo!()
}

#[test]
fn reto_5_contar_donde() {
    assert_eq!(contar_donde(&[1, 2, 3, 4, 5, 6], |x| x % 2 == 0), 3);
    assert_eq!(contar_donde(&["sol", "", "mar"], |s| !s.is_empty()), 2);
    let minimo = 10;
    assert_eq!(contar_donde(&[5, 15, 25], |&x| x > minimo), 2);
}

/// RETO 6 — impl con restricciones
/// `mayor_como_texto` solo tiene sentido si T se puede comparar y mostrar.
/// Devuelve "{mayor} > {menor}" o "{a} = {b}" si son iguales.
struct Par<T> {
    a: T,
    b: T,
}

impl<T> Par<T> {
    fn mayor_como_texto(&self) -> String {
        todo!()
    }
}

#[test]
fn reto_6_par() {
    assert_eq!(Par { a: 3, b: 7 }.mayor_como_texto(), "7 > 3");
    assert_eq!(Par { a: "b", b: "a" }.mayor_como_texto(), "b > a");
    assert_eq!(Par { a: 1.5, b: 1.5 }.mayor_como_texto(), "1.5 = 1.5");
}
