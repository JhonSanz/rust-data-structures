// =============================================================================
// RETOS 05 — STRUCTS Y ENUMS                     (repasa: 05_structs_y_enums)
// Comprueba tu solución:   cargo test -p retos --test r05
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.

// -----------------------------------------------------------------------------
/// RETO 1 — Métodos de un struct
#[derive(Debug, Clone, PartialEq)]
struct Rectangulo {
    ancho: f64,
    alto: f64,
}

impl Rectangulo {
    fn new(ancho: f64, alto: f64) -> Self {
        Self { ancho, alto }
    }
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }
    fn perimetro(&self) -> f64 {
        2.0 * (self.ancho + self.alto)
    }
    fn es_cuadrado(&self) -> bool {
        self.ancho == self.alto
    }
    /// Multiplica ambas dimensiones por `factor` (modifica el rectángulo).
    fn escalar(&mut self, factor: f64) {
        self.ancho *= factor;
        self.alto *= factor;
    }
    /// ¿Cabe `self` dentro de `otro` sin girarlo?
    fn cabe_en(&self, otro: &Rectangulo) -> bool {
        self.ancho <= otro.ancho && self.alto <= otro.alto
    }
}

#[test]
fn reto_1_rectangulo() {
    let mut r = Rectangulo::new(2.0, 3.0);
    assert_eq!(
        r,
        Rectangulo {
            ancho: 2.0,
            alto: 3.0
        }
    );
    assert_eq!(
        (r.area(), r.perimetro(), r.es_cuadrado()),
        (6.0, 10.0, false)
    );
    let grande = Rectangulo::new(5.0, 5.0);
    assert!(grande.es_cuadrado());
    assert!(r.cabe_en(&grande));
    r.escalar(2.0);
    assert_eq!(r, Rectangulo::new(4.0, 6.0));
    assert!(!r.cabe_en(&grande));
}

// -----------------------------------------------------------------------------
/// RETO 2 — Métodos que consumen `self` en un enum
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direccion {
    Norte,
    Este,
    Sur,
    Oeste,
}

impl Direccion {
    /// Gira 90° en el sentido de las agujas del reloj.
    fn derecha(self) -> Self {
        match self {
            Direccion::Norte => Direccion::Este,
            Direccion::Este => Direccion::Sur,
            Direccion::Sur => Direccion::Oeste,
            Direccion::Oeste => Direccion::Norte,
        }
    }
    /// Gira 90° en sentido contrario. Pista: puedes reutilizar `derecha`.
    fn izquierda(self) -> Self {
        self.derecha().derecha().derecha()
    }
}

#[test]
fn reto_2_direccion() {
    use Direccion::*;
    assert_eq!(Norte.derecha(), Este);
    assert_eq!(Oeste.derecha(), Norte);
    assert_eq!(Norte.izquierda(), Oeste);
    assert_eq!(Sur.izquierda(), Este);
    assert_eq!(Este.derecha().derecha().derecha().derecha(), Este);
}

// -----------------------------------------------------------------------------
/// RETO 3 — Un robot que obedece comandos (enum con datos + match)
/// El robot empieza en (0, 0) mirando al Norte. Norte suma 1 a `y`, Este suma
/// 1 a `x`, Sur resta a `y` y Oeste resta a `x`.
#[derive(Debug)]
enum Comando {
    Avanzar(i32),
    GirarDerecha,
    GirarIzquierda,
}

#[derive(Debug, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    dir: Direccion,
}

impl Robot {
    fn new() -> Self {
        Robot {
            x: 0,
            y: 0,
            dir: Direccion::Norte,
        }
    }
    fn ejecutar(&mut self, comando: &Comando) {
        match comando {
            Comando::GirarDerecha => self.dir = self.dir.derecha(),
            Comando::GirarIzquierda => self.dir = self.dir.izquierda(),
            Comando::Avanzar(n) => match self.dir {
                Direccion::Norte => self.y += n,
                Direccion::Este => self.x += n,
                Direccion::Sur => self.y -= n,
                Direccion::Oeste => self.x -= n,
            },
        }
    }
    fn ejecutar_todos(&mut self, comandos: &[Comando]) {
        for c in comandos {
            self.ejecutar(c);
        }
    }
}

#[test]
fn reto_3_robot() {
    use Comando::*;
    let mut robot = Robot::new();
    assert_eq!(
        robot,
        Robot {
            x: 0,
            y: 0,
            dir: Direccion::Norte
        }
    );
    robot.ejecutar_todos(&[Avanzar(3), GirarDerecha, Avanzar(2)]);
    assert_eq!(
        robot,
        Robot {
            x: 2,
            y: 3,
            dir: Direccion::Este
        }
    );
    robot.ejecutar_todos(&[
        GirarIzquierda,
        GirarIzquierda,
        Avanzar(5),
        GirarIzquierda,
        Avanzar(-1),
    ]);
    assert_eq!(
        robot,
        Robot {
            x: -3,
            y: 4,
            dir: Direccion::Sur
        }
    );
}

// -----------------------------------------------------------------------------
/// RETO 4 — Encadenar Option
/// Devuelve el dominio del email del usuario con ese id (lo que va tras la '@').
/// Devuelve None si el usuario no existe, no tiene email o el email no tiene '@'.
/// Requisito: sin `unwrap`. Pista: `iter().find`, `as_deref`, `and_then`, `split_once`.
struct Usuario {
    id: u32,
    email: Option<String>,
}

fn dominio_email(usuarios: &[Usuario], id: u32) -> Option<String> {
    usuarios
        .iter()
        .find(|u| u.id == id)
        .and_then(|u| u.email.as_deref())
        .and_then(|e| e.split_once('@'))
        .map(|(_, dominio)| dominio.to_string())
}

#[test]
fn reto_4_dominio_email() {
    let usuarios = [
        Usuario {
            id: 1,
            email: Some("ana@ejemplo.com".into()),
        },
        Usuario { id: 2, email: None },
        Usuario {
            id: 3,
            email: Some("sin-arroba".into()),
        },
    ];
    assert_eq!(dominio_email(&usuarios, 1), Some("ejemplo.com".to_string()));
    assert_eq!(dominio_email(&usuarios, 2), None);
    assert_eq!(dominio_email(&usuarios, 3), None);
    assert_eq!(dominio_email(&usuarios, 99), None);
}

// -----------------------------------------------------------------------------
/// RETO 5 — Máquina de estados
/// Rojo → Verde → Amarillo → Rojo...  Duraciones: Rojo 30 s, Verde 25 s, Amarillo 5 s.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Semaforo {
    Rojo,
    Verde,
    Amarillo,
}

impl Semaforo {
    fn siguiente(self) -> Self {
        match self {
            Semaforo::Rojo => Semaforo::Verde,
            Semaforo::Verde => Semaforo::Amarillo,
            Semaforo::Amarillo => Semaforo::Rojo,
        }
    }
    fn duracion_s(&self) -> u32 {
        match self {
            Semaforo::Rojo => 30,
            Semaforo::Verde => 25,
            Semaforo::Amarillo => 5,
        }
    }
    /// ¿En qué estado estará tras `segundos` segundos, empezando al principio de `self`?
    /// Ejemplo: Rojo tras 30 s → Verde; Rojo tras 29 s → Rojo; Rojo tras 60 s → Rojo.
    fn tras(self, segundos: u32) -> Self {
        let mut estado = self;
        let mut restante = segundos % 60; // un ciclo completo dura 60 s
        while restante >= estado.duracion_s() {
            restante -= estado.duracion_s();
            estado = estado.siguiente();
        }
        estado
    }
}

#[test]
fn reto_5_semaforo() {
    use Semaforo::*;
    assert_eq!(Rojo.siguiente(), Verde);
    assert_eq!(Amarillo.siguiente(), Rojo);
    assert_eq!(Verde.duracion_s() + Amarillo.duracion_s(), 30);
    assert_eq!(Rojo.tras(29), Rojo);
    assert_eq!(Rojo.tras(30), Verde);
    assert_eq!(Rojo.tras(55), Amarillo);
    assert_eq!(Rojo.tras(60), Rojo);
    assert_eq!(Verde.tras(3600 + 26), Amarillo);
}
