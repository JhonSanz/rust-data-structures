// =============================================================================
// MÓDULO 04 — OWNERSHIP Y BORROWING (el corazón de Rust)
// Ejecuta con:  cargo run -p ownership
// =============================================================================
//
// Rust gestiona la memoria SIN recolector de basura y SIN free() manual,
// gracias a tres reglas que el compilador comprueba (el "borrow checker"):
//
//   1. Cada valor tiene un único DUEÑO (owner).
//   2. Cuando el dueño sale de su ámbito, el valor se LIBERA (se llama a `drop`).
//   3. Puedes PRESTAR el valor:
//        - muchas referencias de lectura  &T      a la vez, O BIEN
//        - una única referencia mutable   &mut T
//      ...pero nunca las dos cosas a la vez. Y una referencia nunca puede
//      vivir más que el valor al que apunta.
//
// Resultado: sin punteros colgantes, sin doble free, sin carreras de datos.
// Todo comprobado en compilación, con coste cero en ejecución.

fn main() {
    println!("===== 1. Ámbito y drop =====");
    ambito_y_drop();

    println!("\n===== 2. Move: transferir la propiedad =====");
    movimientos();

    println!("\n===== 3. Copy y Clone =====");
    copy_y_clone();

    println!("\n===== 4. Ownership y funciones =====");
    ownership_y_funciones();

    println!("\n===== 5. Referencias (&T): prestar sin ceder =====");
    referencias();

    println!("\n===== 6. Referencias mutables (&mut T) =====");
    referencias_mutables();

    println!("\n===== 7. Slices: referencias a una parte =====");
    slices();

    println!("\n===== 8. Box: valores en el heap =====");
    heap_con_box();
}

/*
-----------------------------------------------------------
   A. STACK y HEAP: dónde viven los datos
-----------------------------------------------------------

Para entender ownership hay que saber DÓNDE se guarda cada cosa.
(Lo tienes explicado también en estructuras_de_datos/basic/src/pointers.rs.)

- STACK (pila): datos de tamaño FIJO y conocido al compilar.
  Muy rápido: reservar es mover un puntero, liberar es automático
  al salir de la función.
- HEAP (montículo): datos cuyo tamaño puede CAMBIAR o no se conoce
  hasta ejecutar (un texto que el usuario escribe, una lista que crece).
  Más lento: hay que pedir memoria al sistema y alguien debe liberarla.

Ejemplo:

let n = 5;                      // i32: 4 bytes fijos → STACK
let s = String::from("hola");   // el texto puede crecer → HEAP

STACK                          HEAP
n  [ 5 ]
s  [ ptr | len: 4 | cap: 4 ] ──▶ [ h | o | l | a ]

Fíjate: `s` NO es el texto. `s` es una "cabecera" de 3 números
(24 bytes) en el stack que APUNTA al texto en el heap.

-----------------------------------------------------------
   B. ¿Quién libera el heap? El DUEÑO
-----------------------------------------------------------

Otros lenguajes resuelven "¿cuándo libero el heap?" así:
- C:        tú llamas a free()  → olvidos (fugas) o dobles free (crash)
- Go, Java: un recolector de basura (GC) lo hace solo → pausas y más RAM

Rust usa una tercera vía: cada valor tiene UN DUEÑO (owner), que es
una variable. Cuando el dueño sale de su ÁMBITO (scope, el bloque { }
donde se declaró), Rust llama automáticamente a `drop`, que libera
el heap. Sin GC y sin free() manual.

{
    let s = String::from("hola");   // s es dueño del texto
    ...
}                                    // ← fin del ámbito: drop(s), heap liberado

-----------------------------------------------------------
   C. Drop: el "destructor"
-----------------------------------------------------------

`Drop` es un trait con un solo método, `drop(&mut self)`, que Rust
llama al destruir un valor. String, Vec, Box, File... lo implementan
para liberar su memoria o cerrar su archivo. Tú también puedes
implementarlo (abajo, `Recurso` imprime un mensaje al morir).

IMPORTANTE:
- Los valores se destruyen en orden INVERSO al de creación.
- Este patrón se llama RAII ("la adquisición de un recurso es su
  inicialización"): el recurso vive exactamente lo que vive su dueño.
  Por eso en Rust no hace falta `defer` ni `finally`.
*/

// Un tipo con destructor para VER cuándo se libera cada valor.
struct Recurso(&'static str);

impl Drop for Recurso {
    fn drop(&mut self) {
        println!("  drop({}) → liberado", self.0);
    }
}

fn ambito_y_drop() {
    let _a = Recurso("a");
    {
        let _b = Recurso("b");
        println!("  dentro del bloque interior");
    } // ← aquí muere `b`
    println!("  fuera del bloque interior");
    let c = Recurso("c");
    drop(c); // liberación anticipada explícita
    println!("  fin de la función");
} // ← aquí muere `a` (los valores se liberan en orden inverso de creación)

/*
-----------------------------------------------------------
   D. MOVE: transferir la propiedad
-----------------------------------------------------------

¿Qué pasa al hacer `let s2 = s1;` con un String?

ANTES:
STACK                              HEAP
s1 [ ptr | len 4 | cap 4 ] ──────▶ [ h | o | l | a ]

DESPUÉS de `let s2 = s1;`:
STACK                              HEAP
s1 [   ✗ inválida ✗       ]
s2 [ ptr | len 4 | cap 4 ] ──────▶ [ h | o | l | a ]

- Se copia SOLO la cabecera (24 bytes), no el texto del heap.
- `s1` queda INVÁLIDA: el compilador ya no te deja usarla.

¿Por qué invalidar s1? Si las dos fueran válidas, al terminar el
ámbito se haría drop(s1) y drop(s2) → se liberaría DOS VECES la misma
memoria (un "double free", un bug grave en C). Con un único dueño,
eso es imposible.

A esto se le llama MOVER (move). Ocurre al:
- asignar:              let b = a;
- pasar a una función:  f(a);
- devolver:             return a;
- recorrer con for:     for x in vec { }   (el for se queda con el Vec)

Conclusión:
- Mover es BARATO (copiar 24 bytes), nunca copia datos del heap.
- Si de verdad quieres dos copias independientes: `.clone()`.
- Si solo quieres que alguien USE el valor sin quedárselo: préstalo
  con `&` (ver sección F).
*/

fn movimientos() {
    // String guarda sus datos en el heap. Asignarla a otra variable MUEVE la
    // propiedad: no se copian los datos, y la variable original queda inválida.
    let s1 = String::from("hola");
    let s2 = s1;
    // println!("{s1}");  // ERROR: borrow of moved value: `s1`
    println!("s2 = {s2} (s1 ya no es válida)");

    // ¿Por qué? Si las dos fueran válidas, al salir del ámbito ambas
    // intentarían liberar la misma memoria (doble free). Rust lo impide.

    // Lo mismo pasa con Vec, Box, y cualquier tipo que no sea Copy.
    let v1 = vec![1, 2, 3];
    let v2 = v1;
    println!("v2 = {v2:?}");

    // Un `for` sobre un Vec también lo mueve (lo consume):
    let nombres = vec![String::from("Ana"), String::from("Luis")];
    for n in nombres {
        print!("{n} ");
    }
    println!();
    // println!("{nombres:?}"); // ERROR: `nombres` se movió al for
}

/*
-----------------------------------------------------------
   E. Copy vs Clone
-----------------------------------------------------------

¿Por qué `let y = x;` con un i32 NO invalida x?

Porque i32 implementa el trait `Copy`: vive entero en el stack y
copiarlo es tan barato como moverlo. Así que Rust lo copia y las dos
variables siguen siendo válidas.

STACK
x [ 5 ]
y [ 5 ]     ← copia independiente, no hay heap de por medio

| Trait   | Qué hace                         | Cuándo ocurre   | Coste        |
| ------- | -------------------------------- | --------------- | ------------ |
| Copy    | Copia bit a bit                  | AUTOMÁTICO      | Muy barato   |
| Clone   | Copia profunda (incluido heap)   | Llamando clone() | Puede ser caro |

Son Copy: enteros, f32/f64, bool, char, referencias &T, y tuplas o
arrays formados SOLO por tipos Copy.
NO son Copy: String, Vec, Box... (todo lo que es dueño de heap).

IMPORTANTE:
- Ver muchos `.clone()` en un código suele indicar que se está
  "peleando" con el borrow checker. Casi siempre hay una solución con
  referencias.
*/

fn copy_y_clone() {
    // Los tipos simples de tamaño fijo (enteros, f64, bool, char, y tuplas o
    // arrays de ellos) implementan `Copy`: se copian bit a bit, no se mueven.
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y} (i32 es Copy)");

    // `clone()` hace una copia PROFUNDA explícita (puede ser costosa).
    let s1 = String::from("datos");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2} (clonada)");

    // Tus tipos pueden ser Copy/Clone con derive:
    #[derive(Debug, Clone, Copy)]
    struct Punto {
        x: i32,
        y: i32,
    }
    let p1 = Punto { x: 1, y: 2 };
    let p2 = p1; // copia, no move
    println!("p1 = {p1:?}, p2 = {p2:?} (x+y = {})", p2.x + p2.y);
}

fn ownership_y_funciones() {
    let s = String::from("mensaje");
    tomar_propiedad(s); // `s` se MUEVE a la función...
    // println!("{s}"); // ERROR: ...y ya no es nuestra

    let n = 7;
    copiar_entero(n); // i32 es Copy: `n` sigue siendo válida
    println!("n sigue valiendo {n}");

    // Una función puede devolver la propiedad
    let s = crear_string();
    let s = anadir_exclamacion(s); // la cedemos y nos la devuelven
    println!("recuperada: {s}");
    // Esto es incómodo... por eso existen las REFERENCIAS.
}

fn tomar_propiedad(texto: String) {
    println!("  tomar_propiedad recibió {texto:?}");
} // aquí se libera `texto`

fn copiar_entero(n: i32) {
    println!("  copiar_entero recibió {n}");
}

fn crear_string() -> String {
    String::from("nueva")
}

fn anadir_exclamacion(mut s: String) -> String {
    s.push('!');
    s
}

/*
-----------------------------------------------------------
   F. BORROWING: prestar sin ceder la propiedad
-----------------------------------------------------------

¿Qué es una referencia?
Un puntero SEGURO a un valor que es de otro. Crearla se llama
"tomar prestado" (borrow). Como prestar un libro: tú lo lees, pero
sigue siendo del dueño, y tienes que devolverlo antes de que el
dueño se vaya.

let s = String::from("hola");
let r = &s;

STACK                              HEAP
s [ ptr | len | cap ] ───────────▶ [ h | o | l | a ]
r [ ptr ] ──▶ s

- `r` apunta a `s` (a su cabecera), que apunta al heap.
- `s` sigue siendo el dueño: cuando `s` muera, se libera el heap.
- `r` NO libera nada al morir: no es dueño.

Dos tipos de préstamo:

| Tipo     | Sintaxis | Permite         | Cuántos a la vez |
| -------- | -------- | --------------- | ---------------- |
| Compartido | &T     | Solo LEER       | Todos los que quieras |
| Exclusivo  | &mut T | Leer y MODIFICAR| Solo UNO (y ningún &T) |

Regla de oro (la comprueba el "borrow checker" al compilar):
    O muchos lectores, O un único escritor. Nunca las dos cosas.

Y además: una referencia NUNCA puede vivir más que el valor al que
apunta. Así es imposible tener un "puntero colgante" (dangling
pointer): un puntero a memoria ya liberada.
*/

fn referencias() {
    let s = String::from("prestada");

    // `&s` crea una referencia: PRESTA el valor sin ceder la propiedad.
    let n = longitud(&s);
    println!("longitud(&s) = {n}, y `s` sigue siendo nuestra: {s}");

    // Se pueden tener MUCHAS referencias inmutables a la vez
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {r1}, r2 = {r2}");

    // El compilador impide referencias colgantes:
    //
    //   let r;
    //   {
    //       let temporal = String::from("x");
    //       r = &temporal;
    //   }               // `temporal` muere aquí...
    //   println!("{r}"); // ERROR: `temporal` does not live long enough
}

// `&String` sería válido, pero `&str` es más flexible (acepta &String y literales).
fn longitud(s: &str) -> usize {
    s.len()
}

/*
-----------------------------------------------------------
   G. ¿Por qué no puedo leer y modificar a la vez?
-----------------------------------------------------------

Parece una regla caprichosa hasta que ves el bug que evita:

let mut v = vec![1, 2, 3];
let primero = &v[0];   // préstamo compartido: apunta DENTRO del heap del Vec
v.push(4);             // ✗ ERROR de compilación... ¿por qué?

El Vec tenía sitio para 3 elementos. Para meter el 4º, `push` pide un
bloque NUEVO más grande, copia los datos y LIBERA el bloque viejo:

ANTES del push:
v [ ptr | len 3 | cap 3 ] ──▶ [ 1 | 2 | 3 ]      (bloque viejo)
primero ─────────────────────────^

DESPUÉS del push:
v [ ptr | len 4 | cap 6 ] ──▶ [ 1 | 2 | 3 | 4 | _ | _ ]   (bloque nuevo)
primero ──▶ [ memoria liberada ] 💥

`primero` apuntaría a memoria que ya no existe. En C esto compila y
falla "a veces". En Rust no compila: es exactamente la regla
"no se puede tener &v mientras alguien tiene &mut v".

-----------------------------------------------------------

NLL ("Non-Lexical Lifetimes"): un préstamo termina en su ÚLTIMO USO,
no al cerrar la llave. Por eso esto SÍ compila:

let r1 = &mut s;
r1.push('!');      // último uso de r1 → aquí acaba el préstamo
let r2 = &s;       // OK: r1 ya no está activo
*/

fn referencias_mutables() {
    let mut s = String::from("hola");
    anadir_mundo(&mut s); // préstamo MUTABLE
    println!("tras anadir_mundo: {s}");

    // Solo UNA referencia mutable a la vez (y ninguna inmutable mientras tanto):
    let r1 = &mut s;
    r1.push('!');
    // let r2 = &mut s;     // ERROR: cannot borrow `s` as mutable more than once
    // println!("{s}");     // ERROR: no se puede leer mientras r1 está activa
    println!("r1 = {r1}");

    // Pero un préstamo termina en su ÚLTIMO USO, no al final del bloque
    // (Non-Lexical Lifetimes). Desde aquí `r1` ya no se usa, así que:
    let r2 = &s;
    println!("r2 = {r2}");

    // Por qué existe esta regla: evita que modifiques una colección mientras
    // alguien la está leyendo. Ejemplo clásico que Rust rechaza:
    //
    //   let mut v = vec![1, 2, 3];
    //   let primero = &v[0];
    //   v.push(4);           // ERROR: push podría mover el Vec a otra zona de memoria
    //   println!("{primero}"); //        y `primero` apuntaría a memoria liberada
    let mut v = vec![1, 2, 3];
    let primero = v[0]; // copiamos el i32 en vez de prestar
    v.push(4);
    println!("primero = {primero}, v = {v:?}");
}

fn anadir_mundo(s: &mut String) {
    s.push_str(", mundo");
}

/*
-----------------------------------------------------------
   H. SLICES: una vista a un trozo
-----------------------------------------------------------

¿Qué es un slice?
Una referencia a una parte CONTIGUA de una colección, sin copiarla.
Tipos: &str (trozo de texto), &[T] (trozo de array o Vec).

let frase = String::from("hola mundo");
let hola: &str = &frase[0..4];

STACK                                HEAP
frase [ ptr | len 10 | cap 10 ] ───▶ [ h | o | l | a |   | m | u | n | d | o ]
hola  [ ptr | len 4 ] ──────────────▶  ^

Un slice es un "puntero gordo" (fat pointer): guarda DOS cosas,
dónde empieza y cuántos elementos tiene. No tiene `cap` porque no
puede crecer: no es dueño de nada.

IMPORTANTE:
- Por eso las funciones suelen recibir &str en vez de &String, y
  &[T] en vez de &Vec<T>: aceptan más cosas (literales, arrays,
  trozos) y no exigen nada que no necesiten.
- Como cualquier préstamo, mientras exista el slice no puedes
  modificar el original (frase.clear() no compilaría).
*/

fn slices() {
    // Un slice es una referencia a un RANGO contiguo de una colección.
    let frase = String::from("hola mundo cruel");
    let primera = primera_palabra(&frase);
    println!("primera palabra: {primera:?}");
    // frase.clear(); // ERROR: no se puede mutar mientras `primera` la presta

    let numeros = [10, 20, 30, 40, 50];
    let medio: &[i32] = &numeros[1..4];
    println!("medio = {medio:?}, len = {}", medio.len());

    // Slices mutables
    let mut datos = [5, 3, 1, 4, 2];
    let mitad = &mut datos[..3];
    mitad.sort();
    println!("tras ordenar la primera mitad: {datos:?}");
}

fn primera_palabra(s: &str) -> &str {
    // El slice devuelto vive tanto como `s` (lifetimes, módulo 09)
    s.split(' ').next().unwrap_or("")
}

fn heap_con_box() {
    // Box<T> pone un valor en el heap; el Box es su dueño y lo libera al morir.
    // Útil para tipos recursivos o grandes (más en el módulo 11).
    let b = Box::new(Recurso("en el heap"));
    println!("  tengo un Box con {}", b.0);

    // Rust no tiene `nil`: una referencia o un Box SIEMPRE apuntan a algo válido.
    // Para "puede que no haya nada" se usa Option<T> (módulo 05).
    let quizas: Option<Box<i32>> = None;
    println!("  quizas = {quizas:?}");
}
