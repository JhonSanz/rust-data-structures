# 15 — Testing

```bash
cargo run -p testing                            # el programa
cargo test -p testing                           # unitarios + integración + doc-tests
cargo test -p testing palindromo                # filtrar por nombre
cargo test -p testing -- --nocapture            # ver los println! de los tests
cargo test -p testing -- --ignored              # solo los #[ignore]
cargo bench -p testing                          # benchmark
```

## Archivos

- `src/lib.rs` — el código a probar **y sus tests unitarios** (`mod tests`) y doc-tests.
- `tests/integracion.rs` — tests de integración: solo ven la API pública.
- `benches/rendimiento.rs` — un benchmark mínimo sin dependencias.
- `src/main.rs` — un pequeño programa que usa la biblioteca.

## Temas

| Tema | Idea clave |
|------|-----------|
| Tests unitarios | `#[cfg(test)] mod tests { #[test] fn ... }` en el mismo archivo; ven lo privado |
| Aserciones | `assert!`, `assert_eq!`, `assert_ne!`, con mensaje opcional `"{x:?}"` |
| Table-driven tests | Array de tuplas `(nombre, entrada, esperado)` + bucle |
| Tests con `Result` | `fn t() -> Result<(), Box<dyn Error>>` para usar `?` |
| Panics esperados | `#[should_panic(expected = "...")]` |
| `#[ignore]` | Tests lentos; se ejecutan con `-- --ignored` |
| Helpers | `#[track_caller]` (como `t.Helper()`) |
| Propiedades | Comprobar invariantes con muchas entradas (en serio: crate `proptest`) |
| Integración | Carpeta `tests/`: cada archivo es un crate externo |
| Doc-tests | Los ejemplos ```` ``` ```` de `///` se compilan y ejecutan |
| Recursos temporales | Un guard con `Drop` (o el crate `tempfile`) |
| Benchmarks | `std::hint::black_box`; en serio: `criterion` o `divan` |

## Para recordar

- Los tests se ejecutan **en paralelo** por defecto: no compartas archivos ni estado global entre ellos.
- Cobertura: `cargo install cargo-llvm-cov` y `cargo llvm-cov -p testing`.
- Fuzzing: `cargo install cargo-fuzz` (requiere nightly).
