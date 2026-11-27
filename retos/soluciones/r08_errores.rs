// =============================================================================
// RETOS 08 — MANEJO DE ERRORES                   (repasa: 08_errores)
// Comprueba tu solución:   cargo test -p retos --test r08
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.
// Regla del módulo: prohibido `unwrap()` y `expect()` en tus soluciones.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
enum ErrorEdad {
    NoEsNumero(String),
    FueraDeRango(i64),
}

/// RETO 1 — Devolver errores propios
/// Quita los espacios de los extremos y convierte el texto en una edad (0..=150).
///   "42" → Ok(42)   " 7 " → Ok(7)   "abc" → Err(NoEsNumero("abc"))   "200" → Err(FueraDeRango(200))
fn parsear_edad(texto: &str) -> Result<u8, ErrorEdad> {
    let texto = texto.trim();
    let n: i64 = texto
        .parse()
        .map_err(|_| ErrorEdad::NoEsNumero(texto.to_string()))?;
    if (0..=150).contains(&n) {
        Ok(n as u8)
    } else {
        Err(ErrorEdad::FueraDeRango(n))
    }
}

#[test]
fn reto_1_parsear_edad() {
    assert_eq!(parsear_edad("42"), Ok(42));
    assert_eq!(parsear_edad(" 7 "), Ok(7));
    assert_eq!(
        parsear_edad("abc"),
        Err(ErrorEdad::NoEsNumero("abc".into()))
    );
    assert_eq!(parsear_edad("200"), Err(ErrorEdad::FueraDeRango(200)));
    assert_eq!(parsear_edad("-1"), Err(ErrorEdad::FueraDeRango(-1)));
}

/// RETO 2 — Mensajes de error para humanos
///   NoEsNumero("abc")  → "'abc' no es un número"
///   FueraDeRango(200)  → "200 está fuera del rango 0-150"
impl fmt::Display for ErrorEdad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorEdad::NoEsNumero(t) => write!(f, "'{t}' no es un número"),
            ErrorEdad::FueraDeRango(n) => write!(f, "{n} está fuera del rango 0-150"),
        }
    }
}

impl Error for ErrorEdad {}

#[test]
fn reto_2_display_error() {
    assert_eq!(
        ErrorEdad::NoEsNumero("abc".into()).to_string(),
        "'abc' no es un número"
    );
    assert_eq!(
        ErrorEdad::FueraDeRango(200).to_string(),
        "200 está fuera del rango 0-150"
    );
}

/// RETO 3 — Propagar con ?
/// Suma todos los números del slice. Si alguno no es un número, devuelve el
/// error del PRIMERO que falle.
/// Requisito: usa `?` dentro de un bucle.
fn sumar_textos(textos: &[&str]) -> Result<i64, ParseIntError> {
    let mut total = 0;
    for t in textos {
        total += t.parse::<i64>()?;
    }
    Ok(total)
}

#[test]
fn reto_3_sumar_textos() {
    assert_eq!(sumar_textos(&["1", "2", "39"]), Ok(42));
    assert_eq!(sumar_textos(&[]), Ok(0));
    assert_eq!(
        sumar_textos(&["1", "x", ""]),
        Err("x".parse::<i64>().unwrap_err())
    );
}

// -----------------------------------------------------------------------------
/// RETO 4 — Convertir errores con From para poder usar ?
/// Lee una configuración con líneas "clave=valor". Ignora líneas vacías y las
/// que empiezan por '#'. Necesita `host` y `puerto` (u16). Los espacios
/// alrededor de claves y valores se ignoran.
///   - Si falta una clave → Err(FaltaClave("host")) o Err(FaltaClave("puerto"))
///   - Si el puerto no es un u16 → Err(PuertoInvalido(error_original))
/// Requisito: implementa From<ParseIntError> para ErrorConfig y usa `?` al
/// parsear el puerto (sin map_err).
#[derive(Debug, PartialEq)]
enum ErrorConfig {
    FaltaClave(&'static str),
    PuertoInvalido(ParseIntError),
}

impl From<ParseIntError> for ErrorConfig {
    fn from(e: ParseIntError) -> Self {
        ErrorConfig::PuertoInvalido(e)
    }
}

#[derive(Debug, PartialEq)]
struct Config {
    host: String,
    puerto: u16,
}

fn leer_config(texto: &str) -> Result<Config, ErrorConfig> {
    let mut host = None;
    let mut puerto = None;
    for linea in texto.lines().map(str::trim) {
        if linea.is_empty() || linea.starts_with('#') {
            continue;
        }
        if let Some((clave, valor)) = linea.split_once('=') {
            match clave.trim() {
                "host" => host = Some(valor.trim().to_string()),
                "puerto" => puerto = Some(valor.trim().parse::<u16>()?),
                _ => {}
            }
        }
    }
    Ok(Config {
        host: host.ok_or(ErrorConfig::FaltaClave("host"))?,
        puerto: puerto.ok_or(ErrorConfig::FaltaClave("puerto"))?,
    })
}

#[test]
fn reto_4_leer_config() {
    let ok = "# servidor\nhost = localhost\n\npuerto=8080\nmodo=dev";
    assert_eq!(
        leer_config(ok),
        Ok(Config {
            host: "localhost".into(),
            puerto: 8080
        })
    );
    assert_eq!(
        leer_config("puerto=1"),
        Err(ErrorConfig::FaltaClave("host"))
    );
    assert_eq!(
        leer_config("host=x"),
        Err(ErrorConfig::FaltaClave("puerto"))
    );
    let err = "99999".parse::<u16>().unwrap_err();
    assert_eq!(
        leer_config("host=x\npuerto=99999"),
        Err(ErrorConfig::PuertoInvalido(err))
    );
}

/// RETO 5 — Box<dyn Error>: cualquier error
/// Devuelve la media de los números (f64) del slice.
///   - si alguno no es un número, devuelve ese error (con ?)
///   - si el slice está vacío, devuelve un error con el mensaje "lista vacía"
/// Pista: un &str se convierte en Box<dyn Error> con `.into()`.
fn media(textos: &[&str]) -> Result<f64, Box<dyn Error>> {
    if textos.is_empty() {
        return Err("lista vacía".into());
    }
    let mut suma = 0.0;
    for t in textos {
        suma += t.trim().parse::<f64>()?;
    }
    Ok(suma / textos.len() as f64)
}

#[test]
fn reto_5_media() {
    assert_eq!(media(&["1", "2", " 6 "]).ok(), Some(3.0));
    assert_eq!(media(&[]).unwrap_err().to_string(), "lista vacía");
    assert!(
        media(&["1", "dos"])
            .unwrap_err()
            .to_string()
            .contains("invalid float")
    );
}

/// RETO 6 — collect sobre Result
/// Divide cada par (a, b). Si algún b es 0, devuelve Err("división por cero en la posición {i}").
/// Requisito: sin bucles; usa `iter().enumerate().map(...).collect()`.
fn dividir_todos(pares: &[(i32, i32)]) -> Result<Vec<i32>, String> {
    pares
        .iter()
        .enumerate()
        .map(|(i, &(a, b))| {
            a.checked_div(b)
                .ok_or(format!("división por cero en la posición {i}"))
        })
        .collect()
}

#[test]
fn reto_6_dividir_todos() {
    assert_eq!(dividir_todos(&[(10, 2), (9, 3)]), Ok(vec![5, 3]));
    assert_eq!(
        dividir_todos(&[(1, 1), (5, 0), (2, 0)]),
        Err("división por cero en la posición 1".into())
    );
}
