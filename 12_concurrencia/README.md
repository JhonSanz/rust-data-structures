# 12 — Concurrencia con hilos

```bash
cargo run -p concurrencia
cargo run --release -p concurrencia   # el speedup se nota más en release
```

## Temas

| Tema | Idea clave |
|------|-----------|
| `Send` / `Sync` | Traits marcadores: el compilador impide las carreras de datos |
| `thread::spawn` | Hilo del SO; devuelve `JoinHandle`; `join()` → `Result` |
| `move` | El hilo se lleva los datos (`'static`) |
| `thread::scope` | Hilos que pueden **prestar** datos locales; espera a todos al salir |
| Canales `mpsc` | `channel()`, `sync_channel(n)`, `tx.clone()`, `for msg in rx`, `recv_timeout` |
| Worker pool | Receptor compartido con `Arc<Mutex<Receiver>>` |
| `Arc<Mutex<T>>` | Estado compartido; el *guard* suelta el lock al morir |
| `RwLock` | Muchos lectores **o** un escritor |
| Atómicos | `AtomicU64::fetch_add` + `Ordering` |
| `OnceLock` | Inicialización perezosa y segura entre hilos (como `sync.Once`) |

## Comparado con Go

| Go | Rust |
|----|------|
| `go f()` | `thread::spawn(move \|\| f())` (hilo del SO) o `tokio::spawn` (tarea async, módulo 13) |
| `sync.WaitGroup` | `JoinHandle::join` o `thread::scope` |
| `chan T` | `mpsc::channel` (std), `crossbeam-channel`, `tokio::sync::mpsc` |
| `select` | `recv_timeout`, o `tokio::select!` en async |
| `sync.Mutex` protege datos "por convención" | `Mutex<T>` **contiene** los datos: imposible tocarlos sin el lock |
| `go run -race` | No hace falta: las carreras de datos no compilan |

> Las carreras de datos son imposibles en Rust seguro, pero los **interbloqueos** (deadlocks) no: cuidado con tomar varios locks en distinto orden.
