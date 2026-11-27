// =============================================================================
// RETOS 07 — TRAITS                              (repasa: 07_traits)
// Comprueba tu solución:   cargo test -p retos --test r07
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.

use std::fmt;
use std::ops::{Add, Mul};

// -----------------------------------------------------------------------------
/// RETO 1 — Implementar un trait y un método por defecto
/// - Perro:  "Perro llamado {nombre}"
/// - Coche:  "Coche {marca} {modelo}"
/// - gritar (por defecto, para todos): describir() en MAYÚSCULAS seguido de "!"
trait Describir {
    fn describir(&self) -> String;
    fn gritar(&self) -> String {
        format!("{}!", self.describir().to_uppercase())
    }
}

struct Perro {
    nombre: String,
}
struct Coche {
    marca: String,
    modelo: String,
}

impl Describir for Perro {
    fn describir(&self) -> String {
        format!("Perro llamado {}", self.nombre)
    }
}

impl Describir for Coche {
    fn describir(&self) -> String {
        format!("Coche {} {}", self.marca, self.modelo)
    }
}

#[test]
fn reto_1_describir() {
    let p = Perro {
        nombre: "Toby".into(),
    };
    let c = Coche {
        marca: "Seat".into(),
        modelo: "Ibiza".into(),
    };
    assert_eq!(p.describir(), "Perro llamado Toby");
    assert_eq!(c.describir(), "Coche Seat Ibiza");
    assert_eq!(p.gritar(), "PERRO LLAMADO TOBY!");
    // Una lista heterogénea gracias a dyn
    let cosas: Vec<Box<dyn Describir>> = vec![Box::new(p), Box::new(c)];
    assert_eq!(cosas.iter().map(|x| x.describir().len()).sum::<usize>(), 34);
}

// -----------------------------------------------------------------------------
/// RETOS 2 a 5 — Un tipo numérico propio: fracciones
#[derive(Debug, Clone, Copy, PartialEq)]
struct Fraccion {
    num: i64,
    den: i64,
}

/// Máximo común divisor (siempre ≥ 0). Algoritmo de Euclides.
fn mcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { mcd(b, a % b) }
}

impl Fraccion {
    /// RETO 2 — Crea la fracción SIMPLIFICADA y con el signo en el numerador.
    /// new(2, 4) → 1/2   new(3, -6) → -1/2   new(0, 5) → 0/1
    /// Hace panic con el mensaje "denominador cero" si den == 0.
    fn new(num: i64, den: i64) -> Self {
        assert!(den != 0, "denominador cero");
        let d = mcd(num, den) * den.signum();
        Fraccion {
            num: num / d,
            den: den / d,
        }
    }
}

#[test]
fn reto_2_fraccion_new() {
    assert_eq!(mcd(12, 18), 6);
    assert_eq!(Fraccion::new(2, 4), Fraccion { num: 1, den: 2 });
    assert_eq!(Fraccion::new(3, -6), Fraccion { num: -1, den: 2 });
    assert_eq!(Fraccion::new(-3, -6), Fraccion { num: 1, den: 2 });
    assert_eq!(Fraccion::new(0, 5), Fraccion { num: 0, den: 1 });
    let r = std::panic::catch_unwind(|| Fraccion::new(1, 0));
    let payload = r.expect_err("new(1, 0) debería hacer panic");
    let msg = payload
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_default();
    assert!(
        msg.contains("denominador cero"),
        "mensaje de panic: {msg:?}"
    );
}

/// RETO 3 — Display: "3/4", o solo "3" si el denominador es 1.
impl fmt::Display for Fraccion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

#[test]
fn reto_3_fraccion_display() {
    assert_eq!(Fraccion::new(3, 4).to_string(), "3/4");
    assert_eq!(Fraccion::new(-6, 3).to_string(), "-2");
    assert_eq!(format!("[{}]", Fraccion::new(1, -3)), "[-1/3]");
}

/// RETO 4 — Operadores + y *  (el resultado siempre simplificado)
impl Add for Fraccion {
    type Output = Fraccion;
    fn add(self, otra: Fraccion) -> Fraccion {
        Fraccion::new(
            self.num * otra.den + otra.num * self.den,
            self.den * otra.den,
        )
    }
}

impl Mul for Fraccion {
    type Output = Fraccion;
    fn mul(self, otra: Fraccion) -> Fraccion {
        Fraccion::new(self.num * otra.num, self.den * otra.den)
    }
}

#[test]
fn reto_4_fraccion_operadores() {
    let medio = Fraccion::new(1, 2);
    let tercio = Fraccion::new(1, 3);
    assert_eq!(medio + tercio, Fraccion::new(5, 6));
    assert_eq!(medio + medio, Fraccion::new(1, 1));
    assert_eq!(medio * tercio, Fraccion::new(1, 6));
    assert_eq!((tercio + tercio + tercio).to_string(), "1");
}

/// RETO 5 — From: un entero es una fracción con denominador 1
impl From<i64> for Fraccion {
    fn from(n: i64) -> Self {
        Fraccion::new(n, 1)
    }
}

#[test]
fn reto_5_fraccion_from() {
    let tres: Fraccion = 3.into();
    assert_eq!(tres, Fraccion { num: 3, den: 1 });
    assert_eq!(Fraccion::from(2) * Fraccion::new(1, 4), Fraccion::new(1, 2));
}

// -----------------------------------------------------------------------------
/// RETO 6 — Despacho dinámico
trait Forma {
    fn area(&self) -> f64;
    fn nombre(&self) -> String;
}

struct Circulo {
    radio: f64,
}
struct Cuadrado {
    lado: f64,
}

impl Forma for Circulo {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radio * self.radio
    }
    fn nombre(&self) -> String {
        format!("círculo de radio {}", self.radio)
    }
}

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.lado * self.lado
    }
    fn nombre(&self) -> String {
        format!("cuadrado de lado {}", self.lado)
    }
}

/// Devuelve el nombre de la forma con mayor área (None si no hay formas).
/// Pista: `max_by` con `f64::total_cmp`.
fn la_mayor(formas: &[Box<dyn Forma>]) -> Option<String> {
    formas
        .iter()
        .max_by(|a, b| a.area().total_cmp(&b.area()))
        .map(|f| f.nombre())
}

#[test]
fn reto_6_formas() {
    let formas: Vec<Box<dyn Forma>> = vec![
        Box::new(Cuadrado { lado: 2.0 }),
        Box::new(Circulo { radio: 1.5 }),
        Box::new(Cuadrado { lado: 1.0 }),
    ];
    let total: f64 = formas.iter().map(|f| f.area()).sum();
    assert!((total - (5.0 + std::f64::consts::PI * 2.25)).abs() < 1e-9);
    assert_eq!(la_mayor(&formas), Some("círculo de radio 1.5".to_string()));
    assert_eq!(la_mayor(&[]), None);
}
