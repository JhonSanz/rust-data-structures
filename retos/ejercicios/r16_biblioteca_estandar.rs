// =============================================================================
// RETOS 16 — BIBLIOTECA ESTÁNDAR                 (repasa: 16_biblioteca_estandar)
// Comprueba tu solución:   cargo test -p retos --test r16
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.
// Regla del módulo: usa `?` para propagar los errores de E/S, sin unwrap.

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// Directorio temporal único por test (se borra al terminar).
struct DirTemporal(PathBuf);
impl DirTemporal {
    fn nuevo(nombre: &str) -> Self {
        let d = std::env::temp_dir().join(format!("retos-r16-{nombre}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        DirTemporal(d)
    }
}
impl Drop for DirTemporal {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// RETO 1 — Leer un archivo línea a línea
/// Devuelve (número de líneas, número de palabras) del archivo.
/// Requisito: léelo con BufReader y `lines()` (no cargues todo en memoria).
fn contar_lineas_y_palabras(ruta: &Path) -> io::Result<(usize, usize)> {
    todo!()
}

#[test]
fn reto_1_contar_lineas_y_palabras() -> io::Result<()> {
    let dir = DirTemporal::nuevo("contar");
    let ruta = dir.0.join("texto.txt");
    fs::write(&ruta, "hola mundo\n\nRust es  genial\n")?;
    assert_eq!(contar_lineas_y_palabras(&ruta)?, (3, 5));
    let err = contar_lineas_y_palabras(&dir.0.join("no_existe.txt")).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::NotFound);
    Ok(())
}

/// RETO 2 — Escribir con buffer
/// Escribe un CSV con la cabecera "nombre,edad" y una línea por fila.
/// Requisito: usa BufWriter y llama a `flush()` al final.
fn escribir_csv(ruta: &Path, filas: &[(&str, u32)]) -> io::Result<()> {
    todo!()
}

#[test]
fn reto_2_escribir_csv() -> io::Result<()> {
    let dir = DirTemporal::nuevo("csv");
    let ruta = dir.0.join("gente.csv");
    escribir_csv(&ruta, &[("Ana", 30), ("Luis", 25)])?;
    assert_eq!(fs::read_to_string(&ruta)?, "nombre,edad\nAna,30\nLuis,25\n");
    Ok(())
}

/// RETO 3 — Formatear una duración
///   3723 s → "1h 2m 3s"    120 s → "2m 0s"    59 s → "59s"    3600 s → "1h 0m 0s"
/// Las horas solo aparecen si son > 0; los minutos, si hay horas o minutos > 0.
/// Se ignoran las fracciones de segundo.
fn duracion_legible(d: Duration) -> String {
    todo!()
}

#[test]
fn reto_3_duracion_legible() {
    assert_eq!(duracion_legible(Duration::from_secs(3723)), "1h 2m 3s");
    assert_eq!(duracion_legible(Duration::from_secs(120)), "2m 0s");
    assert_eq!(duracion_legible(Duration::from_millis(59_999)), "59s");
    assert_eq!(duracion_legible(Duration::from_secs(3600)), "1h 0m 0s");
    assert_eq!(duracion_legible(Duration::ZERO), "0s");
}

/// RETO 4 — Ordenar por varios criterios
/// Ordena por edad DESCENDENTE y, a igual edad, por nombre ASCENDENTE.
/// Pista: `sort_by` + `cmp` + `then_with`.
fn ordenar_personas(personas: &mut [(String, u32)]) {
    todo!()
}

#[test]
fn reto_4_ordenar_personas() {
    let mut v: Vec<(String, u32)> = [("Luis", 25), ("Eva", 30), ("Ana", 30), ("Zoe", 40)]
        .iter()
        .map(|(n, e)| (n.to_string(), *e))
        .collect();
    ordenar_personas(&mut v);
    let nombres: Vec<&str> = v.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(nombres, ["Zoe", "Ana", "Eva", "Luis"]);
}

/// RETO 5 — Ejecutar procesos
/// Ejecuta `programa` con `args` y devuelve (código de salida, stdout sin
/// espacios ni saltos de línea en los extremos). Si no hay código (el proceso
/// murió por una señal), usa -1.
fn ejecutar(programa: &str, args: &[&str]) -> io::Result<(i32, String)> {
    todo!()
}

#[test]
fn reto_5_ejecutar() -> io::Result<()> {
    assert_eq!(
        ejecutar("echo", &["hola", "mundo"])?,
        (0, "hola mundo".to_string())
    );
    assert_eq!(
        ejecutar("sh", &["-c", "echo uy; exit 3"])?,
        (3, "uy".to_string())
    );
    assert_eq!(
        ejecutar("programa-que-no-existe-xyz", &[])
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    Ok(())
}

/// RETO 6 — Variables de entorno con valor por defecto
fn variable_o(nombre: &str, por_defecto: &str) -> String {
    todo!()
}

#[test]
fn reto_6_variable_o() {
    assert_eq!(
        variable_o("VARIABLE_QUE_NO_EXISTE_XYZ", "por defecto"),
        "por defecto"
    );
    assert_eq!(variable_o("CARGO_PKG_NAME", "?"), "retos"); // Cargo la define al ejecutar tests
}
