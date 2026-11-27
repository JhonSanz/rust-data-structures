// =============================================================================
// MÓDULO 12 — CONCURRENCIA CON HILOS
// Ejecuta con:  cargo run -p concurrencia
// =============================================================================
//
// "Fearless concurrency": el sistema de tipos impide las CARRERAS DE DATOS en
// compilación. Dos traits marcadores lo hacen posible:
//   - Send: el valor se puede MOVER a otro hilo        (Rc NO es Send)
//   - Sync: el valor se puede COMPARTIR (&T) entre hilos (RefCell NO es Sync)
// El compilador los calcula solo. Si intentas mandar un Rc a otro hilo, no compila.
//
// Rust usa hilos del sistema operativo (no goroutines). Para miles de tareas
// concurrentes de E/S se usa async (módulo 13).

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("===== 1. thread::spawn y join =====");
    spawn_y_join();

    println!("\n===== 2. move: llevar datos al hilo =====");
    move_a_hilos();

    println!("\n===== 3. thread::scope: tomar prestado sin Arc =====");
    hilos_con_scope();

    println!("\n===== 4. Canales (mpsc) =====");
    canales();

    println!("\n===== 5. Worker pool con canales =====");
    worker_pool();

    println!("\n===== 6. Arc<Mutex<T>>: estado compartido =====");
    arc_mutex();

    println!("\n===== 7. RwLock: muchos lectores, un escritor =====");
    rwlock();

    println!("\n===== 8. Atómicos =====");
    atomicos();

    println!("\n===== 9. OnceLock: inicialización perezosa, una sola vez =====");
    for _ in 0..3 {
        println!("config = {:?}", config());
    }

    println!("\n===== 10. Paralelismo real: speedup =====");
    speedup();
}

/*
-----------------------------------------------------------
   HILOS: conceptos básicos
-----------------------------------------------------------

¿Qué es un hilo (thread)?
Una línea de ejecución independiente dentro del mismo programa. Todos
los hilos comparten la memoria del proceso, y el sistema operativo los
reparte entre los núcleos de la CPU: varios hilos = trabajo en
PARALELO de verdad.

    main ──●──spawn──┬──────────────────────join──●──▶
                     │                         ▲
    hilo 1           └──●━━━━ trabaja ━━━━●────┘

- spawn(closure): crea el hilo y ejecuta el closure en él.
- JoinHandle: el "resguardo" que devuelve spawn.
- join(): espera a que el hilo termine y recoge lo que devolvió.
  Devuelve Result: Err si el hilo hizo panic.

-----------------------------------------------------------
   Concurrencia vs paralelismo
-----------------------------------------------------------

- CONCURRENCIA: gestionar varias tareas a la vez (pueden turnarse
  en un solo núcleo).
- PARALELISMO: ejecutarlas literalmente al mismo tiempo (varios núcleos).

Hilos → concurrencia Y paralelismo. Async (módulo 13) → concurrencia
con pocos hilos.

-----------------------------------------------------------
   CARRERA DE DATOS: el bug que Rust impide
-----------------------------------------------------------

Una carrera de datos (data race) ocurre cuando dos hilos acceden al
mismo dato a la vez, al menos uno escribe, y no hay sincronización.

    contador = 5
    hilo A: lee 5 ──────────── escribe 6
    hilo B:      lee 5 ──────────── escribe 6     ← ¡debería ser 7!

El resultado depende de la "suerte" de cada ejecución. En Rust esto
NO COMPILA, gracias a dos traits "marcadores" (no tienen métodos,
solo marcan una propiedad):

| Trait | Significa                                         | NO lo cumple |
| ----- | ------------------------------------------------- | ------------ |
| Send  | se puede MOVER a otro hilo                        | Rc (su contador no es atómico) |
| Sync  | se puede COMPARTIR (&T) entre hilos a la vez      | RefCell, Cell |

El compilador los calcula solo para cada tipo. thread::spawn exige
que lo que envías sea Send: si intentas mandar un Rc, no compila.
*/

fn spawn_y_join() {
    let mut manejadores = Vec::new();
    for id in 0..3 {
        // spawn devuelve un JoinHandle; el closure se ejecuta en otro hilo
        let h = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10 * (3 - id)));
            println!("  hilo {id} terminado");
            id * 100 // valor devuelto por el hilo
        });
        manejadores.push(h);
    }
    // join espera a que acabe y devuelve Result (Err si el hilo hizo panic)
    let resultados: Vec<u64> = manejadores.into_iter().map(|h| h.join().unwrap()).collect();
    println!("resultados = {resultados:?}");
    // Si main termina, el proceso termina aunque queden hilos (no se esperan solos).
}

fn move_a_hilos() {
    let datos: Vec<i32> = (1..=3).collect(); // un Vec (en el heap, no es Copy)
    // Sin `move` no compila: el hilo podría vivir más que `datos`.
    let h = thread::spawn(move || {
        let suma: i32 = datos.iter().sum();
        println!("  el hilo sumó {suma}");
    });
    // println!("{datos:?}"); // ERROR: `datos` se movió al hilo
    h.join().unwrap();
}

/*
-----------------------------------------------------------
   thread::spawn vs thread::scope
-----------------------------------------------------------

thread::spawn exige 'static (que el closure no tome nada prestado):
el hilo podría seguir vivo después de que la función que lo creó
termine, y sus préstamos apuntarían a datos muertos. Por eso hace
falta `move` (o Arc).

thread::scope GARANTIZA que todos sus hilos terminan antes de salir
del bloque. Así pueden PRESTAR datos locales sin Arc ni move:

    let datos = vec![...];
    thread::scope(|s| {
        s.spawn(|| usa(&datos));     // ✓ préstamo
        s.spawn(|| usa(&datos));
    });                              // ← aquí se espera a los dos
    // datos sigue vivo y es nuestro
*/

fn hilos_con_scope() {
    // thread::scope garantiza que todos los hilos terminan antes de salir del
    // bloque, así que pueden PRESTAR datos del hilo actual. Es lo más cómodo
    // para paralelizar un cálculo sobre datos locales.
    let numeros: Vec<u64> = (1..=1_000).collect();
    let mut resultados = [0u64; 4];

    thread::scope(|s| {
        for (trozo, res) in numeros.chunks(250).zip(resultados.iter_mut()) {
            s.spawn(move || {
                *res = trozo.iter().sum(); // cada hilo escribe en SU celda: sin carreras
            });
        }
    }); // ← aquí se espera a todos

    println!(
        "sumas parciales = {resultados:?}, total = {}",
        resultados.iter().sum::<u64>()
    );
}

/*
-----------------------------------------------------------
   CANALES: comunicarse enviando mensajes
-----------------------------------------------------------

"No comuniques compartiendo memoria; comparte memoria comunicando."

Un canal es una tubería con dos extremos:

    productor 1 ── tx ──┐
    productor 2 ── tx ──┼──▶ [ cola de mensajes ] ──▶ rx ── consumidor
    productor 3 ── tx ──┘

mpsc = Multiple Producer, Single Consumer:
- tx (Sender): se puede clonar → muchos productores
- rx (Receiver): solo uno

Al enviar un valor por el canal, se MUEVE: el productor ya no lo
tiene, así que no puede haber dos hilos tocándolo.

IMPORTANTE: el canal se cierra cuando mueren TODOS los tx. Hasta
entonces, `for msg in rx` sigue esperando. Si olvidas hacer drop del
tx original, el programa se queda colgado para siempre.

| Función          | Si el canal está vacío/lleno          |
| ---------------- | ------------------------------------- |
| channel()        | ilimitado: send nunca espera          |
| sync_channel(n)  | capacidad n: send ESPERA si está lleno ("back-pressure") |
| recv_timeout(d)  | espera como mucho d; Err si no llega nada |
*/

fn canales() {
    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel::<String>();

    for id in 0..3 {
        let tx = tx.clone(); // cada productor tiene su emisor
        thread::spawn(move || {
            for i in 0..2 {
                tx.send(format!("productor {id}, mensaje {i}")).unwrap();
                thread::sleep(Duration::from_millis(5));
            }
        });
    }
    drop(tx); // ¡importante! el canal se cierra cuando mueren TODOS los emisores

    // Iterar sobre rx recibe hasta que el canal se cierra (como range en Go)
    let mut n = 0;
    for msg in rx {
        n += 1;
        println!("  recibido: {msg}");
    }
    println!("total recibidos: {n}");

    // Canal con capacidad limitada (sync_channel): send bloquea si está lleno
    let (tx, rx) = mpsc::sync_channel::<i32>(1);
    thread::spawn(move || {
        for i in 0..3 {
            tx.send(i).unwrap();
        }
    });
    // recv_timeout: el equivalente a un select con timeout
    while let Ok(v) = rx.recv_timeout(Duration::from_millis(100)) {
        print!("  sync_channel → {v}");
    }
    println!();
}

fn worker_pool() {
    let (tx_trabajos, rx_trabajos) = mpsc::channel::<u64>();
    let (tx_resultados, rx_resultados) = mpsc::channel::<(usize, u64, u64)>();
    // El receptor no se puede clonar: lo compartimos entre workers con Arc<Mutex>
    let rx_trabajos = Arc::new(Mutex::new(rx_trabajos));

    let workers: Vec<_> = (0..3)
        .map(|id| {
            let rx = Arc::clone(&rx_trabajos);
            let tx = tx_resultados.clone();
            thread::spawn(move || {
                loop {
                    // El lock se suelta al final de esta sentencia (el guard es temporal)
                    let trabajo = rx.lock().unwrap().recv();
                    let Ok(n) = trabajo else { break }; // canal cerrado → salir
                    let cuadrado = n * n;
                    thread::sleep(Duration::from_millis(5));
                    tx.send((id, n, cuadrado)).unwrap();
                }
            })
        })
        .collect();
    drop(tx_resultados);

    for n in 1..=6 {
        tx_trabajos.send(n).unwrap();
    }
    drop(tx_trabajos); // no hay más trabajo

    let mut res: Vec<_> = rx_resultados.iter().collect();
    res.sort_by_key(|r| r.1);
    for (id, n, c) in res {
        println!("  worker {id}: {n}² = {c}");
    }
    for w in workers {
        w.join().unwrap();
    }
}

/*
-----------------------------------------------------------
   Arc<Mutex<T>>: estado compartido y mutable entre hilos
-----------------------------------------------------------

Cada pieza resuelve UN problema:
- Arc   → varios DUEÑOS en varios hilos (contador atómico)
- Mutex → un solo hilo MODIFICA a la vez (exclusión mutua)

    hilo 1 ── Arc ─┐
    hilo 2 ── Arc ─┼──▶ Mutex [ 🔒 ] ──▶ valor
    hilo 3 ── Arc ─┘

lock() espera hasta conseguir el candado y devuelve un GUARD
(MutexGuard): un smart pointer que da acceso al valor y que, al
destruirse, SUELTA el candado automáticamente (RAII, como Drop).

    {
        let mut num = mutex.lock().unwrap();   // 🔒 bloqueado
        *num += 1;
    }                                          // 🔓 el guard muere: liberado

Diferencia con otros lenguajes: en Go el Mutex está AL LADO de los
datos, y protegerlos es una convención. En Rust el Mutex CONTIENE los
datos: no hay forma de tocarlos sin hacer lock().

TÉRMINOS:
- ENVENENAMIENTO (poisoning): si un hilo hace panic con el candado
  tomado, el Mutex queda "envenenado" y lock() devuelve Err, por si el
  dato quedó a medias. Por eso hay un unwrap().
- INTERBLOQUEO (deadlock): el hilo A tiene el candado 1 y espera el 2;
  el hilo B tiene el 2 y espera el 1. Esperan para siempre. Rust NO
  lo impide. Regla: toma siempre los candados en el mismo orden.
*/

fn arc_mutex() {
    // Arc = Rc atómico (se puede compartir entre hilos)
    // Mutex = exclusión mutua; lock() devuelve un GUARD que suelta el lock al morir.
    let contador = Arc::new(Mutex::new(0));
    let mut hilos = vec![];
    for _ in 0..8 {
        let c = Arc::clone(&contador);
        hilos.push(thread::spawn(move || {
            for _ in 0..1000 {
                *c.lock().unwrap() += 1;
            }
        }));
    }
    for h in hilos {
        h.join().unwrap();
    }
    println!(
        "contador = {} (siempre 8000: sin carreras)",
        *contador.lock().unwrap()
    );

    // Sin Mutex, esto NO compila:
    //   let mut x = 0;
    //   thread::scope(|s| { s.spawn(|| x += 1); s.spawn(|| x += 1); });
    //   → error: cannot borrow `x` as mutable more than once at a time

    // lock() devuelve Result: es Err si otro hilo hizo panic con el lock tomado
    // ("envenenamiento"). En la práctica casi siempre se hace unwrap().
}

fn rwlock() {
    let cache: Arc<RwLock<HashMap<&str, u32>>> = Arc::new(RwLock::new(HashMap::new()));
    cache.write().unwrap().insert("a", 1);

    thread::scope(|s| {
        for i in 0..3 {
            let cache = &cache;
            s.spawn(move || {
                let lectura = cache.read().unwrap(); // varios lectores a la vez
                println!("  lector {i} ve a = {:?}", lectura.get("a"));
            });
        }
        s.spawn(|| {
            cache.write().unwrap().insert("b", 2); // escritor exclusivo
        });
    });
    println!("claves finales: {}", cache.read().unwrap().len());
}

/*
-----------------------------------------------------------
   ATÓMICOS: sincronización sin candados
-----------------------------------------------------------

Una operación ATÓMICA se ejecuta de una sola vez, sin que otro hilo
pueda verla a medias. `fetch_add(1)` lee, suma y escribe en UN paso
indivisible de la CPU, así que el problema de "los dos leen 5" no
puede ocurrir.

Para un contador o un flag, un atómico es más rápido que un Mutex.
Para algo más complejo que un número (un Vec, un HashMap), usa Mutex.

Ordering (cuánto se garantiza sobre el orden de OTRAS operaciones):
- Relaxed: solo que esta operación es atómica. Suficiente para contadores.
- Acquire/Release: para usar un atómico como "señal" de que otros
  datos ya están listos.
- SeqCst: el más estricto y fácil de razonar. Si dudas, este.
*/

fn atomicos() {
    // Para contadores y flags simples, los atómicos evitan el Mutex
    static PETICIONES: AtomicU64 = AtomicU64::new(0);
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..2500 {
                    PETICIONES.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
    println!("peticiones = {}", PETICIONES.load(Ordering::Relaxed));
    // Ordering::Relaxed basta para contadores. SeqCst es el más estricto (y
    // seguro por defecto). Acquire/Release para sincronizar datos entre hilos.
}

#[derive(Debug)]
struct Config {
    nombre: String,
    hilos: usize,
}

fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        println!("  (inicializando config... solo una vez)");
        Config {
            nombre: "app".into(),
            hilos: thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
        }
    })
}

fn es_primo(n: u64) -> bool {
    n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

fn speedup() {
    let limite = 300_000u64;
    let hilos = config().hilos.min(8);
    println!("usando {hilos} hilos ({})", config().nombre);

    let t = Instant::now();
    let secuencial = (0..limite).filter(|&n| es_primo(n)).count();
    let t_seq = t.elapsed();

    let t = Instant::now();
    let paralelo: usize = thread::scope(|s| {
        let manejadores: Vec<_> = (0..hilos as u64)
            .map(|i| s.spawn(move || (i..limite).step_by(hilos).filter(|&n| es_primo(n)).count()))
            .collect();
        manejadores.into_iter().map(|h| h.join().unwrap()).sum()
    });
    let t_par = t.elapsed();

    println!("primos < {limite}: {secuencial} en {t_seq:?} | {paralelo} en {t_par:?} en paralelo");
    // (En el mundo real, el crate `rayon` hace esto con solo cambiar .iter() por .par_iter())
}
