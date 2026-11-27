# 08 — Manejo de errores

```bash
cargo run -p errores
```

## Temas

| Tema | Idea clave |
|------|-----------|
| `Result<T, E>` | `Ok(v)` / `Err(e)`: los errores son valores; ignorar un `Result` da aviso |
| Operador `?` | Propaga el error (y lo convierte con `From`). Sustituye a `if err != nil { return err }` |
| Métodos | `unwrap_or`, `unwrap_or_else`, `map`, `map_err`, `and_then`, `ok`, `ok_or` |
| Errores propios | `enum` con una variante por caso + `Display` + `impl Error` |
| Causas | `Error::source()` encadena errores (como `%w` y `errors.Unwrap`) |
| Comprobar el tipo | `match` / `matches!` sobre el enum (como `errors.Is/As`) |
| `Box<dyn Error>` | "Cualquier error": cómodo en `main` y aplicaciones |
| `collect` sobre `Result` | `Vec<Result<T,E>>` → `Result<Vec<T>,E>` |
| `?` con `Option` | Sale con `None` |
| `panic!` | Para bugs, no para errores esperados. `assert!`, `unreachable!`, `todo!` |

## En proyectos reales

| Crate | Para qué |
|-------|----------|
| [`thiserror`](https://docs.rs/thiserror) | Librerías: genera `Display`, `Error` y `From` con `#[derive(Error)]` |
| [`anyhow`](https://docs.rs/anyhow) | Aplicaciones: `anyhow::Result<T>` y `.context("...")` para añadir contexto |

> `main` también puede devolver `Result<(), Box<dyn Error>>`: si es `Err`, imprime el error y sale con código 1.
