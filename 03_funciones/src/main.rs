// =============================================================================
// MÓDULO 03 — FUNCIONES
// Ejecuta con:  cargo run -p funciones
// =============================================================================
//
// - Las funciones se declaran con `fn`, en snake_case.
// - Los tipos de los parámetros son OBLIGATORIOS (no hay inferencia en la firma).
// - El tipo de retorno va tras `->`. Sin `->` la función devuelve `()`.
// - El orden no importa: puedes llamar a una función definida más abajo.

fn main() {
    println!("===== 1. Parámetros y retorno =====");
    println!("sumar(2, 3) = {}", sumar(2, 3));
    println!("absoluto(-7) = {}", absoluto(-7));
    saludar("Ferris");

    println!("\n===== 2. Varios valores de retorno (tuplas) =====");
    let (cociente, resto) = dividir(17, 5);
    println!("17 / 5 = {cociente} resto {resto}");
    let (min, max, media) = estadisticas(&[4.0, 8.0, 15.0, 16.0, 23.0, 42.0]);
    println!("min={min} max={max} media={media:.2}");

    println!("\n===== 3. Slices en lugar de variádicas =====");
    println!("suma_todos(&[1,2,3,4]) = {}", suma_todos(&[1, 2, 3, 4]));
    println!("suma_todos(&[]) = {}", suma_todos(&[]));
    // Rust no tiene funciones variádicas ni parámetros opcionales ni
    // sobrecarga. Se usan slices, Option<T>, o el patrón builder (módulo 05).
    println!("{}", saludo_opcional(None));
    println!("{}", saludo_opcional(Some("Ana")));

    println!("\n===== 4. Funciones como valores =====");
    funciones_como_valores();

    println!("\n===== 5. Closures =====");
    closures();

    println!("\n===== 6. Funciones que devuelven funciones =====");
    let por_tres = multiplicador(3);
    let mut contador = crear_contador();
    println!("por_tres(7) = {}", por_tres(7));
    println!("contador: {} {} {}", contador(), contador(), contador());

    println!("\n===== 7. Recursión =====");
    println!("factorial(10) = {}", factorial(10));
    println!(
        "fibonacci(0..10) = {:?}",
        (0..10).map(fibonacci).collect::<Vec<_>>()
    );

    println!("\n===== 8. Funciones que nunca vuelven: ! =====");
    let n: u32 = "8".parse().unwrap_or_else(|_| fallar("no es un número"));
    println!("n = {n}");
}

fn sumar(a: i32, b: i32) -> i32 {
    a + b // la última expresión es el valor de retorno (sin `;` y sin `return`)
}

fn absoluto(n: i32) -> i32 {
    // `return` existe, y se usa para salir ANTES del final
    if n >= 0 {
        return n;
    }
    -n
}

fn saludar(nombre: &str) {
    // Sin `->`: devuelve `()`
    println!("¡Hola, {nombre}!");
}

fn dividir(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

// `&[f64]` es un SLICE: una vista a una secuencia de f64 (de un array, de un
// Vec...). Es la forma idiomática de aceptar "una lista" sin copiarla.
fn estadisticas(datos: &[f64]) -> (f64, f64, f64) {
    let min = datos.iter().copied().fold(f64::INFINITY, f64::min);
    let max = datos.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let media = datos.iter().sum::<f64>() / datos.len() as f64;
    (min, max, media)
}

fn suma_todos(numeros: &[i32]) -> i32 {
    numeros.iter().sum()
}

fn saludo_opcional(nombre: Option<&str>) -> String {
    format!("Hola, {}", nombre.unwrap_or("desconocido"))
}

fn funciones_como_valores() {
    // Una función tiene un tipo puntero-a-función: fn(i32, i32) -> i32
    let operacion: fn(i32, i32) -> i32 = sumar;
    println!("operacion(10, 5) = {}", operacion(10, 5));

    // Tabla de operaciones. `type` da un alias a un tipo largo.
    type Operacion = fn(i32, i32) -> i32;
    let operaciones: [(&str, Operacion); 3] = [
        ("suma", sumar),
        ("resta", |a, b| a - b), // un closure que no captura nada se convierte en fn
        ("producto", |a, b| a * b),
    ];
    for (nombre, f) in operaciones {
        println!("  {nombre:<8} 6,3 → {}", f(6, 3));
    }

    // Función que recibe otra función (genérica sobre cualquier cosa "llamable")
    fn aplicar_dos_veces(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(f(x))
    }
    println!(
        "aplicar_dos_veces(+10, 1) = {}",
        aplicar_dos_veces(|x| x + 10, 1)
    );
}

fn closures() {
    // Un closure es una función anónima que puede CAPTURAR variables del entorno.
    // Sintaxis: |parámetros| expresión   (los tipos se suelen inferir)
    let factor = 2;
    let doblar = |x: i32| x * factor; // captura `factor` por referencia
    println!("doblar(21) = {}", doblar(21));

    // Los closures implementan uno o varios de estos traits, según qué hagan
    // con lo que capturan (el compilador lo decide solo):
    //   Fn      → solo LEE lo capturado           (se puede llamar muchas veces)
    //   FnMut   → MODIFICA lo capturado           (necesita `let mut`)
    //   FnOnce  → CONSUME (mueve) lo capturado    (solo se puede llamar una vez)
    let mut total = 0;
    let mut acumular = |x: i32| total += x; // FnMut
    acumular(5);
    acumular(10);
    println!("total acumulado = {total}");

    let mensaje = String::from("adiós");
    let consumir = move || mensaje; // `move` fuerza a tomar posesión → FnOnce aquí
    let recuperado = consumir();
    // consumir();  // ERROR: ya se consumió
    // println!("{mensaje}"); // ERROR: `mensaje` se movió dentro del closure
    println!("FnOnce devolvió {recuperado:?}");

    // Uso típico: ordenar con un criterio
    let mut palabras = vec!["kiwi", "plátano", "uva", "fresa"];
    palabras.sort_by_key(|p| p.chars().count());
    println!("ordenadas por longitud: {palabras:?}");
}

// Devolver un closure: `impl Fn(...)` = "algún tipo que implementa Fn".
// `move` hace que el closure se lleve `n` consigo (si no, `n` moriría al salir).
fn multiplicador(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

// Un contador con estado privado: el closure guarda `cuenta`
fn crear_contador() -> impl FnMut() -> u32 {
    let mut cuenta = 0;
    move || {
        cuenta += 1;
        cuenta
    }
}

fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// El tipo `!` ("never") significa que la función NUNCA retorna (panic, exit,
// bucle infinito). Encaja con cualquier tipo, por eso sirve en unwrap_or_else.
fn fallar(msg: &str) -> ! {
    panic!("{msg}");
}
