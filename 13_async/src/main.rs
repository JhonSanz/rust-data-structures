// =============================================================================
// MÓDULO 13 — ASYNC/AWAIT CON TOKIO
// Ejecuta con:  cargo run -p async_tour
// =============================================================================
//
// async/await permite tener MILES de tareas concurrentes de E/S (red, disco,
// temporizadores) con unos pocos hilos. Es lo más parecido a las goroutines.
//
// - `async fn` devuelve un Future: un cálculo que AÚN NO se ha ejecutado.
// - `.await` lo ejecuta, cediendo el hilo mientras espera.
// - Los futures son PEREZOSOS: sin .await (o spawn) no hacen nada.
// - La biblioteca estándar define Future, pero NO trae un "runtime" que los
//   ejecute. Tokio es el runtime de facto (es la dependencia de este módulo).
//
// Regla de oro: nunca bloquees dentro de async (thread::sleep, cálculos
// largos, E/S síncrona). Usa las versiones async o spawn_blocking.

use std::time::{Duration, Instant};
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;

// #[tokio::main] crea el runtime y ejecuta main dentro de él.
#[tokio::main]
async fn main() {
    println!("===== 1. async fn y .await =====");
    let t = Instant::now();
    let a = descargar("a.txt", 100).await;
    let b = descargar("b.txt", 100).await;
    println!("secuencial: {a}, {b} en {:?}", t.elapsed());

    println!("\n===== 2. join!: concurrencia en la misma tarea =====");
    let t = Instant::now();
    let (a, b, c) = tokio::join!(
        descargar("a.txt", 100),
        descargar("b.txt", 100),
        descargar("c.txt", 100)
    );
    println!("concurrente: {a}, {b}, {c} en {:?}", t.elapsed());

    println!("\n===== 3. tokio::spawn y JoinSet =====");
    tareas().await;

    println!("\n===== 4. select!: la primera que termine =====");
    select().await;

    println!("\n===== 5. Timeouts =====");
    match timeout(Duration::from_millis(50), descargar("lento.iso", 500)).await {
        Ok(v) => println!("a tiempo: {v}"),
        Err(_) => println!("timeout tras 50 ms"),
    }

    println!("\n===== 6. Canales async: productor/consumidor =====");
    canales().await;

    println!("\n===== 7. Estado compartido: Arc<tokio::sync::Mutex> =====");
    estado_compartido().await;

    println!("\n===== 8. Cancelación (el context de Go) =====");
    cancelacion().await;

    println!("\n===== 9. Trabajo bloqueante: spawn_blocking =====");
    let suma = tokio::task::spawn_blocking(|| (1..=10_000_000u64).sum::<u64>())
        .await
        .unwrap();
    println!("calculado en un hilo aparte sin bloquear el runtime: {suma}");
}

async fn descargar(nombre: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await; // simula E/S: cede el hilo
    format!("{nombre}✓")
}

async fn tareas() {
    // spawn lanza una tarea independiente (como `go f()`). Debe ser 'static:
    // por eso se usa `move` y datos propios.
    let h = tokio::spawn(async {
        sleep(Duration::from_millis(20)).await;
        42
    });
    println!("spawn devolvió {}", h.await.unwrap());

    // JoinSet: un grupo de tareas (como un WaitGroup que además recoge resultados)
    let mut set = JoinSet::new();
    for i in 1..=5u64 {
        set.spawn(async move {
            sleep(Duration::from_millis(60 - i * 10)).await;
            i * i
        });
    }
    let mut orden = vec![];
    while let Some(r) = set.join_next().await {
        orden.push(r.unwrap());
    }
    println!("resultados en orden de llegada: {orden:?}");

    // 10.000 tareas son baratas (no son hilos del SO)
    let t = Instant::now();
    let mut set = JoinSet::new();
    for _ in 0..10_000 {
        set.spawn(sleep(Duration::from_millis(50)));
    }
    set.join_all().await;
    println!(
        "10.000 tareas que duermen 50 ms → {:?} en total",
        t.elapsed()
    );
}

async fn select() {
    // select! espera a varias ramas y se queda con la PRIMERA; las demás se cancelan.
    let resultado = tokio::select! {
        v = descargar("servidor-eu", 80) => format!("ganó EU: {v}"),
        v = descargar("servidor-us", 30) => format!("ganó US: {v}"),
        _ = sleep(Duration::from_millis(200)) => "timeout".to_string(),
    };
    println!("{resultado}");
}

async fn canales() {
    // mpsc acotado: send().await espera si el canal está lleno (back-pressure)
    let (tx, mut rx) = mpsc::channel::<u32>(2);

    let productor = tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
            println!("  enviado {i}");
        }
        // al terminar, `tx` se destruye y el canal se cierra
    });

    while let Some(v) = rx.recv().await {
        sleep(Duration::from_millis(10)).await;
        println!("  procesado {v}");
    }
    productor.await.unwrap();

    // oneshot: un único valor (ideal para respuestas petición/respuesta)
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        let _ = tx.send("respuesta única");
    });
    println!("oneshot: {}", rx.await.unwrap());

    // Otros: tokio::sync::broadcast (varios receptores) y watch (último valor)
}

async fn estado_compartido() {
    use std::sync::Arc;
    // tokio::sync::Mutex se puede mantener bloqueado a través de un .await.
    // Si NO necesitas eso, std::sync::Mutex es más rápido (y válido en async).
    let saldo = Arc::new(Mutex::new(0i64));
    let mut set = JoinSet::new();
    for i in 0..100 {
        let saldo = Arc::clone(&saldo);
        set.spawn(async move {
            let mut s = saldo.lock().await;
            *s += if i % 2 == 0 { 10 } else { -5 };
        });
    }
    set.join_all().await;
    println!("saldo final = {}", *saldo.lock().await);
}

async fn cancelacion() {
    // CancellationToken (de tokio-util) cumple el papel de context.Context:
    // se clona, se pasa a las tareas y todas se enteran al cancelarlo.
    let token = CancellationToken::new();
    let mut set = JoinSet::new();
    for id in 0..3 {
        let token = token.clone();
        set.spawn(async move {
            let mut pasos = 0;
            loop {
                tokio::select! {
                    _ = token.cancelled() => {
                        println!("  worker {id}: cancelado tras {pasos} pasos, limpiando...");
                        return pasos;
                    }
                    _ = sleep(Duration::from_millis(20)) => pasos += 1,
                }
            }
        });
    }
    sleep(Duration::from_millis(110)).await;
    token.cancel();
    let total: u32 = set.join_all().await.iter().sum();
    println!("pasos totales antes de cancelar: {total}");

    // Además, soltar (drop) un future lo cancela: por eso timeout y select! funcionan.
}
