// =============================================================================
// MÓDULO 02 — CONTROL DE FLUJO
// Ejecuta con:  cargo run -p control_de_flujo
// =============================================================================
//
// La idea más importante de este módulo: en Rust casi TODO es una EXPRESIÓN
// (produce un valor). `if`, `match`, `loop` y los bloques `{ }` devuelven
// valores. Por eso no existe el operador ternario `a ? b : c`.

fn main() {
    println!("===== 1. if es una expresión =====");
    if_expresion(7);
    if_expresion(-3);

    println!("\n===== 2. loop, while y for =====");
    bucles();

    println!("\n===== 3. Rangos =====");
    rangos();

    println!("\n===== 4. Etiquetas en bucles =====");
    etiquetas();

    println!("\n===== 5. match =====");
    for n in [0, 1, 7, 15, 42, -5] {
        println!("{n:>3} → {}", clasificar(n));
    }

    println!("\n===== 6. if let, let else y while let =====");
    if_let_y_let_else();

    println!("\n===== 7. Bloques como expresiones =====");
    bloques();
}

fn if_expresion(n: i32) {
    // La condición DEBE ser bool (no vale `if n {}` con un número).
    // No lleva paréntesis; las llaves son obligatorias.
    if n > 0 {
        println!("{n} es positivo");
    } else if n < 0 {
        println!("{n} es negativo");
    } else {
        println!("es cero");
    }

    // Como expresión: ambas ramas deben devolver el MISMO tipo.
    // Fíjate en que la última línea de cada rama NO lleva `;`.
    let paridad = if n % 2 == 0 { "par" } else { "impar" };
    println!("{n} es {paridad}");
}

fn bucles() {
    // `loop`: bucle infinito. Se sale con `break`, que puede DEVOLVER un valor.
    let mut intentos = 0;
    let resultado = loop {
        intentos += 1;
        if intentos == 3 {
            break intentos * 10; // el valor del loop
        }
    };
    println!("loop devolvió {resultado} tras {intentos} intentos");

    // `while`: mientras se cumpla la condición
    let mut cuenta = 3;
    while cuenta > 0 {
        print!("{cuenta}... ");
        cuenta -= 1;
    }
    println!("¡despegue!");

    // `for`: recorre cualquier cosa que sea un ITERADOR (módulo 10).
    // Es el bucle más usado y el más seguro: no hay índices fuera de rango.
    let frutas = ["manzana", "pera", "uva"];
    for fruta in frutas {
        print!("{fruta} ");
    }
    println!();

    // Con índice: .enumerate() da (índice, elemento)
    for (i, fruta) in frutas.iter().enumerate() {
        print!("{i}:{fruta} ");
    }
    println!();

    // `continue` salta a la siguiente iteración
    for n in 1..=10 {
        if n % 3 != 0 {
            continue;
        }
        print!("{n} ");
    }
    println!("(múltiplos de 3)");
}

fn rangos() {
    // a..b   excluye b      a..=b  incluye b
    let excl: Vec<i32> = (1..5).collect();
    let incl: Vec<i32> = (1..=5).collect();
    println!("1..5 = {excl:?}   1..=5 = {incl:?}");

    // Al revés y con paso
    let atras: Vec<i32> = (1..=5).rev().collect();
    let pares: Vec<i32> = (0..=10).step_by(2).collect();
    println!("rev = {atras:?}   step_by(2) = {pares:?}");

    // Los rangos de char también funcionan
    let letras: String = ('a'..='e').collect();
    println!("'a'..='e' = {letras}");

    // contains
    println!("¿(1..10).contains(&5)? {}", (1..10).contains(&5));
}

fn etiquetas() {
    // Una etiqueta 'nombre permite hacer break/continue de un bucle EXTERIOR.
    'exterior: for i in 0..5 {
        for j in 0..5 {
            if i * j == 6 {
                println!("encontrado: {i} * {j} = 6");
                break 'exterior;
            }
        }
    }

    // `break` con etiqueta también puede devolver un valor desde un bloque:
    let primero_par = 'busqueda: {
        for n in [3, 7, 8, 11] {
            if n % 2 == 0 {
                break 'busqueda Some(n);
            }
        }
        None
    };
    println!("primer par: {primero_par:?}");
}

// `match` compara un valor con PATRONES. Es EXHAUSTIVO: el compilador
// comprueba que cubres todos los casos posibles (si no, no compila).
#[allow(clippy::manual_range_patterns)]
fn clasificar(n: i32) -> String {
    match n {
        0 => "cero".to_string(),
        1 | 2 | 3 => "pequeño".to_string(), // varias opciones con `|` (o 1..=3)
        4..=9 => "un dígito".to_string(),   // rango
        x if x % 2 == 0 && x > 0 => format!("par grande ({x})"), // guarda `if`
        x if x < 0 => "negativo".into(),
        _ => "impar grande".into(), // `_` = cualquier otra cosa
    }
}

fn if_let_y_let_else() {
    // `Option<T>` es Some(valor) o None (módulo 05). Rust no tiene null.
    let config: Option<&str> = Some("modo oscuro");

    // `if let`: cuando solo te interesa UN patrón
    if let Some(valor) = config {
        println!("config = {valor}");
    }

    // `let else`: desestructura o SAL (return/break/continue/panic).
    // Ideal para "validaciones tempranas" sin anidar.
    fn doble_de(texto: &str) -> Option<i32> {
        let Ok(n) = texto.parse::<i32>() else {
            println!("  {texto:?} no es un número");
            return None;
        };
        Some(n * 2)
    }
    println!("doble_de(\"21\") = {:?}", doble_de("21"));
    println!("doble_de(\"x\")  = {:?}", doble_de("x"));

    // `while let`: repite mientras el patrón encaje
    let mut pila = vec![1, 2, 3];
    while let Some(tope) = pila.pop() {
        print!("pop {tope} ");
    }
    println!();

    // Cadenas de `let` en if (edición 2024): varias condiciones encadenadas con &&
    let edad: Option<u32> = Some(20);
    if let Some(e) = edad
        && e >= 18
    {
        println!("mayor de edad ({e})");
    }

    // `matches!`: match que devuelve bool
    let c = 'k';
    println!(
        "¿'{c}' es vocal? {}",
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
    );
}

fn bloques() {
    // Un bloque `{ }` es una expresión: vale lo que vale su última línea sin `;`.
    let area = {
        let ancho = 3;
        let alto = 4;
        ancho * alto // ← sin `;` : este es el valor del bloque
    };
    println!("area = {area}");

    // Si pones `;` al final, el bloque vale `()` (unidad).
    #[allow(clippy::let_unit_value)]
    let nada = {
        let _x = 1;
    };
    println!("bloque con ; al final vale {nada:?}");

    // Rust no tiene `defer` como Go: la limpieza se hace con `Drop` (el
    // destructor se ejecuta automáticamente al salir del ámbito). Ver módulo 04.
}
