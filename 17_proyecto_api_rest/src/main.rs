// =============================================================================
// MÓDULO 17 — PROYECTO FINAL: API REST DE TAREAS
// =============================================================================
//
// Une casi todo lo anterior: structs, enums, traits, errores, colecciones,
// smart pointers (Arc<dyn Trait>), concurrencia (RwLock, Mutex), async,
// módulos, testing... y una base de datos.
//
// Modos de ejecución (desde la raíz del tutorial):
//
//   cargo run -p tareas_api -- --demo
//       Arranca el servidor, le hace peticiones de ejemplo con un cliente HTTP
//       mínimo y se apaga solo. Ideal para ver todo funcionando de un vistazo.
//
//   cargo run -p tareas_api -- --demo --db tareas.db
//       Lo mismo, pero guardando en SQLite. Ejecútalo dos veces: los datos persisten.
//
//   cargo run -p tareas_api [-- --db tareas.db]
//       Servidor en :8080 hasta Ctrl+C. Prueba:
//         curl -s localhost:8080/tareas
//         curl -s -X POST localhost:8080/tareas -H 'content-type: application/json' -d '{"titulo":"Aprender Rust"}'
//         curl -s localhost:8080/tareas/1
//         curl -s -X PATCH localhost:8080/tareas/1 -H 'content-type: application/json' -d '{"completada":true}'
//         curl -s -X DELETE localhost:8080/tareas/1 -i
//
//   cargo test -p tareas_api
//       Tests de la API contra AMBOS adaptadores, y del adaptador SQLite.

use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tareas_api::{Repositorio, RepositorioMemoria, RepositorioSqlite, app};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// La versión se puede fijar AL COMPILAR sin tocar el código:
//   APP_VERSION=1.2.3 cargo build --release -p tareas_api
// option_env! lee la variable en compilación; si no existe, usamos la de Cargo.toml.
// (El Dockerfile del módulo 18 lo usa para grabar la versión en el binario.)
const VERSION: &str = match option_env!("APP_VERSION") {
    Some(v) => v,
    None => env!("CARGO_PKG_VERSION"),
};

type ResultadoMain<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// Opciones de línea de comandos. Los valores por defecto vienen del entorno
/// (12-factor: en contenedores la configuración llega por variables).
/// En proyectos reales: el crate `clap` con `#[derive(Parser)]`.
struct Opciones {
    addr: String,
    db: Option<String>,
    demo: bool,
    healthcheck: bool,
}

fn leer_opciones() -> ResultadoMain<Opciones> {
    let mut o = Opciones {
        addr: std::env::var("ADDR").unwrap_or_else(|_| ":8080".into()),
        db: std::env::var("DB_PATH").ok().filter(|s| !s.is_empty()),
        demo: false,
        healthcheck: false,
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--addr" => o.addr = args.next().ok_or("--addr necesita un valor")?,
            "--db" => o.db = Some(args.next().ok_or("--db necesita un valor")?),
            "--demo" => o.demo = true,
            "--healthcheck" => o.healthcheck = true,
            "-h" | "--help" => {
                println!(
                    "uso: tareas_api [--addr :8080] [--db archivo.db] [--demo] [--healthcheck]\n\
                     env: ADDR, DB_PATH"
                );
                std::process::exit(0);
            }
            otro => return Err(format!("argumento desconocido: {otro} (prueba --help)").into()),
        }
    }
    Ok(o)
}

/// Acepta ":8080" (estilo Go: todas las interfaces) además de "127.0.0.1:8080".
fn parsear_addr(addr: &str) -> ResultadoMain<SocketAddr> {
    let completa = if addr.starts_with(':') {
        format!("0.0.0.0{addr}")
    } else {
        addr.to_string()
    };
    completa
        .parse()
        .map_err(|e| format!("dirección inválida {addr:?}: {e}").into())
}

// main solo delega en run() y decide el código de salida. Así todos los Drop
// (cerrar la BD, etc.) se ejecutan antes de salir: process::exit no los ejecuta.
#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

async fn run() -> ResultadoMain<()> {
    let opciones = leer_opciones()?;
    let addr = parsear_addr(&opciones.addr)?;

    if opciones.healthcheck {
        return comprobar_salud(addr).await;
    }

    println!("INFO arrancando version={VERSION}");
    let repo = elegir_repositorio(opciones.db.as_deref())?;
    let aplicacion = app(repo);

    if opciones.demo {
        ejecutar_demo(aplicacion).await
    } else {
        ejecutar_servidor(addr, aplicacion).await
    }
}

/// El ÚNICO lugar del programa que conoce los adaptadores concretos.
/// Todo lo demás trabaja con `Arc<dyn Repositorio>`.
fn elegir_repositorio(ruta_db: Option<&str>) -> ResultadoMain<Arc<dyn Repositorio>> {
    match ruta_db {
        None => {
            println!("INFO usando repositorio en memoria");
            Ok(Arc::new(RepositorioMemoria::new()))
        }
        Some(ruta) => {
            let repo = RepositorioSqlite::abrir(ruta)?;
            println!("INFO usando repositorio SQLite archivo={ruta}");
            Ok(Arc::new(repo))
        }
    }
    // No hace falta una función "cerrar": la conexión SQLite se cierra en el
    // Drop del repositorio cuando el último Arc desaparece.
}

/// Sirve HTTP hasta recibir Ctrl+C o SIGTERM, y entonces se apaga de forma
/// ordenada: deja de aceptar conexiones y espera a las peticiones en curso.
async fn ejecutar_servidor(addr: SocketAddr, aplicacion: axum::Router) -> ResultadoMain<()> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| format!("no se pudo escuchar en {addr}: {e}"))?;
    println!("INFO servidor escuchando addr={addr}");
    axum::serve(listener, aplicacion)
        .with_graceful_shutdown(senal_de_apagado())
        .await?;
    println!("INFO servidor apagado");
    Ok(())
}

async fn senal_de_apagado() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    // SIGTERM es lo que envía `docker stop`
    #[cfg(unix)]
    let sigterm = async {
        use tokio::signal::unix::{SignalKind, signal};
        match signal(SignalKind::terminate()) {
            Ok(mut s) => {
                s.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = sigterm => {},
    }
    println!("INFO apagando servidor...");
}

/// Levanta el servidor en un puerto libre y actúa como cliente.
async fn ejecutar_demo(aplicacion: axum::Router) -> ResultadoMain<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?; // puerto 0 = el SO elige uno libre
    let addr = listener.local_addr()?;

    // Canal oneshot para pedir al servidor que se apague al final
    let (apagar_tx, apagar_rx) = tokio::sync::oneshot::channel::<()>();
    let servidor = tokio::spawn(async move {
        axum::serve(listener, aplicacion)
            .with_graceful_shutdown(async {
                let _ = apagar_rx.await;
            })
            .await
    });

    let pasos = [
        ("GET", "/salud", ""),
        ("POST", "/tareas", r#"{"titulo":"Aprender Rust"}"#),
        ("POST", "/tareas", r#"{"titulo":"Escribir tests"}"#),
        ("POST", "/tareas", r#"{"titulo":""}"#), // inválida → 400
        ("POST", "/tareas", r#"{"titlo":"typo"}"#), // campo desconocido → 400
        ("GET", "/tareas", ""),
        ("PATCH", "/tareas/1", r#"{"completada":true}"#),
        ("GET", "/tareas/1", ""),
        ("DELETE", "/tareas/2", ""),
        ("GET", "/tareas/2", ""),   // ya no existe → 404
        ("GET", "/tareas/abc", ""), // id inválido → 400
    ];
    for (metodo, ruta, cuerpo) in pasos {
        let (estado, respuesta) = peticion_http(addr, metodo, ruta, cuerpo).await?;
        println!("\n>>> {metodo} {ruta} {cuerpo}\n<<< {estado} {respuesta}");
    }

    let _ = apagar_tx.send(());
    servidor.await??;
    println!("\nDemo terminada.");
    Ok(())
}

/// Un cliente HTTP/1.1 mínimo sobre TCP, para no añadir otra dependencia.
/// (En proyectos reales: el crate `reqwest`.) Sirve para ver que HTTP es texto.
async fn peticion_http(
    addr: SocketAddr,
    metodo: &str,
    ruta: &str,
    cuerpo: &str,
) -> ResultadoMain<(u16, String)> {
    let mut conexion = TcpStream::connect(addr).await?;
    let peticion = format!(
        "{metodo} {ruta} HTTP/1.1\r\nHost: {addr}\r\nContent-Type: application/json\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n{cuerpo}",
        cuerpo.len()
    );
    conexion.write_all(peticion.as_bytes()).await?;

    let mut bruto = Vec::new();
    conexion.read_to_end(&mut bruto).await?; // Connection: close → lee hasta que el servidor cierra
    let texto = String::from_utf8_lossy(&bruto);
    let (cabeceras, cuerpo) = texto.split_once("\r\n\r\n").unwrap_or((&texto, ""));
    // Primera línea: "HTTP/1.1 201 Created"
    let estado = cabeceras
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .ok_or("respuesta HTTP inválida")?;
    Ok((estado, cuerpo.trim().to_string()))
}

/// Hace de "curl" para el HEALTHCHECK de Docker. Las imágenes mínimas
/// (scratch, distroless) no traen curl, así que el binario sabe comprobarse:
/// `tareas_api --healthcheck` sale con código 0 si el servidor está sano.
async fn comprobar_salud(mut addr: SocketAddr) -> ResultadoMain<()> {
    if addr.ip().is_unspecified() {
        addr.set_ip([127, 0, 0, 1].into()); // escuchar en "todas" → preguntamos a localhost
    }
    let (estado, _) = tokio::time::timeout(
        Duration::from_secs(2),
        peticion_http(addr, "GET", "/salud", ""),
    )
    .await
    .map_err(|_| "timeout")??;
    if estado == 200 {
        Ok(())
    } else {
        Err(format!("estado {estado}").into())
    }
}
