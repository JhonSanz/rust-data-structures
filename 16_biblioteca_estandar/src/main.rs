// =============================================================================
// MÓDULO 16 — BIBLIOTECA ESTÁNDAR ESENCIAL
// Ejecuta con:  cargo run -p biblioteca_estandar -- argumento1 argumento2
// =============================================================================
//
// La std de Rust es deliberadamente PEQUEÑA comparada con la de Go: no trae
// JSON, regex, fechas de calendario, HTTP ni logging. Esas piezas viven en
// crates del ecosistema que son estándar de facto (ver el README). Aquí está
// lo que SÍ trae y usarás a diario.

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fmt::Write as _; // trae el trait (para write! sobre String) sin ocupar el nombre
use std::fs::{self, File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() -> io::Result<()> {
    // main puede devolver Result: si es Err, se imprime y el proceso sale con 1.
    println!("===== 1. std::env: argumentos y variables =====");
    entorno();

    println!("\n===== 2. Texto: parseo, char y fmt::Write =====");
    texto();

    println!("\n===== 3. std::time =====");
    tiempo();

    println!("\n===== 4. Archivos: fs, io, path =====");
    archivos()?;

    println!("\n===== 5. std::process: ejecutar comandos =====");
    procesos();

    println!("\n===== 6. std::mem y std::cmp =====");
    mem_y_cmp();

    println!("\n===== 7. Leer de stdin =====");
    stdin()?;
    Ok(())
}

fn entorno() {
    // El primer argumento es el nombre del programa
    let args: Vec<String> = env::args().collect();
    println!("programa: {}", args[0]);
    println!("argumentos: {:?}", &args[1..]);
    // Un mini parser de flags (en proyectos reales: el crate `clap`)
    let verbose = args.iter().any(|a| a == "-v" || a == "--verbose");
    println!("¿--verbose? {verbose}");

    // env::var devuelve Result (Err si no existe o no es UTF-8)
    let home = env::var("HOME").unwrap_or_else(|_| "(sin HOME)".into());
    println!("HOME = {home}");
    println!("¿existe NO_EXISTE? {}", env::var("NO_EXISTE").is_ok());
    println!(
        "directorio actual = {}",
        env::current_dir().unwrap().display()
    );

    // Variables de Cargo en TIEMPO DE COMPILACIÓN
    println!(
        "compilado como {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!(
        "SO: {} / arquitectura: {}",
        env::consts::OS,
        env::consts::ARCH
    );
}

fn texto() {
    // Parseo: FromStr está implementado para todos los tipos numéricos, bool, char...
    let a: f64 = "3.14".parse().unwrap();
    let b = i64::from_str_radix("ff", 16).unwrap();
    let c = u8::from_str_radix("1010", 2).unwrap();
    let d: bool = "true".parse().unwrap();
    println!("parse: {a} {b} {c} {d}");

    // char
    for ch in ['a', 'Ñ', '7', ' ', '🦀'] {
        println!(
            "  {ch:?}: alfabético={} numérico={} espacio={} mayúscula={} → {:?}, u32={:#x}",
            ch.is_alphabetic(),
            ch.is_numeric(),
            ch.is_whitespace(),
            ch.is_uppercase(),
            ch.to_lowercase().to_string(),
            ch as u32
        );
    }

    // Construir strings eficientemente (como strings.Builder): con capacidad
    // reservada y write! (trait fmt::Write)
    let mut s = String::with_capacity(64);
    for i in 1..=5 {
        write!(s, "{i}²={} ", i * i).unwrap();
    }
    println!("builder: {s}");

    // Escapes y formato de depuración
    println!("debug escapa: {:?}", "línea1\n\t\"comillas\"");
    println!("bytes: {:?}", "ñ".as_bytes());
    println!("desde bytes: {:?}", String::from_utf8(vec![0xc3, 0xb1]));
    println!(
        "lossy: {}",
        String::from_utf8_lossy(&[0x68, 0x6f, 0xff, 0x6c, 0x61])
    );
}

fn tiempo() {
    // Instant: reloj MONOTÓNICO, para medir duraciones (no se ve afectado si
    // cambian la hora del sistema)
    let inicio = Instant::now();
    std::thread::sleep(Duration::from_millis(15));
    println!("transcurrido: {:?}", inicio.elapsed());

    // Duration
    let d = Duration::from_secs(90) + Duration::from_millis(250);
    println!(
        "duración: {d:?} = {} s = {} ms",
        d.as_secs_f64(),
        d.as_millis()
    );
    println!(
        "checked_sub: {:?}",
        Duration::from_secs(1).checked_sub(Duration::from_secs(2))
    );

    // SystemTime: reloj de pared. La std solo da segundos desde 1970 (epoch);
    // para fechas de calendario, zonas horarias y formatos usa `chrono` o `jiff`.
    let ahora = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let (anio, mes, dia) = fecha_civil((ahora.as_secs() / 86_400) as i64);
    let segs_dia = ahora.as_secs() % 86_400;
    println!(
        "epoch = {} s → {anio:04}-{mes:02}-{dia:02} {:02}:{:02}:{:02} UTC (calculado a mano)",
        ahora.as_secs(),
        segs_dia / 3600,
        segs_dia % 3600 / 60,
        segs_dia % 60
    );
}

// Algoritmo de Howard Hinnant: días desde 1970-01-01 → (año, mes, día).
// Solo para demostrar que la std no trae calendario: ¡no lo hagas en producción!
fn fecha_civil(dias: i64) -> (i64, u32, u32) {
    let z = dias + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

fn archivos() -> io::Result<()> {
    // Un directorio temporal propio para la demo
    let dir: PathBuf = env::temp_dir().join(format!("tour-rust-{}", std::process::id()));
    fs::create_dir_all(&dir)?;

    // Path/PathBuf: rutas multiplataforma (como &str/String, pero para rutas)
    let ruta = dir.join("notas.txt");
    println!("ruta: {}", ruta.display());
    println!(
        "  nombre={:?} extensión={:?} padre={:?}",
        ruta.file_name(),
        ruta.extension(),
        ruta.parent().map(Path::file_name)
    );

    // Escribir/leer TODO de una vez (lo más simple)
    fs::write(&ruta, "línea 1\nlínea 2\n")?;
    let contenido = fs::read_to_string(&ruta)?;
    println!("read_to_string: {contenido:?}");

    // Añadir al final con OpenOptions
    let mut f = OpenOptions::new().append(true).open(&ruta)?;
    writeln!(f, "línea 3 (añadida)")?;
    // No hay que cerrar: el archivo se cierra al salir de su ámbito (Drop).
    drop(f);

    // Leer línea a línea con buffer (eficiente para archivos grandes)
    let lector = BufReader::new(File::open(&ruta)?);
    for (i, linea) in lector.lines().enumerate() {
        println!("  {}: {}", i + 1, linea?);
    }

    // Escribir con buffer: agrupa muchas escrituras pequeñas en pocas syscalls
    let csv = dir.join("datos.csv");
    {
        let mut w = BufWriter::new(File::create(&csv)?);
        for i in 0..1000 {
            writeln!(w, "{i},{}", i * i)?;
        }
        w.flush()?; // si no, el Drop hace flush pero IGNORA los errores
    }

    // Metadatos
    let meta = fs::metadata(&csv)?;
    println!(
        "datos.csv: {} bytes, ¿es archivo? {}",
        meta.len(),
        meta.is_file()
    );

    // Leer bytes con el trait Read
    let mut cabecera = [0u8; 8];
    File::open(&csv)?.read_exact(&mut cabecera)?;
    println!("primeros 8 bytes: {:?}", String::from_utf8_lossy(&cabecera));

    // Listar un directorio
    let mut nombres: Vec<String> = fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    nombres.sort();
    println!("contenido del directorio: {nombres:?}");

    // Errores de E/S: io::ErrorKind para distinguir casos
    match fs::read_to_string(dir.join("no_existe.txt")) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => println!("no_existe.txt: NotFound ({e})"),
        Err(e) => println!("otro error: {e}"),
        Ok(_) => unreachable!(),
    }

    fs::remove_dir_all(&dir)?;
    println!("¿existe tras borrar? {}", dir.exists());
    Ok(())
}

fn procesos() {
    // Ejecutar un programa externo y capturar su salida
    match Command::new("uname").arg("-sr").output() {
        Ok(out) if out.status.success() => {
            println!(
                "uname -sr → {}",
                String::from_utf8_lossy(&out.stdout).trim()
            )
        }
        Ok(out) => println!("uname falló con {}", out.status),
        Err(e) => println!("no se pudo ejecutar uname: {e}"),
    }
    // Pasar variables de entorno y comprobar el código de salida
    let estado = Command::new("sh")
        .args(["-c", "exit 3"])
        .env("MI_VAR", "1")
        .status();
    println!(
        "sh -c 'exit 3' → código {:?}",
        estado.ok().and_then(|s| s.code())
    );
    // std::process::exit(código) termina el proceso SIN ejecutar los Drop pendientes.
}

fn mem_y_cmp() {
    // mem::swap, mem::replace, mem::take: mover valores sin clonar
    let mut a = String::from("A");
    let mut b = String::from("B");
    std::mem::swap(&mut a, &mut b);
    let viejo = std::mem::replace(&mut a, String::from("nuevo"));
    let tomado = std::mem::take(&mut b); // deja el valor Default ("") en su lugar
    println!("swap/replace/take: a={a:?} b={b:?} viejo={viejo:?} tomado={tomado:?}");
    println!(
        "size_of: u8={} u64={} String={} Option<Box<u8>>={}",
        size_of::<u8>(),
        size_of::<u64>(),
        size_of::<String>(),
        size_of::<Option<Box<u8>>>()
    );

    // cmp: Ordering, min, max, clamp
    let orden = 3.cmp(&5);
    println!("3.cmp(&5) = {orden:?}, ¿Less? {}", orden == Ordering::Less);
    println!(
        "clamp(15, 0, 10) = {}, max(2, 9) = {}",
        15.clamp(0, 10),
        std::cmp::max(2, 9)
    );
    // Ordenar por varios criterios con then_with
    let mut gente = vec![("Ana", 30), ("Luis", 25), ("Eva", 30)];
    gente.sort_by(|x, y| y.1.cmp(&x.1).then_with(|| x.0.cmp(y.0)));
    println!("por edad desc y nombre: {gente:?}");
    // f64 no es Ord (por NaN): usa total_cmp
    let mut fs = vec![2.5, -1.0, 9.75];
    fs.sort_by(f64::total_cmp);
    println!("f64 ordenados: {fs:?}");

    // Hash de cualquier valor Hash
    let mut h = DefaultHasher::new();
    ("clave", 42).hash(&mut h);
    println!("hash de (\"clave\", 42) = {:x}", h.finish());
}

fn stdin() -> io::Result<()> {
    // Si stdin es una terminal no esperamos (para que el tour no se quede parado).
    // Prueba:  echo -e "hola\nmundo" | cargo run -p biblioteca_estandar
    use std::io::IsTerminal;
    if io::stdin().is_terminal() {
        println!(
            "(stdin es una terminal: prueba con  echo -e \"hola\\nmundo\" | cargo run -p biblioteca_estandar)"
        );
        return Ok(());
    }
    let mut total = 0;
    for linea in io::stdin().lock().lines() {
        let linea = linea?;
        total += 1;
        println!("  stdin: {}", linea.to_uppercase());
    }
    println!("líneas leídas: {total}");
    Ok(())
}
