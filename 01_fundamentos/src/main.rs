// =============================================================================
// MÓDULO 01 — FUNDAMENTOS
// Ejecuta con:  cargo run -p fundamentos
// =============================================================================
//
// Todo programa ejecutable de Rust tiene una función `main` en `src/main.rs`.
// Rust es compilado, de tipado estático y fuerte, y SIN recolector de basura:
// la memoria se libera de forma determinista cuando una variable sale de su
// ámbito (lo veremos a fondo en el módulo 04).
//
// `println!` lleva `!` porque es una MACRO, no una función: se expande en
// tiempo de compilación y comprueba que los `{}` coincidan con los argumentos.

// -----------------------------------------------------------------------------
// CONSTANTES y estáticos a nivel de módulo
// -----------------------------------------------------------------------------
// `const`: valor calculado en compilación y copiado en cada uso. El tipo es
// OBLIGATORIO y el nombre va en MAYÚSCULAS_CON_GUIONES (convención).
const IVA: f64 = 0.21;
const KB: u64 = 1 << 10;
const MB: u64 = KB << 10;
const GB: u64 = MB << 10;

// `static`: una única posición en memoria durante todo el programa.
static VERSION: &str = "1.0";

// Rust no tiene `iota`: los "enums" son enums de verdad (módulo 05).
// Aun así se les puede asignar un valor numérico (discriminante):
#[derive(Debug, Clone, Copy)]
enum DiaSemana {
    Lunes = 1,
    Martes,
    Miercoles,
    Jueves,
    Viernes,
}

fn main() {
    println!("===== 1. Variables y mutabilidad =====");
    variables();

    println!("\n===== 2. Tipos escalares =====");
    tipos_escalares();

    println!("\n===== 3. Tipos compuestos: tuplas y arrays =====");
    tipos_compuestos();

    println!("\n===== 4. Conversiones de tipo =====");
    conversiones();

    println!("\n===== 5. Constantes y enums con valor =====");
    println!(
        "IVA: {IVA} | Miércoles: {:?} = {} | KB: {KB} MB: {MB} GB: {GB} | versión {VERSION}",
        DiaSemana::Miercoles,
        DiaSemana::Miercoles as i32
    );
    let _ = (
        DiaSemana::Lunes,
        DiaSemana::Martes,
        DiaSemana::Jueves,
        DiaSemana::Viernes,
    );

    println!("\n===== 6. Strings, &str y char =====");
    strings_y_chars();

    println!("\n===== 7. Formato con println! =====");
    formato();
}

fn variables() {
    // `let` declara una variable. Por defecto es INMUTABLE.
    let nombre = "Ferris"; // el compilador infiere &str
    let edad: u32 = 10; // tipo explícito

    // nombre = "otro";  // ERROR: cannot assign twice to immutable variable

    // `mut` la hace mutable. Rust te obliga a declarar la intención.
    let mut contador = 0;
    contador += 1;
    contador += 1;

    println!("{nombre} tiene {edad} años; contador = {contador}");

    // SHADOWING: declarar otra variable con el MISMO nombre. Es una variable
    // NUEVA (puede cambiar incluso de tipo). Muy usado para transformar datos.
    let entrada = "  42  ";
    let entrada = entrada.trim(); // &str sin espacios
    let entrada: i32 = entrada.parse().unwrap(); // ahora es un i32
    println!("shadowing: entrada = {entrada} (ahora es un número)");

    // Rust NO tiene "valores cero" como Go: usar una variable sin inicializar
    // no compila. Pero se puede declarar y asignar después, una sola vez.
    // (Clippy sugiere usar `if` como expresión, que veremos en el módulo 02.)
    #[allow(clippy::needless_late_init)]
    let resultado;
    if edad > 5 {
        resultado = "mayor de 5";
    } else {
        resultado = "5 o menos";
    }
    println!("resultado = {resultado}");

    // Las variables que no usas generan un AVISO (no un error como en Go).
    // Prefijo `_` para indicar que es intencionado.
    let _sin_usar = 3;
}

fn tipos_escalares() {
    // Enteros con signo: i8 i16 i32 i64 i128 isize
    // Enteros sin signo: u8 u16 u32 u64 u128 usize (usize = tamaño de puntero,
    //                    se usa para índices y longitudes)
    let a: i32 = -42; // i32 es el entero por defecto
    let b: u8 = 255;
    let c = 1_000_000_u64; // `_` como separador y sufijo de tipo
    let hex = 0xff;
    let bin = 0b1010;
    println!("a={a} b={b} c={c} hex={hex} bin={bin}");

    // Desbordamiento: en modo debug, `255u8 + 1` hace PANIC. En release da la
    // vuelta. Si quieres un comportamiento concreto, dilo explícitamente:
    println!(
        "checked: {:?} | wrapping: {} | saturating: {}",
        b.checked_add(1),    // None
        b.wrapping_add(1),   // 0
        b.saturating_add(1)  // 255
    );

    // Flotantes: f32 y f64 (por defecto)
    let x = 2.5;
    let y: f32 = 1.0 / 3.0;
    println!("x={x} y={y} raíz de 2={}", 2.0_f64.sqrt());

    // Booleanos
    let activo: bool = true;
    println!("activo && !false = {}", activo && !false);

    // char: un valor Unicode de 4 bytes (¡no un byte!)
    let letra = 'ñ';
    let emoji = '🦀';
    println!("char: {letra} {emoji} ocupa {} bytes", size_of::<char>());

    // El tipo unidad `()`: "nada". Es lo que devuelven las funciones sin valor.
    let unidad: () = ();
    println!("unidad = {unidad:?}");
}

fn tipos_compuestos() {
    // Tupla: tamaño fijo, tipos distintos. Se accede con .0, .1...
    let persona: (&str, u32, f64) = ("Ana", 30, 1.68);
    println!(
        "tupla: {} tiene {} años y mide {}",
        persona.0, persona.1, persona.2
    );

    // Desestructuración
    let (nombre, edad, _) = persona;
    println!("desestructurada: {nombre}, {edad}");

    // Array: tamaño FIJO conocido en compilación, mismo tipo. [T; N]
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];
    let ceros = [0u8; 4]; // cuatro ceros
    println!("array: {numeros:?}, len={}, ceros={ceros:?}", numeros.len());

    // Acceder fuera de rango hace PANIC (no hay memoria corrupta, nunca):
    // numeros[10];  // error de compilación si el índice es constante
    // `.get()` devuelve Option: Some(valor) o None
    println!("get(1)={:?} get(10)={:?}", numeros.get(1), numeros.get(10));
}

fn conversiones() {
    // NO hay conversiones implícitas. Ni siquiera de i32 a i64.
    let entero: i32 = 10;
    let flotante: f64 = 3.99;

    // `as`: conversión primitiva. Rápida pero puede perder información.
    let a = entero as f64 / 3.0;
    let b = flotante as i32; // trunca: 3
    let c = 300_i32 as u8; // ¡44! (300 mod 256) — cuidado
    println!("as: {a:.3} {b} {c}");

    // `From`/`Into`: conversiones que NUNCA fallan (sin pérdida)
    let grande: i64 = i64::from(entero);
    let tambien: f64 = entero.into();
    println!("from/into: {grande} {tambien}");

    // `TryFrom`/`TryInto`: conversiones que pueden fallar → Result
    let ok: Result<u8, _> = u8::try_from(200_i32);
    let err: Result<u8, _> = u8::try_from(300_i32);
    println!("try_from: {ok:?} {err:?}");

    // De texto a número: `parse` (también devuelve Result)
    let n: i32 = "123".parse().unwrap();
    let mal = "12a".parse::<i32>(); // "turbofish" ::<T> para indicar el tipo
    println!("parse: {n} {mal:?}");

    // De número a texto
    let s = 65.to_string();
    let ch = char::from(65u8); // 'A'
    println!("to_string: {s:?} | char::from(65) = {ch:?}");
}

fn strings_y_chars() {
    // Hay DOS tipos de texto principales:
    //   &str   → "slice de string": una vista prestada e inmutable a texto UTF-8
    //            (los literales "..." son &'static str)
    //   String → texto propio, en el heap, que puede crecer
    let literal: &str = "Hola";
    let mut propio: String = String::from("Hola");
    propio.push_str(", mundo");
    propio.push('!');
    println!("{literal} | {propio}");

    // len() cuenta BYTES, no caracteres (UTF-8)
    let s = "año 🦀";
    println!("{s:?}: {} bytes, {} chars", s.len(), s.chars().count());

    // No se puede indexar s[0]: un byte no es necesariamente un carácter.
    // Recorre por chars o por bytes según necesites:
    for (i, c) in s.char_indices() {
        print!("[{i}:{c}] ");
    }
    println!();

    // Slices por rango de BYTES (debe caer en frontera de carácter o hay panic)
    println!("&s[0..3] = {:?}", &s[0..3]); // "añ" ocupa 3 bytes

    // Concatenar
    let a = String::from("foo");
    let b = "bar";
    let c = a + b; // `a` se MUEVE dentro de c (ya no se puede usar `a`)
    let d = format!("{c}-{b}-{}", 42); // format! no mueve nada
    println!("c={c} d={d}");

    // Strings "raw": sin escapes. Útiles para regex o JSON.
    let raw = r#"{"clave": "valor\n"}"#;
    println!("raw: {raw}");
}

#[allow(clippy::print_literal)] // aquí los literales son a propósito
fn formato() {
    let nombre = "Rust";
    let pi = std::f64::consts::PI;
    let v = vec![1, 2, 3];
    let tupla = (1, "dos");

    println!("{} y {}", nombre, 2024); // posicional
    println!("{0} {1} {0}", "a", "b"); // por índice
    println!("{nombre} capturado del entorno"); // variables en línea
    println!("{:?} {:?}", v, tupla); // Debug (para programadores)
    println!("{:#?}", tupla); // Debug "bonito" multilínea
    println!("[{:>8}] [{:<8}] [{:^8}]", "der", "izq", "centro"); // alineación
    println!("[{:08.3}] [{:+}] [{:e}]", pi, 5, 1234.5); // relleno, signo, científico
    println!("{:x} {:X} {:o} {:b} {:#x}", 255, 255, 8, 5, 255); // bases
    println!("{{ llaves literales }}");
    eprintln!("(esto va a stderr)");
}
