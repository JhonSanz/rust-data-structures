use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::cell::RefCell;
use std::thread;



/**
-----------------------------------------------------------
   -1. El HEAP y el STACK
-----------------------------------------------------------

En rust tenemos dos maneras de almacenar información en memoria. Dependiendo de las
características de lo que queramos guardar elegimos una u otra

1. Para datos con tamaño conocido y de corta duración se usa el STACK
2. Para datos de tamaño variable o desconocido usamos el HEAP


- El stack es muy rápido ya que se almacenan cosas predecibles en tamaño y acceso
- El heap es lento porque se hacen operaciones complejas de búsqueda y asignación de memoria

La ventaja de rust es que no tiene garbage collector, sino que utiliza sistemas
de ownership para evitar fugas de memoria

| Característica     | **STACK**         | **HEAP**            |
| ------------------ | ----------------- | ------------------  |
| **Velocidad**      | Rápido            | Lento               |
| **Tamaño**         | Fijo              | Variable            |
| **Gestión**        | Automática        | Puntero + Ownership |
| **Almacenamiento** | Valores           | Punteros a valores  |
| **Uso**            | Datos predecibles | Datos dinámicos     |
*/

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

    /*
    ---------------------
        IMPORTANTE
    ---------------------

    Box<T> es un tipo de puntero inteligente que asigna memoria en el Heap y devuelve 
    un puntero a esa memoria. La variable Box en sí misma vive en el Stack.

    Por lo cual es posible crear un puntero mutable y editar el valor dentro del Box
    */


    let mut b = Box::new(5); // La variable 'b' y su contenido son mutables
    *b = 10; // Ahora podemos cambiar el valor dentro de la caja
    println!("El valor en la caja es: {}", b); // Muestra "El valor en la caja es: 10"
}

/***
-----------------------------------------------------------
   2. Rc<T>: Contador de referencias (solo single-thread)
-----------------------------------------------------------

¿Qué es Rc<T>?
Rc significa Reference Counted.

Es un puntero inteligente que te permite tener múltiples dueños
de un mismo valor EN EL HEAP.

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

---

Importante:
- Rc<T> solo permite acceso **inmutable** al valor que guarda.
- Si necesitas mutar el dato, debes envolverlo en `RefCell<T>`:
  Ejemplo: `Rc<RefCell<T>>`
  Esto te permite mutabilidad controlada en tiempo de ejecución.

---

Se crea un Rc que contiene "hola".
El String está en el heap.
rc1 está en el stack, apuntando al heap.
El contador de referencias ahora vale 1.


Se crea rc2 que apunta al mismo "hola".
El contenido NO se copia (es decir, no se duplica "hola").
Solo aumenta el contador de referencias a 2.


Ambos rc1 y rc2 imprimen "hola", porque apuntan al mismo valor y strong_count retorna 2.

---

IMPORTANTE recordar que se guarda en el HEAP y esto tiene muchas ventajas:

1- Permite manejar datos indeterminados en tamaño.
2- Permite compartir datos entre diferentes partes de un programa sin necesidad de copiarlos. 
   Por ejemplo, cuando una función terminó es posible mantener la información en el heap y 
   seguir usándola.

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
 4. RefCell<T> y Rc<T>: Mutabilidad Interior y Múltiples Dueños
-----------------------------------------------------------

Demostración de `Rc<RefCell<T>>`

`RefCell<T>` permite el patrón de **interior mutability**, es decir, modificar
un valor que está en un contenedor inmutable. `Rc<T>` permite que un dato tenga
**múltiples dueños**. Juntos, `Rc<RefCell<T>>` resuelven el problema de tener
un dato que es compartido pero que también necesita ser modificado.

Diferencia clave: el borrow checker de Rust usualmente valida los préstamos
(& y &mut) en **tiempo de compilación**. `RefCell<T>` traslada esa verificación
a **tiempo de ejecución (runtime)**.

Reglas de préstamos en runtime:
- Se permiten múltiples `borrow()` (préstamos inmutables).
- Solo se permite un `borrow_mut()` (préstamo mutable exclusivo).
- Si estas reglas se violan, el programa entra en `panic!`.

Casos de uso:
- Cuando necesitas que un dato sea compartido por múltiples partes del programa
  y que al mismo tiempo pueda ser modificado.
- Un ejemplo común son las estructuras de datos como grafos o árboles, donde un
  nodo puede tener múltiples referencias y necesitar ser actualizado.

Comparación con otros tipos de punteros inteligentes:
- **`Box<T>`:** El ownership es único, por lo que no se pueden compartir referencias.
- **`Rc<T>`:** Permite compartir la propiedad, pero no mutar el valor que contiene.
- **`Rc<RefCell<T>>`:** Logra ambas cosas, compartir y mutar el valor.

En este ejemplo, un simple contador (`i32`) es envuelto en `Rc<RefCell<T>>`.
Esto permite que:
1. El contador sea compartido por la variable `contador` y `contador_clonado`.
2. Se pueda mutar (`borrow_mut()`) el valor del contador desde cualquiera de las dos variables.

*/

fn refcell_demo() {
    // Se crea un `Rc<RefCell<i32>>` con un amount inicial de 0.
    // El dato `0` vive en el Heap. `amount` es la primera referencia.
    let amount = Rc::new(RefCell::new(0));
    println!("El valor original de amount es: {}", *amount.borrow());

    println!("--- Contamos referencias y manipulamos el dato ---");
    println!("El amount original tiene {} dueños.", Rc::strong_count(&amount));

    // Primer scope: se clona el amount y se modifica dentro de un bloque.
    {
        println!("\n-> Entrando en el primer scope...");
        let amount_clon1 = Rc::clone(&amount);
        println!("Ahora tenemos {} dueños.", Rc::strong_count(&amount));

        // Obtenemos un préstamo mutable y modificamos el valor.
        *amount_clon1.borrow_mut() += 1;
        println!("El valor del amount es: {}", *amount_clon1.borrow());
        
        // El `amount_clon1` se libera aquí al final del scope,
        // pero el dato en el Heap sigue vivo porque `amount` aún existe.
    } // `amount_clon1` se libera

    println!("\n-> Saliendo del primer scope...");
    println!("Volvemos a tener {} dueño(s).", Rc::strong_count(&amount));
    println!("El valor del amount sigue siendo: {}", *amount.borrow());

    // Segundo scope: se clona de nuevo y se modifica.
    {
        println!("\n-> Entrando en el segundo scope...");
        let amount_clon2 = Rc::clone(&amount);
        println!("De nuevo {} dueños.", Rc::strong_count(&amount));

        *amount_clon2.borrow_mut() += 5;
        println!("Ahora el valor del amount es: {}", *amount_clon2.borrow());
    } // `amount_clon2` se libera

    println!("\n-> Saliendo del segundo scope...");
    println!("Volvemos a tener {} dueño(s).", Rc::strong_count(&amount));
    println!("El valor final del amount es: {}", *amount.borrow());

    // La variable `amount` se libera aquí.
    // Ahora que la última referencia ha desaparecido, el dato en el Heap se limpia.
}

/*
----------------------------------------------------------- 
 5. Mutex<T> Mutabilidad en multithreading
-----------------------------------------------------------

Mutex<T> permite mutabilidad segura en entornos multihilo.
Proporciona un mecanismo de bloqueo para garantizar que solo un hilo
pueda acceder a los datos en un momento dado.

Funciona de manera similar a `RefCell<T>`, pero está diseñado para entornos multihilo.


RefCell<T> = mutabilidad controlada en single-thread.
Mutex<T> = mutabilidad controlada en multi-thread.
uno es el "hermano multithread" del otro.

*/


fn mutex_demo() {
    let shared = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let s = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut num = s.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Resultado final = {}", *shared.lock().unwrap());
}


/***
-----------------------------------------------------------
   6. RwLock<T>: Bloqueo de Lectura/Escritura
-----------------------------------------------------------

¿Qué es RwLock<T>?
Es un "Read-Write Lock".  
Permite que varios hilos lean el mismo dato al mismo tiempo,
pero SOLO un hilo puede escribir (y cuando escribe, nadie más puede acceder).

Diferencias clave:
- Mutex<T>: solo un acceso a la vez (lectura o escritura).
- RwLock<T>: múltiples lecturas concurrentes, pero escritura exclusiva.

-----------------------------------------------------------

Ejemplo mental:

Arc ─┐
     ├──> RwLock ───> valor (ej. i32)
Arc ─┘

* Varios hilos pueden hacer `.read()` al mismo tiempo.
* Cuando un hilo hace `.write()`, bloquea todas las lecturas
  y escrituras hasta terminar.

-----------------------------------------------------------

¿Cuándo usar RwLock<T>?

Cuando el patrón de acceso es:
- Muchas lecturas concurrentes.
- Pocas escrituras.

Ejemplo típico: cachés, configuraciones compartidas, tablas de datos
que rara vez cambian pero se leen constantemente.

-----------------------------------------------------------

Advertencias:
- Coordinar lectores y escritores tiene un costo extra.
- Si hay muchas escrituras, puede ser más lento que Mutex<T>.
- Al igual que Mutex, se puede caer en "deadlocks" si no se usa bien.

***/

fn rwlock_demo() {
    // Compartido entre hilos
    let data = Arc::new(RwLock::new(0));

    let mut handles = vec![];

    // Escritor
    {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut num = data.write().unwrap(); // bloqueo exclusivo
            *num += 10;
            println!("Escritor: valor actualizado a {}", *num);
        }));
    }

    // Lectores
    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let num = data.read().unwrap(); // bloqueo compartido
            println!("Lector {}: valor actual = {}", i, *num);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}



//
// 7. Raw pointers (punteros crudos, inseguros)
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
// 8. Box + trait objects (dinamismo en heap)
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
    // box_demo()
    // rc_demo();
    // arc_demo();
    refcell_demo();
    // raw_pointers_demo();
    // trait_objects_demo();
}
