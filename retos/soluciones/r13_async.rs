// =============================================================================
// RETOS 13 — ASYNC CON TOKIO                     (repasa: 13_async)
// Comprueba tu solución:   cargo test -p retos --test r13
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.
//
// Los tests usan `#[tokio::test(start_paused = true)]`: el reloj de Tokio está
// "congelado" y avanza solo cuando todas las tareas esperan. Así un sleep de
// 10 s dura 0 s de verdad, y podemos MEDIR si tu código es concurrente sin que
// los tests sean lentos ni dependan de la velocidad de la máquina.

use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::{Instant, sleep};
use tokio_util::sync::CancellationToken;

/// RETO 1 — Tu primera async fn
/// Espera `ms` milisegundos (sin bloquear el hilo) y devuelve x * 2.
async fn duplicar_tras(x: u64, ms: u64) -> u64 {
    sleep(Duration::from_millis(ms)).await;
    x * 2
}

#[tokio::test(start_paused = true)]
async fn reto_1_duplicar_tras() {
    let inicio = Instant::now();
    assert_eq!(duplicar_tras(21, 500).await, 42);
    assert!(
        inicio.elapsed() >= Duration::from_millis(500),
        "¿has usado tokio::time::sleep?"
    );
}

/// RETO 2 — Concurrencia con spawn
/// Cada tarea (nombre, ms) simula una descarga que tarda `ms` y devuelve
/// "{nombre}✓". Ejecútalas TODAS A LA VEZ y devuelve los resultados en el
/// MISMO orden que la entrada.
/// Pista: guarda los JoinHandle de `tokio::spawn` en un Vec y espéralos en orden.
async fn descargar_todas(tareas: Vec<(String, u64)>) -> Vec<String> {
    let manejadores: Vec<_> = tareas
        .into_iter()
        .map(|(nombre, ms)| {
            tokio::spawn(async move {
                sleep(Duration::from_millis(ms)).await;
                format!("{nombre}✓")
            })
        })
        .collect();
    let mut resultados = Vec::new();
    for h in manejadores {
        resultados.push(h.await.unwrap());
    }
    resultados
}

#[tokio::test(start_paused = true)]
async fn reto_2_descargar_todas() {
    let inicio = Instant::now();
    let r = descargar_todas(vec![
        ("a".into(), 300),
        ("b".into(), 100),
        ("c".into(), 200),
    ])
    .await;
    assert_eq!(r, ["a✓", "b✓", "c✓"]);
    let t = inicio.elapsed();
    assert!(
        t < Duration::from_millis(350),
        "tardó {t:?}: las descargas no fueron concurrentes"
    );
}

/// RETO 3 — Timeouts
/// Simula una tarea que tarda `ms_tarea`. Si termina antes de `ms_limite`,
/// devuelve Ok(ms_tarea); si no, Err("timeout").
/// Pista: `tokio::time::timeout`.
async fn con_limite(ms_tarea: u64, ms_limite: u64) -> Result<u64, String> {
    tokio::time::timeout(Duration::from_millis(ms_limite), async {
        sleep(Duration::from_millis(ms_tarea)).await;
        ms_tarea
    })
    .await
    .map_err(|_| "timeout".to_string())
}

#[tokio::test(start_paused = true)]
async fn reto_3_con_limite() {
    assert_eq!(con_limite(100, 500).await, Ok(100));
    let inicio = Instant::now();
    assert_eq!(con_limite(10_000, 200).await, Err("timeout".to_string()));
    assert!(
        inicio.elapsed() < Duration::from_millis(300),
        "no esperes a la tarea lenta"
    );
}

/// RETO 4 — select!: el primero que termine
/// Lanza dos esperas (de `a_ms` y `b_ms`) y devuelve "a" o "b" según cuál
/// termine antes.
async fn el_mas_rapido(a_ms: u64, b_ms: u64) -> &'static str {
    tokio::select! {
        _ = sleep(Duration::from_millis(a_ms)) => "a",
        _ = sleep(Duration::from_millis(b_ms)) => "b",
    }
}

#[tokio::test(start_paused = true)]
async fn reto_4_el_mas_rapido() {
    assert_eq!(el_mas_rapido(100, 50).await, "b");
    let inicio = Instant::now();
    assert_eq!(el_mas_rapido(10, 60_000).await, "a");
    assert!(inicio.elapsed() < Duration::from_millis(100));
}

/// RETO 5 — Productor y consumidor
/// Lanza una tarea productora que envía 1, 2, ..., n por un canal mpsc con
/// capacidad 4; la función consume los mensajes y devuelve su suma.
async fn sumar_con_canal(n: u64) -> u64 {
    let (tx, mut rx) = mpsc::channel(4);
    tokio::spawn(async move {
        for i in 1..=n {
            tx.send(i).await.unwrap();
        }
    });
    let mut suma = 0;
    while let Some(v) = rx.recv().await {
        suma += v;
    }
    suma
}

#[tokio::test(start_paused = true)]
async fn reto_5_sumar_con_canal() {
    assert_eq!(sumar_con_canal(100).await, 5050);
    assert_eq!(sumar_con_canal(0).await, 0);
}

/// RETO 6 — Cancelación
/// Cuenta "pasos": uno cada 10 ms. Cuando el token se cancele, deja de contar
/// y devuelve cuántos pasos completó.
async fn trabajar_hasta_cancelar(token: CancellationToken) -> u32 {
    let mut pasos = 0;
    loop {
        tokio::select! {
            _ = token.cancelled() => return pasos,
            _ = sleep(Duration::from_millis(10)) => pasos += 1,
        }
    }
}

#[tokio::test(start_paused = true)]
async fn reto_6_cancelacion() {
    let token = CancellationToken::new();
    let tarea = tokio::spawn(trabajar_hasta_cancelar(token.clone()));
    sleep(Duration::from_millis(55)).await;
    token.cancel();
    assert_eq!(tarea.await.unwrap(), 5);
}
