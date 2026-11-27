// =============================================================================
// MÓDULO 08 — MANEJO DE ERRORES
// Ejecuta con:  cargo run -p errores
// =============================================================================
//
// Rust distingue dos tipos de error:
//   - IRRECUPERABLES → panic!  (un bug: índice fuera de rango, invariante rota)
//   - RECUPERABLES   → Result<T, E>  (archivo que no existe, entrada inválida)
//
//   enum Result<T, E> { Ok(T), Err(E) }
//
// No hay excepciones ni try/catch. Los errores son VALORES que forman parte
// del tipo de retorno, y el compilador avisa si ignoras un Result
// (#[must_use]). El operador `?` los propaga sin el `if err != nil` de Go.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

fn main() {
    println!("===== 1. Result básico =====");
    for entrada in ["42", "-7", "abc", ""] {
        match entrada.parse::<i32>() {
            Ok(n) => println!("{entrada:?} → Ok({n})"),
            Err(e) => println!("{entrada:?} → Err: {e}"),
        }
    }

    println!("\n===== 2. El operador ? =====");
    println!("sumar_textos(\"2\", \"3\") = {:?}", sumar_textos("2", "3"));
    println!("sumar_textos(\"2\", \"x\") = {:?}", sumar_textos("2", "x"));

    println!("\n===== 3. Métodos de Result =====");
    metodos_result();

    println!("\n===== 4. Errores propios con enum =====");
    let mut banco = Banco::default();
    banco.cuentas.insert("ana".into(), 100);
    for (de, a, cantidad) in [
        ("ana", "luis", 30),
        ("ana", "luis", 500),
        ("zoe", "luis", 1),
        ("ana", "luis", 0),
    ] {
        match banco.transferir(de, a, cantidad) {
            Ok(()) => println!("✅ {de} → {a}: {cantidad}"),
            Err(e) => println!("❌ {e}"),
        }
    }

    println!("\n===== 5. Comprobar qué error es (como errors.Is / errors.As) =====");
    if let Err(e) = banco.transferir("zoe", "ana", 5) {
        // Con enums basta un match o `matches!`
        if matches!(e, ErrorBanco::CuentaNoExiste(_)) {
            println!("es CuentaNoExiste → ofrecer crear la cuenta");
        }
    }

    println!("\n===== 6. Conversión automática con From y Box<dyn Error> =====");
    for cfg in ["puerto=8080", "puerto=abc", "host=x"] {
        match leer_puerto(cfg) {
            Ok(p) => println!("{cfg:?} → puerto {p}"),
            Err(e) => {
                println!("{cfg:?} → error: {e}");
                // La cadena de causas (como el wrapping %w de Go)
                let mut causa = e.source();
                while let Some(c) = causa {
                    println!("    causado por: {c}");
                    causa = c.source();
                }
            }
        }
    }
    match ejecutar_todo() {
        Ok(()) => println!("ejecutar_todo: ok"),
        Err(e) => println!("ejecutar_todo: {e}"),
    }

    println!("\n===== 7. Option y Result juntos =====");
    let datos = ["10", "20", "x", "40"];
    // collect puede convertir Vec<Result<T,E>> en Result<Vec<T>,E>: se para en el primer error
    let todos: Result<Vec<i32>, _> = datos.iter().map(|s| s.parse::<i32>()).collect();
    println!("collect en Result: {todos:?}");
    let validos: Vec<i32> = datos.iter().filter_map(|s| s.parse().ok()).collect();
    println!("solo los válidos: {validos:?}");
    println!("primer_par(&[1,3,4]) = {:?}", primer_par_doble(&[1, 3, 4]));

    println!("\n===== 8. panic! y catch_unwind =====");
    panics();
}

// `?` : si es Ok(v) devuelve v; si es Err(e), RETORNA Err(e.into()) de la función.
fn sumar_textos(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.trim().parse::<i32>()?;
    let y = b.trim().parse::<i32>()?;
    Ok(x + y)
}

fn metodos_result() {
    let ok: Result<i32, String> = Ok(3);
    let err: Result<i32, String> = Err("falló".into());

    println!(
        "unwrap_or:      {} {}",
        ok.clone().unwrap_or(0),
        err.clone().unwrap_or(0)
    );
    println!(
        "unwrap_or_else: {}",
        err.clone().unwrap_or_else(|e| e.len() as i32)
    );
    println!("map:            {:?}", ok.clone().map(|n| n * 10));
    println!(
        "map_err:        {:?}",
        err.clone().map_err(|e| e.to_uppercase())
    );
    println!(
        "and_then:       {:?}",
        ok.clone()
            .and_then(|n| if n > 0 { Ok(n) } else { Err("negativo".into()) })
    );
    println!(
        "ok():           {:?} {:?}",
        ok.clone().ok(),
        err.clone().ok()
    );
    println!("is_ok/is_err:   {} {}", ok.is_ok(), err.is_err());
    // unwrap()/expect() hacen panic si hay error. Úsalos en tests, prototipos,
    // o cuando un error sería un bug (y documenta por qué con expect).
}

// --- Error propio: un enum con una variante por cada caso ---------------------
#[derive(Debug, PartialEq)]
enum ErrorBanco {
    CuentaNoExiste(String),
    SaldoInsuficiente { disponible: u64, necesario: u64 },
    CantidadInvalida,
}

// Display: el mensaje para humanos
impl fmt::Display for ErrorBanco {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorBanco::CuentaNoExiste(c) => write!(f, "la cuenta '{c}' no existe"),
            ErrorBanco::SaldoInsuficiente {
                disponible,
                necesario,
            } => {
                write!(
                    f,
                    "saldo insuficiente: hay {disponible}, se necesitan {necesario}"
                )
            }
            ErrorBanco::CantidadInvalida => write!(f, "la cantidad debe ser mayor que 0"),
        }
    }
}

// std::error::Error: lo convierte en un error "estándar" (compatible con Box<dyn Error>)
impl Error for ErrorBanco {}

#[derive(Default)]
struct Banco {
    cuentas: HashMap<String, u64>,
}

impl Banco {
    fn transferir(&mut self, de: &str, a: &str, cantidad: u64) -> Result<(), ErrorBanco> {
        if cantidad == 0 {
            return Err(ErrorBanco::CantidadInvalida);
        }
        let saldo = *self
            .cuentas
            .get(de)
            .ok_or_else(|| ErrorBanco::CuentaNoExiste(de.into()))?;
        if saldo < cantidad {
            return Err(ErrorBanco::SaldoInsuficiente {
                disponible: saldo,
                necesario: cantidad,
            });
        }
        *self.cuentas.get_mut(de).unwrap() -= cantidad;
        *self.cuentas.entry(a.into()).or_insert(0) += cantidad;
        Ok(())
    }
}

// --- Error que envuelve otros errores (con causa) -----------------------------
#[derive(Debug)]
enum ErrorConfig {
    FaltaClave(&'static str),
    ValorInvalido {
        clave: &'static str,
        fuente: ParseIntError,
    },
}

impl fmt::Display for ErrorConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorConfig::FaltaClave(k) => write!(f, "falta la clave '{k}'"),
            ErrorConfig::ValorInvalido { clave, .. } => write!(f, "valor inválido para '{clave}'"),
        }
    }
}

impl Error for ErrorConfig {
    // source() expone el error original: así se encadenan las causas
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorConfig::ValorInvalido { fuente, .. } => Some(fuente),
            _ => None,
        }
    }
}

fn leer_puerto(cfg: &str) -> Result<u16, ErrorConfig> {
    let (clave, valor) = cfg
        .split_once('=')
        .ok_or(ErrorConfig::FaltaClave("puerto"))?;
    if clave != "puerto" {
        return Err(ErrorConfig::FaltaClave("puerto"));
    }
    valor
        .parse::<u16>()
        .map_err(|e| ErrorConfig::ValorInvalido {
            clave: "puerto",
            fuente: e,
        })
}

// Box<dyn Error>: "cualquier error". Cómodo en main o en aplicaciones, donde
// solo quieres propagar y mostrar. `?` convierte automáticamente cualquier
// tipo que implemente Error, gracias a `impl From<E: Error> for Box<dyn Error>`.
// (En proyectos reales se suelen usar los crates `thiserror` para librerías
// y `anyhow` para aplicaciones; aquí lo hacemos a mano para ver cómo funciona.)
fn ejecutar_todo() -> Result<(), Box<dyn Error>> {
    let n: i32 = "5".parse()?; // ParseIntError → Box<dyn Error>
    let puerto = leer_puerto("puerto=9000")?; // ErrorConfig → Box<dyn Error>
    let mut banco = Banco::default();
    banco.transferir("nadie", "x", n as u64)?; // ErrorBanco → Box<dyn Error>
    println!("nunca llega aquí ({puerto})");
    Ok(())
}

// `?` también funciona con Option dentro de funciones que devuelven Option
fn primer_par_doble(v: &[i32]) -> Option<i32> {
    let par = v.iter().find(|&&x| x % 2 == 0)?;
    Some(par * 2)
}

fn panics() {
    // Un panic desenrolla la pila del hilo (ejecutando los Drop) y termina el
    // hilo. Si es el hilo principal, termina el programa con código 101.
    //
    // catch_unwind existe para casos MUY concretos (FFI, frameworks que aíslan
    // tareas); NO es un try/catch para el control de flujo normal.
    std::panic::set_hook(Box::new(|info| {
        println!("  (hook) panic capturado: {info}")
    }));
    let resultado = std::panic::catch_unwind(|| {
        let v: Vec<i32> = vec![];
        v[0] // index out of bounds → panic
    });
    println!("catch_unwind → is_err = {}", resultado.is_err());
    let _ = std::panic::take_hook();

    // Cuándo usar panic: invariantes que, si fallan, indican un BUG.
    //   - unreachable!()     código que no debería alcanzarse
    //   - todo!()            "sin implementar todavía" (compila, falla al ejecutar)
    //   - assert!(cond)      comprobaciones (también en release)
    //   - debug_assert!()    solo en debug
    let x = 4;
    assert!(x % 2 == 0, "x debe ser par, es {x}");
    println!("assert! superado");
}
