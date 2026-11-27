# 13 — Async/await con Tokio

```bash
cargo run -p async_tour
```

> Este módulo usa dos dependencias: [`tokio`](https://tokio.rs) (el runtime) y `tokio-util` (para `CancellationToken`).
> La biblioteca estándar define `Future` y `async/await`, pero **no incluye un runtime** que los ejecute.

## Temas

| Tema | Idea clave |
|------|-----------|
| `async fn` / `.await` | Devuelve un `Future` perezoso; `.await` lo ejecuta cediendo el hilo |
| `#[tokio::main]` | Crea el runtime (pool de hilos) y ejecuta `main` en él |
| `tokio::join!` | Varios futures a la vez dentro de la misma tarea |
| `tokio::spawn` | Tarea independiente (como `go f()`); necesita `'static` → `async move` |
| `JoinSet` | Grupo de tareas; `join_next` (orden de llegada) o `join_all` |
| `tokio::select!` | La primera rama que termina gana; el resto se cancela |
| `timeout` | Envuelve cualquier future con un límite de tiempo |
| Canales | `mpsc` (con back-pressure), `oneshot`, `broadcast`, `watch` |
| `tokio::sync::Mutex` | Solo si mantienes el lock a través de un `.await`; si no, `std::sync::Mutex` |
| `CancellationToken` | El `context.Context` de Rust |
| `spawn_blocking` | Para trabajo de CPU o E/S síncrona sin bloquear el runtime |

## Hilos vs async

| | Hilos (módulo 12) | Async (este módulo) |
|---|---|---|
| Ideal para | Cálculo en paralelo (CPU) | Mucha E/S concurrente (red) |
| Coste por unidad | ~MB de pila, hilo del SO | ~KB, una máquina de estados |
| Cuántos | Decenas–cientos | Cientos de miles |
| Bloquear | Normal | **Prohibido** (usa `spawn_blocking`) |
