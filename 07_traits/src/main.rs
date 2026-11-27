// =============================================================================
// MÓDULO 07 — TRAITS
// Ejecuta con:  cargo run -p traits
// =============================================================================
//
// Un trait define comportamiento compartido (como las interfaces de Go), pero:
//   - La implementación es EXPLÍCITA: `impl Trait for Tipo`.
//   - Pueden tener métodos con implementación POR DEFECTO.
//   - Se usan de dos maneras:
//       * estática (genéricos / `impl Trait`): monomorfización, coste cero
//       * dinámica (`dyn Trait`): vtable, permite colecciones heterogéneas
//   - Regla del huérfano: puedes implementar un trait para un tipo solo si el
//     trait O el tipo son de tu crate.

use std::fmt;
use std::ops::{Add, Mul};

// -----------------------------------------------------------------------------
// 1. Definir e implementar un trait
// -----------------------------------------------------------------------------
trait Forma {
    // Métodos obligatorios
    fn area(&self) -> f64;
    fn nombre(&self) -> String;

    // Método con implementación por defecto (se puede sobrescribir)
    fn describir(&self) -> String {
        format!("{} de área {:.2}", self.nombre(), self.area())
    }
}

struct Circulo {
    radio: f64,
}
struct Rectangulo {
    ancho: f64,
    alto: f64,
}

impl Forma for Circulo {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radio * self.radio
    }
    fn nombre(&self) -> String {
        "círculo".into()
    }
}

impl Forma for Rectangulo {
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }
    fn nombre(&self) -> String {
        "rectángulo".into()
    }
    // sobrescribimos el método por defecto
    fn describir(&self) -> String {
        format!("rectángulo {}×{}", self.ancho, self.alto)
    }
}

// -----------------------------------------------------------------------------
// 2. Despacho estático vs dinámico
// -----------------------------------------------------------------------------
// Estático: el compilador genera una versión de la función por cada tipo.
fn imprimir_estatico(f: &impl Forma) {
    println!("  [estático] {}", f.describir());
}

// Dinámico: un único código; la llamada se resuelve en ejecución vía vtable.
fn imprimir_dinamico(f: &dyn Forma) {
    println!("  [dinámico] {}", f.describir());
}

fn area_total(formas: &[Box<dyn Forma>]) -> f64 {
    formas.iter().map(|f| f.area()).sum()
}

// -----------------------------------------------------------------------------
// 3. Traits de la biblioteca estándar
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Vector2 {
    x: f64,
    y: f64,
}

// Display → {}
impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Sobrecarga de operadores: implementando los traits de std::ops
impl Add for Vector2 {
    type Output = Vector2; // "tipo asociado": parte del contrato del trait
    fn add(self, otro: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + otro.x,
            y: self.y + otro.y,
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;
    fn mul(self, k: f64) -> Vector2 {
        Vector2 {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

// From → conversión infalible. Implementar From te da Into gratis.
impl From<(f64, f64)> for Vector2 {
    fn from((x, y): (f64, f64)) -> Self {
        Vector2 { x, y }
    }
}

// Orden: PartialEq/Eq (==), PartialOrd/Ord (<, sort), Hash (claves de HashMap)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Version {
    mayor: u32,
    menor: u32,
    parche: u32,
}

// -----------------------------------------------------------------------------
// 4. Traits genéricos sobre cualquier tipo, y supertraits
// -----------------------------------------------------------------------------
// Un supertrait: para implementar `Resumible` hace falta ser `Display`.
trait Resumible: fmt::Display {
    fn resumen(&self) -> String {
        let texto = self.to_string(); // disponible gracias a Display
        if texto.chars().count() > 12 {
            format!("{}…", texto.chars().take(12).collect::<String>())
        } else {
            texto
        }
    }
}

impl Resumible for Vector2 {}
impl Resumible for String {} // podemos: el trait es nuestro

// Implementación "blanket": para TODOS los T que cumplan una condición.
trait Gritar {
    fn gritar(&self) -> String;
}
impl<T: fmt::Display> Gritar for T {
    fn gritar(&self) -> String {
        format!("¡{}!", self.to_string().to_uppercase())
    }
}

// -----------------------------------------------------------------------------
// 5. Downcasting con Any (el "type switch" de Rust, raramente necesario)
// -----------------------------------------------------------------------------
fn que_es(valor: &dyn std::any::Any) -> &'static str {
    if valor.is::<i32>() {
        "un i32"
    } else if valor.is::<String>() {
        "un String"
    } else if let Some(v) = valor.downcast_ref::<Vector2>() {
        if v.x == 0.0 {
            "un Vector2 en el eje y"
        } else {
            "un Vector2"
        }
    } else {
        "algo desconocido"
    }
}

fn main() {
    println!("===== 1. Traits propios =====");
    let c = Circulo { radio: 1.5 };
    let r = Rectangulo {
        ancho: 2.0,
        alto: 3.0,
    };
    println!("{}", c.describir());
    println!("{}", r.describir());

    println!("\n===== 2. Estático vs dinámico =====");
    imprimir_estatico(&c);
    imprimir_dinamico(&r);
    // Una colección heterogénea necesita dyn (y Box, porque cada tipo mide distinto)
    let formas: Vec<Box<dyn Forma>> = vec![
        Box::new(Circulo { radio: 1.0 }),
        Box::new(Rectangulo {
            ancho: 4.0,
            alto: 5.0,
        }),
    ];
    println!("área total = {:.2}", area_total(&formas));

    println!("\n===== 3. Traits de std: Display, operadores, From =====");
    let a = Vector2 { x: 1.0, y: 2.0 };
    let b: Vector2 = (3.0, 4.0).into(); // gracias a From<(f64, f64)>
    println!("a = {a}, b = {b}, a + b = {}, a * 3 = {}", a + b, a * 3.0);
    println!("Default: {:?}, ¿a == b? {}", Vector2::default(), a == b);

    let mut versiones = [
        Version {
            mayor: 1,
            menor: 10,
            parche: 0,
        },
        Version {
            mayor: 1,
            menor: 2,
            parche: 3,
        },
        Version {
            mayor: 0,
            menor: 9,
            parche: 9,
        },
    ];
    versiones.sort(); // gracias a Ord (derivado: compara campo a campo en orden)
    let texto: Vec<String> = versiones
        .iter()
        .map(|v| format!("{}.{}.{}", v.mayor, v.menor, v.parche))
        .collect();
    println!("versiones ordenadas: {texto:?}");
    println!("max: {:?}", versiones.iter().max());

    println!("\n===== 4. Supertraits y blanket impls =====");
    println!("resumen: {}", a.resumen());
    println!(
        "resumen: {}",
        String::from("un texto bastante largo").resumen()
    );
    println!("gritar: {} {} {}", "hola".gritar(), 42.gritar(), b.gritar());

    println!("\n===== 5. Any (downcasting) =====");
    println!("5 es {}", que_es(&5_i32));
    println!("\"x\" es {}", que_es(&String::from("x")));
    println!("Vector2 es {}", que_es(&Vector2 { x: 0.0, y: 1.0 }));
    println!("2.5 es {}", que_es(&2.5_f64));

    println!("\n===== 6. Escribir en cualquier destino: io::Write =====");
    escribir_en_destinos();
}

// Igual que io.Writer en Go: una función que acepta "cualquier cosa escribible".
fn escribir_informe(w: &mut impl std::io::Write) -> std::io::Result<()> {
    writeln!(w, "informe: todo correcto")?;
    writeln!(w, "líneas: 2")
}

fn escribir_en_destinos() {
    // A stdout
    escribir_informe(&mut std::io::stdout()).unwrap();
    // A un buffer en memoria (Vec<u8> implementa Write)
    let mut buffer: Vec<u8> = Vec::new();
    escribir_informe(&mut buffer).unwrap();
    println!(
        "en buffer: {} bytes → {:?}",
        buffer.len(),
        String::from_utf8_lossy(&buffer)
    );
}
