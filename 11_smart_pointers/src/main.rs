// =============================================================================
// MÓDULO 11 — SMART POINTERS
// Ejecuta con:  cargo run -p smart_pointers
// =============================================================================
//
// Un "smart pointer" es un struct que se comporta como una referencia (implementa
// Deref) y además gestiona algo (memoria, conteo, préstamos...). Al morir, su
// Drop hace la limpieza. Ya conoces dos: String y Vec<T>.
//
// | Tipo         | Para qué                                             | ¿Hilos? |
// |--------------|------------------------------------------------------|---------|
// | Box<T>       | Un dueño, valor en el heap                           | sí      |
// | Rc<T>        | VARIOS dueños (conteo de referencias)                | no      |
// | Arc<T>       | Varios dueños, conteo atómico                        | sí      |
// | Cell<T>      | Mutabilidad interior para tipos Copy                 | no      |
// | RefCell<T>   | Mutabilidad interior, préstamos comprobados en RUNTIME| no     |
// | Mutex/RwLock | Mutabilidad interior entre hilos (módulo 12)         | sí      |
// | Weak<T>      | Referencia que no mantiene vivo el valor (rompe ciclos)| —     |
// | Cow<T>       | "Clone on write": prestado hasta que hay que modificar | —     |

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::{Rc, Weak};

fn main() {
    println!("===== 1. Box<T> =====");
    boxes();

    println!("\n===== 2. Deref: comportarse como una referencia =====");
    deref();

    println!("\n===== 3. Rc<T>: varios dueños =====");
    rc();

    println!("\n===== 4. Cell y RefCell: mutabilidad interior =====");
    mutabilidad_interior();

    println!("\n===== 5. Rc<RefCell<T>>: estado compartido y mutable =====");
    compartido_y_mutable();

    println!("\n===== 6. Weak<T>: árbol con referencia al padre =====");
    arbol_con_weak();

    println!("\n===== 7. Cow: copiar solo si hace falta =====");
    for texto in ["sin espacios", "con  dos  espacios", "limpio"] {
        let r = normalizar(texto);
        let tipo = if matches!(r, Cow::Borrowed(_)) {
            "prestado"
        } else {
            "nuevo"
        };
        println!("{texto:?} → {r:?} ({tipo})");
    }
}

// Lista enlazada: un tipo recursivo necesita tamaño conocido → Box
#[derive(Debug)]
enum Lista {
    Nodo(i32, Box<Lista>),
    Fin,
}
use Lista::{Fin, Nodo};

impl Lista {
    fn suma(&self) -> i32 {
        match self {
            Nodo(v, resto) => v + resto.suma(),
            Fin => 0,
        }
    }
}

fn boxes() {
    let b = Box::new(5);
    println!("b = {b}, *b + 1 = {}", *b + 1);

    let lista = Nodo(1, Box::new(Nodo(2, Box::new(Nodo(3, Box::new(Fin))))));
    println!("lista = {lista:?}");
    println!("suma = {}", lista.suma());

    // Box<dyn Trait>: objetos de tipos distintos tras una misma interfaz (módulo 07)
    let operaciones: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(|x| x + 1), Box::new(|x| x * 10)];
    println!(
        "operaciones sobre 4: {:?}",
        operaciones.iter().map(|f| f(4)).collect::<Vec<_>>()
    );
}

// Un smart pointer casero: implementar Deref permite usar `*` y llamar a los
// métodos del tipo interior directamente ("deref coercion").
struct MiBox<T>(T);

impl<T> Deref for MiBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn saludar(nombre: &str) {
    println!("  hola, {nombre}");
}

fn deref() {
    let x = MiBox(String::from("Ferris"));
    println!("len a través de MiBox: {}", x.len()); // MiBox → String → str
    // Deref coercion: &MiBox<String> → &String → &str, automáticamente
    saludar(&x);
}

fn rc() {
    // Rc = "Reference counted". Clonar un Rc NO copia los datos: incrementa el contador.
    let compartido = Rc::new(vec![1, 2, 3]);
    println!("contador = {}", Rc::strong_count(&compartido));
    {
        let otro = Rc::clone(&compartido); // convención: Rc::clone para dejar claro que es barato
        println!(
            "contador = {} (otro ve {:?})",
            Rc::strong_count(&compartido),
            otro
        );
    }
    println!("contador = {} (otro murió)", Rc::strong_count(&compartido));
    // Los datos se liberan cuando el contador llega a 0.
    // Rc da acceso SOLO DE LECTURA. Para modificar, combínalo con RefCell.
}

fn mutabilidad_interior() {
    // Cell<T>: get/set de valores Copy a través de una referencia &
    struct Contador {
        visitas: Cell<u32>,
    }
    let c = Contador {
        visitas: Cell::new(0),
    };
    let r1 = &c;
    let r2 = &c;
    r1.visitas.set(r1.visitas.get() + 1); // modificamos a través de una &, ¡sin mut!
    r2.visitas.set(r2.visitas.get() + 1);
    println!("visitas = {}", c.visitas.get());

    // RefCell<T>: las reglas de préstamo (muchas & O una &mut) se comprueban
    // en EJECUCIÓN en vez de en compilación. Si las rompes → panic.
    let cache: RefCell<HashMap<u64, u64>> = RefCell::new(HashMap::new());
    fn fib(n: u64, cache: &RefCell<HashMap<u64, u64>>) -> u64 {
        if n < 2 {
            return n;
        }
        if let Some(&v) = cache.borrow().get(&n) {
            return v;
        }
        let v = fib(n - 1, cache) + fib(n - 2, cache);
        cache.borrow_mut().insert(n, v);
        v
    }
    println!(
        "fib(80) = {} (cache con {} entradas)",
        fib(80, &cache),
        cache.borrow().len()
    );

    // try_borrow_mut devuelve Err en vez de hacer panic
    let celda = RefCell::new(5);
    let lectura = celda.borrow();
    println!(
        "¿try_borrow_mut con una lectura activa? {:?}",
        celda.try_borrow_mut().is_ok()
    );
    drop(lectura);
    *celda.borrow_mut() += 1;
    println!("celda = {}", celda.borrow());
}

fn compartido_y_mutable() {
    // Patrón clásico (de un solo hilo): varios dueños que pueden modificar.
    // Para varios hilos el equivalente es Arc<Mutex<T>> (módulo 12).
    #[derive(Debug)]
    struct Cuenta {
        saldo: i64,
    }
    let cuenta = Rc::new(RefCell::new(Cuenta { saldo: 100 }));
    let titular = Rc::clone(&cuenta);
    let conyuge = Rc::clone(&cuenta);

    titular.borrow_mut().saldo -= 30;
    conyuge.borrow_mut().saldo += 50;
    println!(
        "saldo final = {} (dueños: {})",
        cuenta.borrow().saldo,
        Rc::strong_count(&cuenta)
    );
}

// Con Rc, dos nodos que se apuntan mutuamente formarían un CICLO: el contador
// nunca llegaría a 0 → fuga de memoria. Weak rompe el ciclo: no cuenta como dueño.
#[derive(Debug)]
struct NodoArbol {
    valor: i32,
    padre: RefCell<Weak<NodoArbol>>,
    hijos: RefCell<Vec<Rc<NodoArbol>>>,
}

fn arbol_con_weak() {
    let hoja = Rc::new(NodoArbol {
        valor: 3,
        padre: RefCell::new(Weak::new()),
        hijos: RefCell::new(vec![]),
    });
    println!(
        "padre de la hoja antes: {:?}",
        hoja.padre.borrow().upgrade().map(|p| p.valor)
    );

    let rama = Rc::new(NodoArbol {
        valor: 5,
        padre: RefCell::new(Weak::new()),
        hijos: RefCell::new(vec![Rc::clone(&hoja)]),
    });
    *hoja.padre.borrow_mut() = Rc::downgrade(&rama);

    // upgrade() → Option<Rc<T>>: None si el valor ya se liberó
    println!(
        "padre de la hoja después: {:?}",
        hoja.padre.borrow().upgrade().map(|p| p.valor)
    );
    println!(
        "rama: strong = {}, weak = {}",
        Rc::strong_count(&rama),
        Rc::weak_count(&rama)
    );
    println!(
        "hijos de la rama: {:?}",
        rama.hijos
            .borrow()
            .iter()
            .map(|h| h.valor)
            .collect::<Vec<_>>()
    );
}

// Cow (Clone on Write): devuelve el &str original si no hay que cambiar nada,
// y solo reserva un String nuevo cuando de verdad hace falta.
fn normalizar(texto: &str) -> Cow<'_, str> {
    if texto.contains("  ") {
        Cow::Owned(texto.split_whitespace().collect::<Vec<_>>().join(" "))
    } else {
        Cow::Borrowed(texto)
    }
}
