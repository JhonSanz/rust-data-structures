// =============================================================================
// TESTS DE INTEGRACIÓN (carpeta tests/)
// =============================================================================
// Cada archivo de tests/ es un crate APARTE que usa la biblioteca como un
// usuario externo: solo ve la API pública (`testing::...`).
//
//   cargo test -p testing --test integracion     → solo este archivo
//
// (Solo los paquetes con src/lib.rs pueden tener tests de integración; por eso
// los proyectos suelen poner la lógica en lib.rs y dejar main.rs mínimo.)

use std::fs;
use std::path::PathBuf;
use testing::{ErrorVacio, contar_palabras, es_palindromo, invertir};

#[test]
fn api_publica_combinada() {
    let frase = "Dábale arroz a la zorra el abad";
    // es_palindromo no quita tildes: 'á' != 'a'
    assert!(!es_palindromo(frase));
    assert!(es_palindromo(&frase.replace('á', "a")));
    assert_eq!(contar_palabras(frase), Ok(7));
}

#[test]
fn el_error_implementa_display() {
    let err = contar_palabras("").unwrap_err();
    assert_eq!(err, ErrorVacio);
    assert_eq!(err.to_string(), "texto vacío");
}

// Recursos temporales: no hay t.TempDir() en std, así que creamos un
// directorio único y lo borramos con un guard que implementa Drop (se ejecuta
// aunque el test falle). En proyectos reales: el crate `tempfile`.
struct DirTemporal(PathBuf);

impl DirTemporal {
    fn nuevo(nombre: &str) -> Self {
        let ruta = std::env::temp_dir().join(format!("tour-rust-{nombre}-{}", std::process::id()));
        fs::create_dir_all(&ruta).unwrap();
        DirTemporal(ruta)
    }
}

impl Drop for DirTemporal {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn procesar_archivo() {
    let dir = DirTemporal::nuevo("archivo");
    let ruta = dir.0.join("entrada.txt");
    fs::write(&ruta, "reconocer\nhola\nsalas\n").unwrap();

    let contenido = fs::read_to_string(&ruta).unwrap();
    let palindromos: Vec<&str> = contenido.lines().filter(|l| es_palindromo(l)).collect();
    assert_eq!(palindromos, ["reconocer", "salas"]);

    let invertido: Vec<String> = contenido.lines().map(invertir).collect();
    assert_eq!(invertido[1], "aloh");
} // ← aquí `dir` se destruye y borra el directorio
