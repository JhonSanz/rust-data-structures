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
