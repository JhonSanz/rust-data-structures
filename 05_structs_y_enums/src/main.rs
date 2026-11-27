// =============================================================================
// MÓDULO 05 — STRUCTS Y ENUMS
// Ejecuta con:  cargo run -p structs_y_enums
// =============================================================================
//
// Rust no tiene clases ni herencia. Tiene:
//   - struct: agrupa datos ("Y": un nombre Y una edad)
//   - enum:   una de varias variantes, cada una con sus datos ("O": círculo O rectángulo)
//   - impl:   bloques donde se definen métodos para un tipo
// El comportamiento compartido se expresa con traits (módulo 07).

use std::fmt;

// -----------------------------------------------------------------------------
// STRUCTS
// -----------------------------------------------------------------------------
// `derive` genera implementaciones automáticas de traits:
//   Debug → {:?}   Clone → .clone()   PartialEq → ==   Default → valores por defecto
#[derive(Debug, Clone, PartialEq, Default)]
struct Usuario {
    nombre: String,
    email: String,
    edad: u32,
    activo: bool,
}

// Struct tupla: campos sin nombre. Útil para "newtypes" que dan significado
// a un tipo primitivo y evitan mezclarlos por error.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Metros(f64);
#[derive(Debug, Clone, Copy)]
struct Color(u8, u8, u8);

// Struct unidad: sin campos (útil como marcador o para implementar traits).
#[derive(Debug)]
struct Marcador;

#[derive(Debug, Clone)]
struct Rectangulo {
    ancho: f64,
    alto: f64,
}

// Los métodos se definen en bloques `impl`.
impl Rectangulo {
    // "Función asociada" (sin self): se llama con Tipo::funcion().
    // `new` es solo una convención, no una palabra reservada.
    fn new(ancho: f64, alto: f64) -> Self {
        Self { ancho, alto } // atajo cuando variable y campo se llaman igual
    }

    fn cuadrado(lado: f64) -> Self {
        Self::new(lado, lado)
    }

    // &self → toma prestado el valor para LEER
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }

    fn cabe_dentro_de(&self, otro: &Rectangulo) -> bool {
        self.ancho <= otro.ancho && self.alto <= otro.alto
    }

    // &mut self → toma prestado para MODIFICAR
    fn escalar(&mut self, factor: f64) {
        self.ancho *= factor;
        self.alto *= factor;
    }

    // self → CONSUME el valor (se mueve dentro del método)
    fn en_tupla(self) -> (f64, f64) {
        (self.ancho, self.alto)
    }
}

// Display: cómo se muestra con {} (Debug es para programadores, Display para usuarios)
impl fmt::Display for Rectangulo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "▭ {}×{}", self.ancho, self.alto)
    }
}

// -----------------------------------------------------------------------------
// ENUMS: tipos suma. Cada variante puede llevar datos distintos.
// -----------------------------------------------------------------------------
#[derive(Debug)]
enum Forma {
    Circulo { radio: f64 },   // variante con campos con nombre
    Rectangulo(Rectangulo),   // variante con un struct dentro
    Triangulo(f64, f64, f64), // variante tupla
    Punto,                    // variante sin datos
}

impl Forma {
    fn area(&self) -> f64 {
        // match exhaustivo: si añades una variante, el compilador te avisa
        // de todos los sitios que tienes que actualizar.
        match self {
            Forma::Circulo { radio } => std::f64::consts::PI * radio * radio,
            Forma::Rectangulo(r) => r.area(),
            Forma::Triangulo(a, b, c) => {
                // Fórmula de Herón
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
            Forma::Punto => 0.0,
        }
    }
}

// Un enum típico para modelar estados: cada estado lleva SOLO los datos que
// tienen sentido en él. Estados imposibles = no representables.
#[derive(Debug)]
enum EstadoPedido {
    Pendiente,
    Enviado {
        transportista: String,
        seguimiento: String,
    },
    Entregado {
        firmado_por: String,
    },
    Cancelado(String), // motivo
}

fn main() {
    println!("===== 1. Crear y usar structs =====");
    structs();

    println!("\n===== 2. Métodos y funciones asociadas =====");
    metodos();

    println!("\n===== 3. Structs tupla y newtypes =====");
    let pasillo = Metros(3.5);
    let salon = Metros(5.0);
    let Color(r, g, b) = Color(255, 128, 0);
    println!(
        "¿pasillo < salón? {} | color = ({r}, {g}, {b}) | {:?}",
        pasillo < salon,
        Marcador
    );

    println!("\n===== 4. Enums con datos =====");
    let formas = [
        Forma::Circulo { radio: 1.0 },
        Forma::Rectangulo(Rectangulo::new(2.0, 3.0)),
        Forma::Triangulo(3.0, 4.0, 5.0),
        Forma::Punto,
    ];
    for f in &formas {
        println!("{f:?} → área {:.2}", f.area());
    }

    println!("\n===== 5. Modelar estados con enums =====");
    let pedidos = [
        EstadoPedido::Pendiente,
        EstadoPedido::Enviado {
            transportista: "Correos".into(),
            seguimiento: "ES123".into(),
        },
        EstadoPedido::Entregado {
            firmado_por: "Ana".into(),
        },
        EstadoPedido::Cancelado("sin stock".into()),
    ];
    for p in &pedidos {
        println!("{}", describir(p));
    }

    println!("\n===== 6. Option<T>: Rust no tiene null =====");
    option();

    println!("\n===== 7. Patrón builder =====");
    builder();
}

fn structs() {
    let mut u1 = Usuario {
        nombre: String::from("Ana"),
        email: String::from("ana@ejemplo.com"),
        edad: 30,
        activo: true,
    };
    // Para modificar un campo, toda la variable debe ser `mut`
    u1.edad += 1;
    println!("{} tiene {} años", u1.nombre, u1.edad);

    // Sintaxis de actualización: el resto de campos se toma de otro valor
    let u2 = Usuario {
        nombre: String::from("Luis"),
        email: String::from("luis@ejemplo.com"),
        ..u1.clone() // ¡ojo! sin .clone(), los String de u1 se moverían
    };
    println!("u2 = {u2:?}");

    // Default: todos los campos con su valor por defecto
    let vacio = Usuario::default();
    println!("default = {vacio:?}");
    println!("¿u1 == u2? {}", u1 == u2);

    // Desestructurar un struct
    let Usuario { nombre, edad, .. } = u2;
    println!("desestructurado: {nombre}, {edad}");

    // Debug bonito
    println!("{u1:#?}");
}

fn metodos() {
    let mut r = Rectangulo::new(3.0, 4.0);
    let c = Rectangulo::cuadrado(5.0);
    println!("{r} área={} | {c} área={}", r.area(), c.area());
    println!("¿r cabe en c? {}", r.cabe_dentro_de(&c));

    // Rust hace auto-referencia: `r.escalar(2.0)` equivale a `Rectangulo::escalar(&mut r, 2.0)`
    r.escalar(2.0);
    println!("escalado: {r}");
    println!("¿ahora cabe? {}", r.cabe_dentro_de(&c));

    let (a, b) = r.en_tupla(); // `r` se consume
    // println!("{r}"); // ERROR: r se movió
    println!("en_tupla: ({a}, {b})");
}

fn describir(estado: &EstadoPedido) -> String {
    match estado {
        EstadoPedido::Pendiente => "⏳ pendiente".to_string(),
        EstadoPedido::Enviado {
            transportista,
            seguimiento,
        } => {
            format!("🚚 enviado con {transportista} ({seguimiento})")
        }
        EstadoPedido::Entregado { firmado_por } => format!("✅ entregado, firmó {firmado_por}"),
        EstadoPedido::Cancelado(motivo) => format!("❌ cancelado: {motivo}"),
    }
}

fn option() {
    // enum Option<T> { Some(T), None } — está en el preludio, no hace falta importarlo.
    // El compilador te OBLIGA a tratar el caso None antes de usar el valor.
    fn buscar(id: u32) -> Option<&'static str> {
        match id {
            1 => Some("Ana"),
            2 => Some("Luis"),
            _ => None,
        }
    }

    for id in [1, 3] {
        match buscar(id) {
            Some(nombre) => println!("id {id}: {nombre}"),
            None => println!("id {id}: no existe"),
        }
    }

    // Métodos útiles de Option
    println!("unwrap_or: {}", buscar(9).unwrap_or("anónimo"));
    println!("map: {:?}", buscar(1).map(|n| n.to_uppercase()));
    println!("is_some: {}", buscar(2).is_some());
    println!("and_then: {:?}", buscar(1).and_then(|n| n.chars().next()));
    // unwrap() hace panic si es None: úsalo solo cuando sea imposible que falle
    // (o en prototipos). expect("mensaje") es igual pero con mensaje propio.
    println!("expect: {}", buscar(1).expect("el id 1 siempre existe"));
}

// Builder: alternativa idiomática a los parámetros opcionales/por defecto.
#[derive(Debug)]
struct Peticion {
    url: String,
    metodo: String,
    timeout_s: u32,
    cabeceras: Vec<(String, String)>,
}

struct PeticionBuilder {
    peticion: Peticion,
}

impl PeticionBuilder {
    fn new(url: &str) -> Self {
        Self {
            peticion: Peticion {
                url: url.to_string(),
                metodo: "GET".into(),
                timeout_s: 30,
                cabeceras: vec![],
            },
        }
    }
    // Cada método toma `self` por valor y lo devuelve: permite encadenar
    fn metodo(mut self, m: &str) -> Self {
        self.peticion.metodo = m.into();
        self
    }
    fn timeout(mut self, s: u32) -> Self {
        self.peticion.timeout_s = s;
        self
    }
    fn cabecera(mut self, k: &str, v: &str) -> Self {
        self.peticion.cabeceras.push((k.into(), v.into()));
        self
    }
    fn build(self) -> Peticion {
        self.peticion
    }
}

fn builder() {
    let p = PeticionBuilder::new("https://api.ejemplo.com")
        .metodo("POST")
        .timeout(5)
        .cabecera("Accept", "application/json")
        .build();
    println!(
        "{} {} (timeout {}s) {:?}",
        p.metodo, p.url, p.timeout_s, p.cabeceras
    );
}
