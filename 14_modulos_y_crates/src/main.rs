// =============================================================================
// MÓDULO 14 — MÓDULOS Y CRATES: el BINARIO (src/main.rs)
// Ejecuta con:  cargo run -p modulos
//               APP_ENV=produccion cargo run -p modulos --features cientifica
// =============================================================================
//
// Este binario es un crate distinto de la biblioteca (src/lib.rs). La usa
// por su nombre de paquete, `modulos`, y solo puede ver lo que es `pub`.

// `use` trae nombres al ámbito. Rutas:
//   crate::...  desde la raíz del crate ACTUAL
//   super::...  desde el módulo padre
//   self::...   desde el módulo actual
//   nombre::... un crate externo (aquí, nuestra biblioteca)
use modulos::calculadora::{Calculadora, ErrorCalculo};
use modulos::{Config, lista_cuadrados};

// Agrupar y renombrar imports
use std::collections::{BTreeMap as Ordenado, HashMap};

// Un módulo también puede definirse EN LÍNEA, dentro del mismo archivo.
mod formato {
    pub fn titulo(texto: &str) -> String {
        format!("== {} ==", texto.to_uppercase())
    }

    pub mod colores {
        // Ruta relativa al padre con `super`
        pub fn en_verde(texto: &str) -> String {
            format!("\x1b[32m{}\x1b[0m", super::titulo(texto))
        }
    }
}

fn main() {
    println!("{}", formato::titulo("módulos y crates"));
    println!("{}", formato::colores::en_verde("ok"));
    println!("versión de la biblioteca: {}", modulos::VERSION);
    println!("features: {:?}", modulos::features_activas());

    let config = Config::desde_entorno();
    println!("config: {config:?}");

    let mut calc = Calculadora::new("principal");
    calc.sumar(2.0, 3.0);
    let r = calc.dividir(10.0, 3.0).unwrap();
    println!("10 / 3 = {r:.*}", config.decimales);
    if let Err(ErrorCalculo::DivisionPorCero) = calc.dividir(1.0, 0.0) {
        println!("1 / 0 → error de división por cero");
    }

    // Solo compila con --features cientifica
    #[cfg(feature = "cientifica")]
    println!("2 ^ 10 = {}", calc.potencia(2.0, 10.0));
    #[cfg(not(feature = "cientifica"))]
    println!("(ejecuta con --features cientifica para tener potencia())");

    println!(
        "historial de '{}' ({} entradas):",
        calc.nombre,
        calc.historial().len()
    );
    for e in calc.historial().entradas() {
        println!("  {e}");
    }

    // Estas líneas NO compilarían, por privacidad:
    // calc.historial = ...;                    // campo privado
    // calc.registrar(...);                     // método privado
    // calc.historial().anadir(...);            // pub(super): solo para `calculadora`
    // modulos::config::decimales_para("x");    // módulo privado, función pub(crate)

    // Una macro exportada por la biblioteca
    println!(
        "lista_cuadrados![1, 2, 3, 4] = {:?}",
        lista_cuadrados![1, 2, 3, 4]
    );

    let mut h: HashMap<&str, i32> = HashMap::new();
    h.insert("x", 1);
    let o: Ordenado<_, _> = h.into_iter().collect();
    println!("BTreeMap renombrado: {o:?}");
}
