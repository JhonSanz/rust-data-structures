# 16 — Biblioteca estándar esencial

```bash
cargo run -p biblioteca_estandar -- -v uno dos
echo -e "hola\nmundo" | cargo run -p biblioteca_estandar
```

## Temas

| Módulo | Qué se ve |
|--------|-----------|
| `std::env` | `args()`, `var()`, `current_dir()`, `consts::OS`; `env!` en compilación |
| Texto | `parse`, `from_str_radix`, métodos de `char`, `fmt::Write` + `write!` sobre `String`, `from_utf8(_lossy)` |
| `std::time` | `Instant` (monotónico), `Duration`, `SystemTime` + `UNIX_EPOCH` |
| `std::fs`, `std::io`, `std::path` | `fs::write/read_to_string`, `OpenOptions`, `BufReader::lines`, `BufWriter`, `metadata`, `read_dir`, `io::ErrorKind`, `Path/PathBuf` |
| `std::process` | `Command::new(..).output()/status()`, `exit` |
| `std::mem`, `std::cmp` | `swap`, `replace`, `take`, `size_of`; `Ordering`, `clamp`, `then_with`, `total_cmp` |
| stdin | `io::stdin().lock().lines()`, `IsTerminal` |

## Lo que la std NO trae (y los crates que todo el mundo usa)

| Necesidad | Crate |
|-----------|-------|
| JSON y serialización | [`serde`](https://serde.rs) + [`serde_json`](https://docs.rs/serde_json) (se usa en el módulo 17) |
| Expresiones regulares | [`regex`](https://docs.rs/regex) |
| Fechas y zonas horarias | [`jiff`](https://docs.rs/jiff) o [`chrono`](https://docs.rs/chrono) |
| Logging estructurado | [`tracing`](https://docs.rs/tracing) + `tracing-subscriber` (o `log` + `env_logger`) |
| Argumentos de línea de comandos | [`clap`](https://docs.rs/clap) |
| Async / red | [`tokio`](https://tokio.rs), [`reqwest`](https://docs.rs/reqwest) (cliente HTTP), [`axum`](https://docs.rs/axum) (servidor) |
| Paralelismo de datos | [`rayon`](https://docs.rs/rayon) |
| Errores | [`thiserror`](https://docs.rs/thiserror), [`anyhow`](https://docs.rs/anyhow) |
| Archivos temporales | [`tempfile`](https://docs.rs/tempfile) |

Busca crates en [crates.io](https://crates.io) y su documentación en [docs.rs](https://docs.rs). [blessed.rs](https://blessed.rs) recopila los recomendados por la comunidad.

## Para recordar

- No hay que cerrar archivos: se cierran al salir del ámbito (`Drop`).
- `BufWriter` hace *flush* en su `Drop`, pero **ignora los errores**: llama a `flush()` tú.
- `main` puede devolver `io::Result<()>` (o `Result<(), Box<dyn Error>>`) para usar `?`.
