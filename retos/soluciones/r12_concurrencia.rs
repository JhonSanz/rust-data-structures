// =============================================================================
// RETOS 12 — CONCURRENCIA CON HILOS              (repasa: 12_concurrencia)
// Comprueba tu solución:   cargo test -p retos --test r12
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;

/// RETO 1 — Suma en paralelo con thread::scope
/// Divide `datos` en (como mucho) `hilos` trozos, suma cada trozo en un hilo
/// y devuelve la suma total. `hilos` ≥ 1. Debe funcionar con `datos` vacío.
/// Pista: `chunks(tamaño)` con tamaño = datos.len().div_ceil(hilos) (¡nunca 0!).
fn suma_paralela(datos: &[u64], hilos: usize) -> u64 {
    let tamano = datos.len().div_ceil(hilos).max(1);
    thread::scope(|s| {
        let manejadores: Vec<_> = datos
            .chunks(tamano)
            .map(|t| s.spawn(move || t.iter().sum::<u64>()))
            .collect();
        manejadores.into_iter().map(|h| h.join().unwrap()).sum()
    })
}

#[test]
fn reto_1_suma_paralela() {
    let datos: Vec<u64> = (1..=10_000).collect();
    for hilos in [1, 3, 8, 7] {
        assert_eq!(
            suma_paralela(&datos, hilos),
            50_005_000,
            "con {hilos} hilos"
        );
    }
    assert_eq!(suma_paralela(&[], 4), 0);
}

/// RETO 2 — Arc<Mutex<T>>
/// Lanza `hilos` hilos con `thread::spawn` (¡no scope!); cada uno incrementa
/// un contador compartido `incrementos` veces. Devuelve el valor final.
fn contar_con_mutex(hilos: usize, incrementos: usize) -> usize {
    let contador = Arc::new(Mutex::new(0));
    let manejadores: Vec<_> = (0..hilos)
        .map(|_| {
            let c = Arc::clone(&contador);
            thread::spawn(move || {
                for _ in 0..incrementos {
                    *c.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for h in manejadores {
        h.join().unwrap();
    }
    *contador.lock().unwrap()
}

#[test]
fn reto_2_contar_con_mutex() {
    assert_eq!(contar_con_mutex(8, 1000), 8000);
    assert_eq!(contar_con_mutex(1, 0), 0);
}

/// RETO 3 — Worker pool con canales
/// Lanza `workers` hilos que reciben números por un canal y envían su cuadrado
/// por otro. Devuelve los resultados ORDENADOS de menor a mayor.
/// Pista: el receptor de trabajos se comparte con Arc<Mutex<Receiver<u64>>>.
///        No olvides cerrar los canales (drop) o te quedarás esperando para siempre.
fn cuadrados_con_workers(trabajos: Vec<u64>, workers: usize) -> Vec<u64> {
    let (tx_trabajo, rx_trabajo) = mpsc::channel::<u64>();
    let (tx_res, rx_res) = mpsc::channel::<u64>();
    let rx_trabajo = Arc::new(Mutex::new(rx_trabajo));
    for _ in 0..workers {
        let rx = Arc::clone(&rx_trabajo);
        let tx = tx_res.clone();
        thread::spawn(move || {
            loop {
                let trabajo = rx.lock().unwrap().recv();
                let Ok(n) = trabajo else { break };
                tx.send(n * n).unwrap();
            }
        });
    }
    drop(tx_res);
    for t in trabajos {
        tx_trabajo.send(t).unwrap();
    }
    drop(tx_trabajo);
    let mut r: Vec<u64> = rx_res.iter().collect();
    r.sort();
    r
}

#[test]
fn reto_3_cuadrados_con_workers() {
    assert_eq!(cuadrados_con_workers(vec![3, 1, 2], 2), [1, 4, 9]);
    let muchos = cuadrados_con_workers((1..=100).rev().collect(), 4);
    assert_eq!(muchos.len(), 100);
    assert_eq!(muchos[99], 10_000);
    assert!(cuadrados_con_workers(vec![], 3).is_empty());
}

/// RETO 4 — Atómicos
/// Cuenta los primos menores que `limite` repartiendo los números entre
/// `hilos` hilos (el hilo i mira i, i+hilos, i+2·hilos...) y acumulando en un
/// AtomicUsize compartido (sin Mutex).
fn es_primo(n: u64) -> bool {
    n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

fn contar_primos(limite: u64, hilos: u64) -> usize {
    let total = AtomicUsize::new(0);
    thread::scope(|s| {
        for i in 0..hilos {
            let total = &total;
            s.spawn(move || {
                let n = (i..limite)
                    .step_by(hilos as usize)
                    .filter(|&x| es_primo(x))
                    .count();
                total.fetch_add(n, Ordering::Relaxed);
            });
        }
    });
    total.load(Ordering::Relaxed)
}

#[test]
fn reto_4_contar_primos() {
    assert_eq!(contar_primos(100, 4), 25);
    assert_eq!(contar_primos(100_000, 7), 9592);
}

/// RETO 5 — OnceLock: inicializar una sola vez
/// Devuelve una tabla con los cuadrados de 0 a 999, calculada SOLO la primera
/// vez que se llama (aunque se llame desde muchos hilos a la vez).
/// Incrementa INICIALIZACIONES dentro de la inicialización para demostrarlo.
static INICIALIZACIONES: AtomicUsize = AtomicUsize::new(0);

fn tabla_cuadrados() -> &'static [u64] {
    static TABLA: OnceLock<Vec<u64>> = OnceLock::new();
    TABLA.get_or_init(|| {
        INICIALIZACIONES.fetch_add(1, Ordering::SeqCst);
        (0..1000).map(|x| x * x).collect()
    })
}

#[test]
fn reto_5_tabla_cuadrados() {
    let punteros: Vec<usize> = thread::scope(|s| {
        let hs: Vec<_> = (0..8)
            .map(|_| s.spawn(|| tabla_cuadrados().as_ptr() as usize))
            .collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    });
    assert!(
        punteros.iter().all(|&p| p == punteros[0]),
        "todas deben ser la MISMA tabla"
    );
    assert_eq!(tabla_cuadrados()[12], 144);
    assert_eq!(tabla_cuadrados().len(), 1000);
    assert_eq!(INICIALIZACIONES.load(Ordering::SeqCst), 1);
}
