use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

/*
-----------------------------------------------------------
   0. Variables y punteros básicos
-----------------------------------------------------------

Aqui no hay mucho que comentar, son los punteros de toda la vida

*/
fn pointers_demo() {
    let x = 42; // stack
    let y = &x; // referencia (puntero seguro)
    let z = *y; // desreferencia

    println!("--- Referencias básicas ---");
    println!("x = {}, y = {:p}, z = {}", x, y, z);

    let mut a = 10;
    let b = &mut a; // referencia mutable
    *b += 5; // modificar a través del puntero
    println!("a = {}", a);
}


/***
-----------------------------------------------------------
   1. Box<T>: Dueño único en el heap
-----------------------------------------------------------

¿Qué es Box<T>?
Box es un puntero inteligente muy simple: 
- Mueve un valor al HEAP.
- Mantiene un dueño único en el STACK que libera esa memoria 
  cuando sale de scope.

La gracia de Box es que nos permite trabajar con datos en el 
heap (útil para tamaños dinámicos o estructuras recursivas),
pero manteniendo reglas estrictas de propiedad: **un solo dueño**.

---

Ejemplo:

let bx = Box::new(99);

STACK:       HEAP:
bx ───────▶  99

bx es el único dueño del entero en el heap.

---

Ahora bien:

let y = &*bx;

Aquí ocurre algo importante:
- `*bx` desreferencia el `Box` (llega al valor en el heap).
- `&*bx` toma una **referencia prestada** a ese valor.

STACK:          HEAP:
bx ───────▶ [ 99 ]
 y ────────────^

Y es solo un *préstamo*, no un dueño.
El único dueño sigue siendo bx.
Cuando bx se libera, el heap también se libera, 
aunque y existiera.

---

Conclusión:
- Tener varias referencias (`&bx`, `&*bx`, etc.) no significa 
  múltiples dueños. Solo hay un dueño: el `Box`.
- Si quieres múltiples dueños REALES, debes usar `Rc<T>` o `Arc<T>`.

***/
fn box_demo() {
    let bx = Box::new(99);
    let y = &*bx;
    println!("Box contiene {} {:p}", bx, y);
}


/***
-----------------------------------------------------------
   2. Rc<T>: Contador de referencias (solo single-thread)
-----------------------------------------------------------

¿Qué es Rc<T>?
Rc significa Reference Counted.

Es un puntero inteligente que te permite tener múltiples dueños de un mismo valor EN EL HEAP.

Funciona solo en single-thread (no es seguro para hilos, para eso existe Arc<T>).

La idea: cada vez que clonas un Rc, no se copia el dato, sino que se incrementa un 
contador interno. Cuando todas las referencias se liberan, el valor en el heap también se libera.


Stack:        Heap:
rc1 ─┐
     ├──────> "hola"  (contador: 2)
rc2 ─┘


¿Cuándo usar Rc<T>?

Cuando varias partes de tu programa necesitan leer el mismo dato en el heap.

Ejemplo: estructuras de datos compartidas como árboles o grafos, donde
un mismo nodo puede ser referenciado desde varios lugares.


Se crea un Rc que contiene "hola".
El String está en el heap.
rc1 está en el stack, apuntando al heap.
El contador de referencias ahora vale 1.


Se crea rc2 que apunta al mismo "hola".
El contenido NO se copia (es decir, no se duplica "hola").
Solo aumenta el contador de referencias a 2.


Ambos rc1 y rc2 imprimen "hola", porque apuntan al mismo valor y strong_count retorna 2.

---

IMPORTANTE recordar que se guarda en el HEAP y esto tiene muchas ventajas.

1- Permite manejar datos indeterminados en tamaño.
2- Permite compartir datos entre diferentes partes de un programa sin necesidad de copiarlos. Por ejemplo,
cuando una función terminó es posible mantener la información en el heap y seguir usándola.

***/

fn rc_demo() {
    println!("\n--- Rc<T> ---");
    let rc1 = Rc::new(String::from("hola"));
    let rc2 = Rc::clone(&rc1); // otra referencia al mismo dato
    println!("rc1 = {}, rc2 = {}", rc1, rc2);
    println!("rc1 strong_count = {}", Rc::strong_count(&rc1));
}

/***
-----------------------------------------------------------
   3. Arc<T>: Contador de referencias SEGURO EN HILOS
-----------------------------------------------------------

¿Qué es Arc<T>?
Arc significa *Atomic Reference Counted*.

Es muy parecido a Rc<T>: un puntero inteligente que permite múltiples dueños
de un mismo valor en el heap. 

Diferencia: Arc<T> usa operaciones atómicas para manejar el contador de referencias,
lo que lo hace SEGURO en entornos multi-thread.

Stack (varios threads):          Heap:
thread1 ── arc1 ─┐
                 ├──────> 123 (contador atómico: 2)
thread2 ── arc2 ─┘

-----------------------------------------------------------

¿Cuándo usar Arc<T>?

Cuando varias partes de tu programa (incluso en hilos distintos) necesitan
acceder al mismo valor en el heap.

Por ejemplo:
- Compartir una configuración global entre threads.
- Estructuras grandes que quieres evitar copiar, pero que varios hilos
  necesitan leer.

-----------------------------------------------------------

IMPORTANTE:

- Arc<T> provee seguridad de memoria en hilos, pero NO sincronización de datos.
  Si necesitas modificar el valor compartido, deberías usarlo junto con un
  `Mutex<T>` o `RwLock<T>`.  
  Ejemplo: `Arc<Mutex<T>>`

- Arc<T> solo hace que el *contador* de referencias sea seguro en múltiples hilos.
  El valor que guarda puede seguir siendo inmutable. Para mutabilidad, toca combinarlo.

-----------------------------------------------------------

Ejemplo:

Se crea un Arc que contiene 123 en el heap.
arc1 está en el stack, apuntando al heap.
El contador ahora vale 1.

Se clona arc1 para crear arc2.
El dato 123 no se copia, solo aumenta el contador atómico a 2.

Ambos arc1 y arc2 apuntan al mismo valor y lo pueden imprimir.

***/

fn arc_demo() {
    println!("\n--- Arc<T> ---");
    let arc1 = Arc::new(123);
    let arc2 = arc1.clone();
    println!("arc1 = {}, arc2 = {}", arc1, arc2);
}

/*
----------------------------------------------------------- 
 4. RefCell<T>: Mutabilidad interior (runtime borrow check)
----------------------------------------------------------- 
*/

fn refcell_demo() {
    println!("\n--- RefCell<T> ---");
    let data = RefCell::new(10);
    {
        let mut_ref = data.borrow_mut(); // mut borrow en runtime
        println!("dato mut = {}", mut_ref);
    }
    *data.borrow_mut() += 5;
    println!("dato final = {}", data.borrow());
}

//
// 5. Raw pointers (punteros crudos, inseguros)
//
fn raw_pointers_demo() {
    println!("\n--- Raw pointers (unsafe) ---");
    let mut x = 42;
    let r1 = &x as *const i32; // puntero crudo de solo lectura
    let r2 = &mut x as *mut i32; // puntero crudo mutable

    unsafe {
        println!("r1 = {:p}, valor = {}", r1, *r1);
        *r2 += 1;
        println!("r2 = {:p}, valor = {}", r2, *r2);
    }
}

//
// 6. Box + trait objects (dinamismo en heap)
//
trait Animal {
    fn sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn sound(&self) { println!("Guau!"); }
}

struct Cat;
impl Animal for Cat {
    fn sound(&self) { println!("Miau!"); }
}

fn trait_objects_demo() {
    println!("\n--- Box<dyn Trait> ---");
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for a in animals {
        a.sound();
    }
}

pub fn pointers_tour() {
    // pointers_demo();
    rc_demo();
    // arc_demo();
    // refcell_demo();
    // raw_pointers_demo();
    // trait_objects_demo();
}
