// =============================================================================
// RETOS 11 — SMART POINTERS                      (repasa: 11_smart_pointers)
// Comprueba tu solución:   cargo test -p retos --test r11
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

// -----------------------------------------------------------------------------
/// RETO 1 — Una lista enlazada con Box (tipo recursivo)
#[derive(Debug, PartialEq)]
enum Lista {
    Nodo(i32, Box<Lista>),
    Vacia,
}

impl Lista {
    /// Construye la lista con los elementos en el mismo orden.
    /// Pista: recursión con `split_first`, o recorrer el slice al revés.
    fn desde_slice(v: &[i32]) -> Lista {
        todo!()
    }
    fn suma(&self) -> i32 {
        todo!()
    }
    fn longitud(&self) -> usize {
        todo!()
    }
}

#[test]
fn reto_1_lista() {
    use Lista::*;
    let l = Lista::desde_slice(&[1, 2, 3]);
    assert_eq!(
        l,
        Nodo(1, Box::new(Nodo(2, Box::new(Nodo(3, Box::new(Vacia))))))
    );
    assert_eq!((l.suma(), l.longitud()), (6, 3));
    assert_eq!(Lista::desde_slice(&[]), Vacia);
}

/// RETO 2 — Rc: varios dueños de los mismos datos
/// Devuelve `n` Rc que apuntan TODOS a los mismos datos (sin copiarlos).
fn compartir(datos: Vec<String>, n: usize) -> Vec<Rc<Vec<String>>> {
    todo!()
}

#[test]
fn reto_2_compartir() {
    let copias = compartir(vec!["a".into(), "b".into()], 4);
    assert_eq!(copias.len(), 4);
    assert_eq!(Rc::strong_count(&copias[0]), 4);
    assert!(copias.iter().all(|c| Rc::ptr_eq(c, &copias[0])));
    assert_eq!(copias[3][1], "b");
}

/// RETO 3 — RefCell: mutar a través de &self
/// Un registro que acepta mensajes aunque solo tengas una referencia & a él.
struct Registro {
    mensajes: RefCell<Vec<String>>,
}

impl Registro {
    fn new() -> Self {
        todo!()
    }
    fn registrar(&self, mensaje: &str) {
        todo!()
    }
    fn total(&self) -> usize {
        todo!()
    }
    /// Los `n` últimos mensajes, del más antiguo al más reciente.
    fn ultimos(&self, n: usize) -> Vec<String> {
        todo!()
    }
}

#[test]
fn reto_3_registro() {
    let registro = Registro::new();
    let r1 = &registro;
    let r2 = &registro;
    r1.registrar("inicio");
    r2.registrar("procesando");
    r1.registrar("fin");
    assert_eq!(registro.total(), 3);
    assert_eq!(registro.ultimos(2), ["procesando", "fin"]);
    assert_eq!(registro.ultimos(10).len(), 3);
}

/// RETO 4 — Rc<RefCell<T>>: estado compartido y mutable
type Cuenta = Rc<RefCell<i64>>;

fn nueva_cuenta(saldo: i64) -> Cuenta {
    todo!()
}

/// Retira `cantidad` si hay saldo suficiente; si no, Err("saldo insuficiente").
fn retirar(cuenta: &Cuenta, cantidad: i64) -> Result<(), String> {
    todo!()
}

#[test]
fn reto_4_cuenta_compartida() {
    let cuenta = nueva_cuenta(100);
    let titular = Rc::clone(&cuenta);
    let tarjeta = Rc::clone(&cuenta);
    assert_eq!(retirar(&titular, 30), Ok(()));
    assert_eq!(retirar(&tarjeta, 50), Ok(()));
    assert_eq!(retirar(&tarjeta, 50), Err("saldo insuficiente".into()));
    assert_eq!(*cuenta.borrow(), 20);
}

/// RETO 5 — Cow: copiar solo si hace falta
/// Sustituye & por &amp;, < por &lt; y > por &gt;.
/// Si el texto no contiene ninguno, devuelve el original PRESTADO (sin reservar memoria).
fn escapar_html(texto: &str) -> Cow<'_, str> {
    todo!()
}

#[test]
fn reto_5_escapar_html() {
    assert!(matches!(
        escapar_html("texto normal"),
        Cow::Borrowed("texto normal")
    ));
    let r = escapar_html("<b>Tom & Jerry</b>");
    assert!(matches!(r, Cow::Owned(_)));
    assert_eq!(r, "&lt;b&gt;Tom &amp; Jerry&lt;/b&gt;");
}
