// =============================================================================
// MÓDULO 15 — TESTING: el código a probar y sus TESTS UNITARIOS
// =============================================================================
//
// Rust trae el testing integrado en el lenguaje y en Cargo. Hay tres tipos:
//   1. Tests UNITARIOS:     en el mismo archivo, en `mod tests` (este archivo).
//                           Pueden probar funciones privadas.
//   2. Tests de INTEGRACIÓN: en la carpeta tests/. Usan el crate como un
//                           usuario externo: solo ven la API pública.
//   3. DOC-TESTS:           los bloques ``` de la documentación ///.
//
// Todos se ejecutan con `cargo test`.

//! Utilidades de texto para el tutorial de testing.

use std::fmt;

/// Indica si `s` se lee igual al derecho y al revés, ignorando mayúsculas,
/// espacios y signos de puntuación.
///
/// # Ejemplos
///
/// ```
/// use testing::es_palindromo;
///
/// assert!(es_palindromo("Anita lava la tina"));
/// assert!(!es_palindromo("Hola mundo"));
/// ```
pub fn es_palindromo(s: &str) -> bool {
    let letras: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    letras.iter().eq(letras.iter().rev())
}

/// Devuelve el texto al revés, respetando los caracteres Unicode.
///
/// ```
/// assert_eq!(testing::invertir("año"), "oña");
/// ```
pub fn invertir(s: &str) -> String {
    s.chars().rev().collect()
}

/// Error devuelto cuando la entrada está vacía.
#[derive(Debug, PartialEq, Eq)]
pub struct ErrorVacio;

impl fmt::Display for ErrorVacio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "texto vacío")
    }
}

impl std::error::Error for ErrorVacio {}

/// Cuenta las palabras de `s`.
///
/// # Errores
///
/// Devuelve [`ErrorVacio`] si `s` solo contiene espacios.
///
/// ```
/// assert_eq!(testing::contar_palabras("uno dos  tres"), Ok(3));
/// assert!(testing::contar_palabras("   ").is_err());
/// ```
pub fn contar_palabras(s: &str) -> Result<usize, ErrorVacio> {
    if s.trim().is_empty() {
        return Err(ErrorVacio);
    }
    Ok(s.split_whitespace().count())
}

/// Divide el texto en trozos de `n` caracteres.
///
/// # Panics
///
/// Hace panic si `n` es 0 (es un error de programación del que llama).
///
/// ```should_panic
/// testing::trocear("abc", 0);
/// ```
pub fn trocear(s: &str, n: usize) -> Vec<String> {
    assert!(n > 0, "el tamaño del trozo debe ser mayor que 0");
    let chars: Vec<char> = s.chars().collect();
    chars.chunks(n).map(|c| c.iter().collect()).collect()
}

// Función privada: solo los tests unitarios (de este archivo) pueden probarla.
fn normalizar(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Normaliza espacios y mayúsculas y cuenta palabras distintas.
pub fn palabras_distintas(s: &str) -> usize {
    let n = normalizar(s);
    let mut v: Vec<&str> = n.split(' ').filter(|p| !p.is_empty()).collect();
    v.sort_unstable();
    v.dedup();
    v.len()
}

// =============================================================================
// TESTS UNITARIOS
// =============================================================================
// #[cfg(test)]: este módulo SOLO se compila con `cargo test`; no ocupa nada en
// el binario final.
#[cfg(test)]
mod tests {
    use super::*;

    // Un test es una función con #[test]. Pasa si no hace panic.
    #[test]
    fn palindromo_simple() {
        assert!(es_palindromo("reconocer"));
    }

    // Las macros de aserción: assert!, assert_eq!, assert_ne!
    // Muestran ambos valores si fallan (por eso los tipos deben ser Debug).
    #[test]
    fn invertir_basico() {
        assert_eq!(invertir("hola"), "aloh");
        assert_ne!(invertir("hola"), "hola");
        // Mensaje personalizado (como t.Errorf en Go):
        assert_eq!(invertir(""), "", "invertir de vacío debe ser vacío");
    }

    // TABLE-DRIVEN TESTS: el patrón también es idiomático en Rust
    #[test]
    fn palindromo_tabla() {
        let casos = [
            ("vacío", "", true),
            ("una letra", "a", true),
            ("frase con espacios", "Anita lava la tina", true),
            ("con puntuación", "¡Yo hago yoga hoy!", true),
            ("mayúsculas", "Reconocer", true),
            ("no palíndromo", "Hola mundo", false),
            ("números", "12321", true),
        ];
        for (nombre, entrada, esperado) in casos {
            assert_eq!(
                es_palindromo(entrada),
                esperado,
                "caso '{nombre}': es_palindromo({entrada:?})"
            );
        }
    }

    #[test]
    fn contar_palabras_casos() {
        assert_eq!(contar_palabras("uno"), Ok(1));
        assert_eq!(contar_palabras("  uno   dos \t tres\n"), Ok(3));
        assert_eq!(contar_palabras(""), Err(ErrorVacio));
        assert!(matches!(contar_palabras("  "), Err(ErrorVacio)));
    }

    // Un test puede devolver Result: así se puede usar `?` dentro.
    #[test]
    fn con_interrogacion() -> Result<(), Box<dyn std::error::Error>> {
        let n = contar_palabras("a b c")?;
        let m: usize = "3".parse()?;
        assert_eq!(n, m);
        Ok(())
    }

    // Comprobar que algo HACE PANIC (y, opcionalmente, con qué mensaje)
    #[test]
    #[should_panic(expected = "mayor que 0")]
    fn trocear_con_cero_hace_panic() {
        trocear("abc", 0);
    }

    #[test]
    fn trocear_unicode() {
        assert_eq!(trocear("añoñu", 2), vec!["añ", "oñ", "u"]);
    }

    // Probar una función PRIVADA
    #[test]
    fn normalizar_privada() {
        assert_eq!(normalizar("  Hola   MUNDO "), "hola mundo");
    }

    // Test ignorado: lento o que necesita algo externo.
    //   cargo test -p testing -- --ignored     → ejecutar solo los ignorados
    #[test]
    #[ignore = "lento: ejecútalo con --ignored"]
    fn muy_lento() {
        let texto = "ab".repeat(1_000_000);
        assert_eq!(invertir(&invertir(&texto)), texto);
    }

    // "Property-based testing" casero: comprobar una PROPIEDAD con muchas
    // entradas pseudoaleatorias. (Con el crate `proptest` es más potente:
    // genera entradas y, si falla, reduce el caso al mínimo.)
    #[test]
    fn invertir_dos_veces_es_identidad() {
        let mut semilla: u64 = 42;
        let mut aleatorio = move || {
            // xorshift: un generador pseudoaleatorio mínimo y determinista
            semilla ^= semilla << 13;
            semilla ^= semilla >> 7;
            semilla ^= semilla << 17;
            semilla
        };
        let alfabeto: Vec<char> = "abcñá🦀 ,.".chars().collect();
        for _ in 0..1000 {
            let len = (aleatorio() % 20) as usize;
            let s: String = (0..len)
                .map(|_| alfabeto[(aleatorio() % alfabeto.len() as u64) as usize])
                .collect();
            assert_eq!(invertir(&invertir(&s)), s, "falló con {s:?}");
            assert_eq!(es_palindromo(&s), es_palindromo(&invertir(&s)));
        }
    }

    // Helper con #[track_caller]: si falla, el error señala la línea del TEST
    // que lo llamó, no la del helper (como t.Helper() en Go).
    #[track_caller]
    fn assert_palabras(texto: &str, esperado: usize) {
        assert_eq!(palabras_distintas(texto), esperado, "texto: {texto:?}");
    }

    #[test]
    fn palabras_distintas_casos() {
        assert_palabras("hola hola HOLA", 1);
        assert_palabras("a b  c b a", 3);
        assert_palabras("", 0);
    }
}
