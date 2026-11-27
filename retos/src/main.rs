// =============================================================================
// COMPROBADOR DE RETOS
//   cargo run -p retos            → tabla de progreso de todos los módulos
//   cargo run -p retos -- r04     → salida completa de un módulo (con errores)
// =============================================================================
// Ejecuta `cargo test` sobre cada archivo de retos y resume el resultado.
// (Este programa también es un ejemplo de std::process::Command y de parseo
// de texto: echa un vistazo cuando termines el módulo 16.)

use std::collections::BTreeMap;
use std::env;
use std::process::{Command, ExitCode, Stdio};

const MODULOS: [(&str, &str); 17] = [
    ("r01", "Fundamentos"),
    ("r02", "Control de flujo"),
    ("r03", "Funciones y closures"),
    ("r04", "Ownership y borrowing"),
    ("r05", "Structs y enums"),
    ("r06", "Colecciones"),
    ("r07", "Traits"),
    ("r08", "Errores"),
    ("r09", "Genéricos y lifetimes"),
    ("r10", "Iteradores"),
    ("r11", "Smart pointers"),
    ("r12", "Concurrencia"),
    ("r13", "Async"),
    ("r14", "Módulos y macros"),
    ("r15", "Testing"),
    ("r16", "Biblioteca estándar"),
    ("r17", "API con axum"),
];

enum Resultado {
    /// reto → ¿todos sus tests pasan?
    Tests(BTreeMap<u32, bool>),
    NoCompila,
}

fn cargo() -> Command {
    // Cargo define CARGO al ejecutar `cargo run`: así usamos el mismo cargo
    Command::new(env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
}

fn ejecutar_modulo(objetivo: &str) -> Resultado {
    let salida = cargo()
        .args([
            "test", "-p", "retos", "--test", objetivo, "--color", "never", "--", "--color", "never",
        ])
        .stdin(Stdio::null())
        .output()
        .expect("no se pudo ejecutar cargo");
    let stdout = String::from_utf8_lossy(&salida.stdout);

    // Líneas del tipo:  "test reto_3_mcm ... ok"  /  "test reto_3_mcm ... FAILED"
    let mut retos: BTreeMap<u32, bool> = BTreeMap::new();
    for linea in stdout.lines() {
        let Some(resto) = linea.strip_prefix("test reto_") else {
            continue;
        };
        let Some((nombre, estado)) = resto.split_once(" ... ") else {
            continue;
        };
        let Some(Ok(n)) = nombre.split('_').next().map(str::parse::<u32>) else {
            continue;
        };
        let ok = estado.trim_end().starts_with("ok");
        let entrada = retos.entry(n).or_insert(true);
        *entrada &= ok; // un reto con varios tests solo cuenta si pasan todos
    }
    if retos.is_empty() && !salida.status.success() {
        Resultado::NoCompila
    } else {
        Resultado::Tests(retos)
    }
}

fn barra(hechos: usize, total: usize) -> String {
    let llenos = if total == 0 { 0 } else { hechos * 10 / total };
    format!("{}{}", "█".repeat(llenos), "░".repeat(10 - llenos))
}

fn main() -> ExitCode {
    // Modo detalle: delega en cargo test mostrando toda la salida
    if let Some(objetivo) = env::args().nth(1) {
        let Some((_, titulo)) = MODULOS.iter().find(|(o, _)| *o == objetivo) else {
            eprintln!("módulo desconocido: {objetivo} (usa r01 ... r17)");
            return ExitCode::FAILURE;
        };
        println!("== {objetivo}: {titulo} ==\n");
        let estado = cargo()
            .args(["test", "-p", "retos", "--test", &objetivo])
            .status()
            .expect("no se pudo ejecutar cargo");
        return if estado.success() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }

    println!("Comprobando retos (la primera vez compila las dependencias)...\n");
    println!(
        "  {:<4} {:<24} {:<12} {}",
        "", "Módulo", "Progreso", "Retos"
    );
    let (mut hechos_total, mut total) = (0, 0);
    for (objetivo, titulo) in MODULOS {
        let (progreso, detalle) = match ejecutar_modulo(objetivo) {
            Resultado::NoCompila => (
                barra(0, 1),
                "🔧 no compila — mira los errores: cargo run -p retos -- ".to_string() + objetivo,
            ),
            Resultado::Tests(retos) => {
                let hechos = retos.values().filter(|&&ok| ok).count();
                hechos_total += hechos;
                total += retos.len();
                let marcas: Vec<String> = retos
                    .iter()
                    .map(|(n, ok)| format!("{}{n}", if *ok { "✅" } else { "⬜" }))
                    .collect();
                (
                    format!("{} {hechos}/{}", barra(hechos, retos.len()), retos.len()),
                    marcas.join(" "),
                )
            }
        };
        println!("  {objetivo:<4} {titulo:<24} {progreso:<16} {detalle}");
    }
    println!(
        "\n  Total (sin contar los módulos que no compilan): {hechos_total}/{total} retos superados"
    );
    if total > 0 && hechos_total == total {
        println!("  🦀 ¡Enhorabuena!");
    }
    ExitCode::SUCCESS
}
