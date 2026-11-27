# Glosario de Rust

Términos que aparecen en el tour, explicados de forma sencilla. Cada uno indica dónde se
explica a fondo; en los módulos marcados, busca los bloques `/* --- ... --- */` del código, con
diagramas y ejemplos.

> Lee primero los de **memoria** (stack, heap, ownership, borrowing): casi todo lo demás se apoya en ellos.

---

## Memoria y ownership

| Término | Qué es | Ejemplo | Dónde |
|---------|--------|---------|-------|
| **Stack** (pila) | Memoria rápida para datos de tamaño fijo conocido al compilar. Se libera sola al salir de la función. | `let n = 5;` | 04 · `basic/pointers.rs` |
| **Heap** (montículo) | Memoria para datos que pueden crecer o cuyo tamaño no se conoce hasta ejecutar. Alguien debe liberarla. | El texto de un `String` | 04 · `basic/pointers.rs` |
| **Ownership** (propiedad) | Regla de Rust: cada valor tiene **un único dueño**, y se libera cuando el dueño sale de su ámbito. Sustituye al recolector de basura. | `let s = String::from("x");` → `s` es la dueña | 04 |
| **Dueño** (owner) | La variable responsable de liberar un valor. | | 04 |
| **Ámbito** (scope) | El bloque `{ }` donde vive una variable. Al cerrarse, sus valores se destruyen. | | 04 |
| **Drop** | Trait cuyo método se ejecuta al destruir un valor (el "destructor"): libera memoria, cierra archivos... | `impl Drop for Recurso` | 04 |
| **RAII** | Patrón en el que un recurso vive exactamente lo que vive su dueño. Por eso Rust no necesita `defer` ni `finally`. | El `MutexGuard` suelta el candado al morir | 04, 12 |
| **Move** (mover) | Transferir la propiedad. Se copia solo la cabecera (no el heap) y la variable original queda inválida. | `let b = a;` | 04 |
| **Copy** | Trait de los tipos que se copian bit a bit automáticamente en vez de moverse (números, `bool`, `char`...). | `let y = x;` con `x: i32` | 04 |
| **Clone** | Copia profunda explícita, incluido el heap. Puede ser cara. | `let b = a.clone();` | 04 |
| **Double free** | Bug de liberar dos veces la misma memoria. Rust lo impide con el dueño único. | | 04 |
| **Puntero colgante** (dangling) | Puntero a memoria ya liberada. Imposible en Rust seguro. | | 04, 09 |
| **Fuga de memoria** (leak) | Memoria que nunca se libera. Rust no la considera insegura: puede pasar con ciclos de `Rc`. | | 11 |

## Préstamos y referencias

| Término | Qué es | Ejemplo | Dónde |
|---------|--------|---------|-------|
| **Referencia** | Puntero seguro a un valor que es de otro. No lo libera al morir. | `&s` | 04 |
| **Borrowing** (préstamo) | Crear una referencia: usar un valor sin quedárselo. | `longitud(&s)` | 04 |
| **`&T`** (préstamo compartido) | Solo lectura. Puede haber muchos a la vez. | `let r = &v;` | 04 |
| **`&mut T`** (préstamo exclusivo) | Lectura y escritura. Solo uno, y ningún `&T` a la vez. | `anadir(&mut s)` | 04 |
| **Borrow checker** | La parte del compilador que comprueba las reglas de préstamos y lifetimes. | | 04 |
| **NLL** (Non-Lexical Lifetimes) | Un préstamo termina en su **último uso**, no al cerrar la llave. | | 04 |
| **Slice** | Referencia a un trozo contiguo de una colección, sin copiarlo: `&str`, `&[T]`. | `&v[1..3]` | 04 |
| **Fat pointer** (puntero gordo) | Puntero con información extra: un slice guarda (dirección, longitud); un `&dyn Trait`, (datos, vtable). | | 04, 07 |
| **Desreferenciar** | Acceder al valor al que apunta una referencia, con `*`. | `*x += 1` | 04, 11 |
| **Lifetime** | Tramo del programa en el que una referencia es válida. Se escribe `'a` solo para relacionar varias referencias; **no cambia** cuánto vive nada. | `fn f<'a>(a: &'a str) -> &'a str` | 09 |
| **Elisión** | Reglas con las que el compilador deduce los lifetimes solo, para que casi nunca los escribas. | | 09 |
| **`'static`** | Lifetime de todo el programa (literales, constantes). | `let s: &'static str = "hola";` | 09 |

## Tipos y traits

| Término | Qué es | Ejemplo | Dónde |
|---------|--------|---------|-------|
| **Struct** | Tipo que agrupa datos con nombre ("Y"). | `struct Punto { x: i32, y: i32 }` | 05 |
| **Enum** | Tipo que es **una de varias** variantes, cada una con sus datos ("O"). | `enum Forma { Circulo(f64), Punto }` | 05 |
| **`Option<T>`** | Enum `Some(valor)` / `None`: el sustituto seguro de `null`. | `v.get(10)` | 05 |
| **`Result<T, E>`** | Enum `Ok(valor)` / `Err(error)`: para operaciones que pueden fallar. | `"5".parse::<i32>()` | 08 |
| **Pattern matching** | Comparar un valor con patrones y desestructurarlo: `match`, `if let`, `let else`. Es **exhaustivo**. | `match x { Some(v) => .., None => .. }` | 02, 05 |
| **`impl`** | Bloque donde se definen los métodos de un tipo, o la implementación de un trait. | `impl Rectangulo { fn area(&self) .. }` | 05 |
| **`self` / `&self` / `&mut self`** | Cómo recibe un método al objeto: consumiéndolo, leyéndolo o modificándolo. | | 05 |
| **Trait** | Contrato: lista de métodos que un tipo promete tener. Como una interfaz, con implementación explícita. | `trait Forma { fn area(&self) -> f64; }` | 07 |
| **`derive`** | Atributo que genera implementaciones de traits automáticamente. | `#[derive(Debug, Clone)]` | 05 |
| **Trait bound** (restricción) | Capacidades que exiges a un tipo genérico. | `T: PartialOrd + Copy` | 09 |
| **Genérico** | "Hueco" para un tipo que se rellena al usar la función o el struct. | `fn mayor<T>(..)`, `Vec<T>` | 09 |
| **Monomorfización** | El compilador genera una copia de cada genérico por cada tipo usado: coste cero en ejecución. | | 07, 09 |
| **`impl Trait`** | "Algún tipo que implementa este trait" (despacho estático). | `fn f(x: impl Display)` | 03, 07 |
| **`dyn Trait`** | "Cualquier tipo que implemente este trait", resuelto en ejecución (despacho dinámico). | `Vec<Box<dyn Forma>>` | 07 |
| **Despacho estático / dinámico** | Decidir qué función concreta se llama al compilar (genéricos) o al ejecutar (vtable). | | 07 |
| **Vtable** | Tabla de punteros a funciones que acompaña a un `&dyn Trait`. | | 07 |
| **Tipo asociado** | Tipo que el implementador elige dentro de un trait. | `type Output;` en `Add`, `type Item;` en `Iterator` | 07, 09 |
| **Supertrait** | Trait que exige otro. | `trait Resumible: Display` | 07 |
| **Blanket impl** | Implementar un trait para **todos** los tipos que cumplan una condición. | `impl<T: Display> Gritar for T` | 07 |
| **Regla del huérfano** (orphan rule) | Solo puedes hacer `impl Trait for Tipo` si el trait o el tipo son de tu crate. | | 07 |
| **Const generics** | Parámetros genéricos que son valores, no tipos. | `struct Matriz<const N: usize>` | 09 |
| **Newtype** | Struct tupla de un solo campo para dar significado a un tipo. | `struct Metros(f64);` | 05 |

## Funciones, closures e iteradores

| Término | Qué es | Ejemplo | Dónde |
|---------|--------|---------|-------|
| **Expresión** | Algo que produce un valor. En Rust casi todo lo es (`if`, `match`, bloques). | `let x = if c { 1 } else { 2 };` | 02 |
| **Closure** | Función anónima que puede **capturar** variables de su entorno. | `\|x\| x * factor` | 03 |
| **Capturar** | Que un closure use variables del sitio donde se creó (prestadas o movidas). | | 03 |
| **`Fn` / `FnMut` / `FnOnce`** | Traits de los closures según lo que hacen con lo capturado: leer / modificar / consumir. | | 03 |
| **`move`** | Obliga a un closure a ser dueño de lo que captura. | `move \|\| datos.len()` | 03, 12 |
| **Iterador** | Tipo que implementa `Iterator`: produce elementos uno a uno con `next()`. | `v.iter()` | 10 |
| **Adaptador** | Método que transforma un iterador en otro, sin ejecutar nada todavía. | `map`, `filter`, `zip` | 10 |
| **Consumidor** | Método que recorre el iterador y produce un resultado. | `collect`, `sum`, `count` | 10 |
| **Perezoso** (lazy) | Que no hace nada hasta que se le pide un resultado. Iteradores y Futures lo son. | | 10, 13 |
| **Tipo `!`** (never) | Tipo de lo que nunca devuelve: `panic!`, bucle infinito, `process::exit`. | `fn fallar() -> !` | 03 |

## Errores

| Término | Qué es | Ejemplo | Dónde |
|---------|--------|---------|-------|
| **Operador `?`** | Si es `Ok`/`Some`, saca el valor; si es `Err`/`None`, **sale** de la función con el error, convertido con `From`. | `let n = s.parse::<i32>()?;` | 08 |
| **`panic!`** | Error irrecuperable (un bug): detiene el hilo. | `v[99]` fuera de rango | 08 |
| **`unwrap` / `expect`** | Sacan el valor o hacen panic. Para casos que serían un bug, o tests. | | 05, 08 |
| **`Box<dyn Error>`** | "Cualquier error". Cómodo en aplicaciones y `main`. | | 08 |
| **`source()`** | Método de `Error` que devuelve el error que causó este: la cadena de causas. | | 08 |

## Smart pointers

| Término | Qué es | Ejemplo | Dónde |
|---------|--------|---------|-------|
| **Smart pointer** | Struct que actúa como referencia (`Deref`) y hace algo al morir (`Drop`). | `Box`, `Rc`, `String`, `Vec` | 11 · `basic/pointers.rs` |
| **`Box<T>`** | Valor en el heap con un único dueño. | `Box::new(5)` | 11 · `basic/pointers.rs` |
| **`Rc<T>`** | Varios dueños con un contador de referencias (un hilo). Solo lectura. | `Rc::clone(&a)` | 11 · `basic/pointers.rs` |
| **`Arc<T>`** | Como `Rc`, pero con contador atómico: válido entre hilos. | | 12 · `basic/pointers.rs` |
| **`Weak<T>`** | Referencia que no mantiene vivo el valor; rompe ciclos de `Rc`. `upgrade()` → `Option`. | | 11 |
| **Mutabilidad interior** | Modificar algo a través de una `&` (sin `&mut`), con comprobaciones en ejecución. | `Cell`, `RefCell`, `Mutex` | 11 · `basic/pointers.rs` |
| **`RefCell<T>`** | Mutabilidad interior en un hilo; las reglas de préstamo se comprueban al ejecutar (panic si se rompen). | `c.borrow_mut()` | 11 · `basic/pointers.rs` |
| **`Cow<T>`** | "Clone on write": prestado mientras no haga falta modificar. | | 11 |
| **Deref coercion** | Conversión automática de referencias siguiendo `Deref`: `&String` → `&str`. | | 11 |

## Concurrencia y async

| Término | Qué es | Ejemplo | Dónde |
|---------|--------|---------|-------|
| **Hilo** (thread) | Línea de ejecución del sistema operativo. Varios hilos corren en paralelo. | `thread::spawn(..)` | 12 |
| **Concurrencia / paralelismo** | Gestionar varias tareas a la vez / ejecutarlas literalmente al mismo tiempo. | | 12 |
| **Carrera de datos** (data race) | Dos hilos acceden al mismo dato a la vez, al menos uno escribe, sin sincronizar. **No compila** en Rust. | | 12 |
| **`Send`** | Marca de los tipos que se pueden **mover** a otro hilo. `Rc` no lo es. | | 12 |
| **`Sync`** | Marca de los tipos que se pueden **compartir** (`&T`) entre hilos. `RefCell` no lo es. | | 12 |
| **`Mutex<T>`** | Candado: solo un hilo accede al dato a la vez. El dato está **dentro** del Mutex. | `m.lock().unwrap()` | 12 · `basic/pointers.rs` |
| **Guard** | Valor que da acceso a algo bloqueado y lo libera al morir. | `MutexGuard` | 12 |
| **`RwLock<T>`** | Muchos lectores **o** un escritor. | | 12 · `basic/pointers.rs` |
| **Envenenamiento** (poisoning) | Un Mutex queda "envenenado" si un hilo hizo panic con él tomado. | | 12 |
| **Interbloqueo** (deadlock) | Dos hilos se esperan mutuamente para siempre. Rust **no** lo impide. | | 12 |
| **Atómico** | Operación que se ejecuta de una vez, sin que otro hilo la vea a medias. | `AtomicU64::fetch_add` | 12 |
| **Canal** (channel) | Tubería para enviar mensajes entre hilos o tareas. `mpsc`: varios productores, un consumidor. | `mpsc::channel()` | 12, 13 |
| **Back-pressure** | Un canal con capacidad limitada hace esperar al productor si el consumidor va lento. | `sync_channel(4)` | 12, 13 |
| **Future** | Valor que representa un cálculo que todavía no ha terminado. Lo devuelve una `async fn`. | | 13 |
| **`.await`** | Ejecuta un Future cediendo el hilo mientras espera. | `descargar().await` | 13 |
| **Poll / Waker** | Mecanismo interno: el runtime pregunta al Future si terminó; el Waker avisa cuando puede avanzar. | | 13 |
| **Runtime / executor** | El programa que ejecuta los Futures y reparte las tareas entre hilos. La std no trae uno; Tokio es el estándar. | `#[tokio::main]` | 13 |
| **Tarea** (task) | Future lanzado para ejecutarse por su cuenta. Como una goroutine; pesa KB, no MB. | `tokio::spawn(..)` | 13 |
| **Cancelación** | Destruir (drop) un Future lo detiene. Así funcionan `select!` y `timeout`. | | 13 |

## Organización del código y herramientas

| Término | Qué es | Ejemplo | Dónde |
|---------|--------|---------|-------|
| **Crate** | Unidad de compilación: una biblioteca (`lib.rs`) o un binario (`main.rs`). | | 14 |
| **Paquete** | Lo que define un `Cargo.toml`: una biblioteca como mucho y varios binarios. | | 14 |
| **Workspace** | Varios paquetes que comparten `Cargo.lock` y `target/`. Este repo es uno. | `Cargo.toml` de la raíz | 14 |
| **Módulo** | Forma de organizar el código dentro de un crate, con su visibilidad. | `mod calculadora;` | 14 |
| **Visibilidad** | Qué se ve desde fuera: privado por defecto, `pub`, `pub(crate)`, `pub(super)`. | | 14 |
| **Feature** | Compilación condicional que el usuario de un crate activa. | `--features cientifica` | 14 |
| **Macro** | Código que genera código al compilar. Lleva `!`. | `println!`, `vec!`, `macro_rules!` | 14 |
| **Doc-test** | Ejemplo de la documentación `///` que `cargo test` compila y ejecuta. | | 14, 15 |
| **Shadowing** | Declarar otra variable con el mismo nombre (puede cambiar de tipo). | `let x = x.trim();` | 01 |
| **`unsafe`** | Bloque donde tú, no el compilador, garantizas la seguridad de memoria (punteros crudos, memoria manual). | `estructuras_de_datos/vectors` | `vectors/` |
| **Clippy** | El linter oficial: sugiere código más idiomático. | `cargo clippy` | README |
