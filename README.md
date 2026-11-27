# Tour de Rust en español

Un recorrido rápido y práctico por los aspectos fundamentales de Rust.
Cada directorio es un paquete **ejecutable** de un *workspace* de Cargo, con las
explicaciones escritas como comentarios dentro del código y un `README.md` con el
resumen de sus temas.

> Requiere Rust 1.85 o superior (edición 2024). Comprueba tu versión con `rustc --version`
> e instala o actualiza con [rustup](https://rustup.rs): `rustup update`.

## Cómo usarlo

1. Abre el `src/main.rs` de un módulo y léelo de arriba abajo: los comentarios son la explicación.
2. Ejecútalo desde la raíz del proyecto (cada módulo es un paquete con nombre propio) y compara la salida con el código:

```bash
cargo run -p fundamentos
```

3. Modifica el código, rompe cosas y vuelve a ejecutar. En Rust, **leer los errores del compilador** es la mitad del aprendizaje: suelen decirte exactamente qué cambiar.

## Índice

| # | Módulo | Paquete (`-p`) | Temas principales |
|---|--------|----------------|-------------------|
| 01 | [Fundamentos](01_fundamentos/) | `fundamentos` | Variables, `mut`, shadowing, tipos, conversiones (`as`, `From`, `TryFrom`), `String` vs `&str`, `println!` |
| 02 | [Control de flujo](02_control_de_flujo/) | `control_de_flujo` | `if` como expresión, `loop`/`while`/`for`, rangos, `match`, `if let`, `let else`, etiquetas |
| 03 | [Funciones](03_funciones/) | `funciones` | Retorno implícito, tuplas, closures, `Fn`/`FnMut`/`FnOnce`, `move`, `impl Fn`, tipo `!` |
| 04 | [Ownership y borrowing](04_ownership_y_borrowing/) | `ownership` | **El corazón de Rust**: move, `Copy`/`Clone`, `&T`, `&mut T`, slices, `Drop` |
| 05 | [Structs y enums](05_structs_y_enums/) | `structs_y_enums` | `struct`, `impl`, `&self`/`&mut self`/`self`, `derive`, enums con datos, `Option`, builder |
| 06 | [Colecciones](06_colecciones/) | `colecciones` | `Vec`, slices, `String`, `HashMap` + entry API, `BTreeMap`, `HashSet`, `VecDeque`, `BinaryHeap` |
| 07 | [Traits](07_traits/) | `traits` | Definir/implementar, por defecto, `impl Trait` vs `dyn Trait`, operadores, `From`, blanket impls |
| 08 | [Errores](08_errores/) | `errores` | `Result`, `?`, errores propios, `source()`, `Box<dyn Error>`, `panic!` |
| 09 | [Genéricos y lifetimes](09_genericos_y_lifetimes/) | `genericos` | Restricciones, `where`, tipos genéricos, tipos asociados, const generics, `'a` |
| 10 | [Iteradores](10_iteradores_y_closures/) | `iteradores` | `Iterator`, adaptadores perezosos, `collect`, iteradores propios, `from_fn` |
| 11 | [Smart pointers](11_smart_pointers/) | `smart_pointers` | `Box`, `Deref`, `Rc`, `Cell`, `RefCell`, `Weak`, `Cow` |
| 12 | [Concurrencia](12_concurrencia/) | `concurrencia` | `Send`/`Sync`, `thread::spawn`, `scope`, canales `mpsc`, `Arc<Mutex>`, `RwLock`, atómicos, `OnceLock` |
| 13 | [Async](13_async/) | `async_tour` | `async`/`.await`, Tokio, `spawn`, `JoinSet`, `select!`, `timeout`, canales, cancelación |
| 14 | [Módulos y crates](14_modulos_y_crates/) | `modulos` | `mod`, `pub`/`pub(crate)`/`pub(super)`, `use`, lib + bin, features, `macro_rules!`, doc-tests |
| 15 | [Testing](15_testing/) | `testing` | Tests unitarios, table-driven, `should_panic`, integración (`tests/`), doc-tests, benchmarks |
| 16 | [Biblioteca estándar](16_biblioteca_estandar/) | `biblioteca_estandar` | `env`, `fs`, `io`, `path`, `time`, `process`, `mem`, `cmp`... y qué crates usar para lo demás |
| 17 | [Proyecto: API REST](17_proyecto_api_rest/) | `tareas_api` | axum + Tokio, adaptadores de memoria y SQLite (`rusqlite`), puertos y adaptadores, tests con `oneshot` |
| 18 | [Docker](18_docker/) | — | Imagen de producción de 5,5 MB: multi-stage, musl estático, `scratch`, caché de BuildKit, hardening |
| 🏋️ | [**Retos**](retos/) | `retos` | **92 problemas** (de 4 a 6 por módulo) con tests que comprueban tu solución y un comprobador de progreso |

## Estructuras de datos desde cero

En [`estructuras_de_datos/`](estructuras_de_datos/) están las estructuras implementadas a mano
(son crates del mismo workspace):

| Crate | Contenido | Ejecutar |
|-------|-----------|----------|
| `basic` | Punteros, POO con traits, funciones de la std, ejemplos de hilos | `cargo run -p basic` |
| `vectors` | `MyVec<T>`: un `Vec` propio con `alloc`/`dealloc`, `MaybeUninit` y `unsafe` | `cargo test -p vectors` |
| `stack`, `queue`, `linked_list`, `heap_max`, `graph` | Por implementar | `cargo run -p stack` |

> `vectors` usa `unsafe` a propósito (gestiona memoria a mano). Es un buen complemento a los
> módulos 04 y 11: lo que el tour usa "gratis" con `Vec` y `Box`, aquí se construye pieza a pieza.

## ¿Lo has aprendido de verdad? Los retos

Después de cada módulo, resuelve sus retos en [`retos/ejercicios/`](retos/) y comprueba tu progreso:

```bash
cargo run -p retos               # tabla de progreso de todos los módulos
cargo test -p retos --test r04   # los retos de un módulo
```

> Los módulos 01–12 y 14–16 usan solo la biblioteca estándar. El 13 usa `tokio` (Rust no trae un
> runtime async) y el 17 usa `axum`, `tokio`, `serde` y `rusqlite`. Las versiones se declaran una
> sola vez en el `Cargo.toml` de la raíz (`[workspace.dependencies]`); la primera compilación las
> descarga automáticamente.

## Comandos útiles

```bash
cargo run -p <paquete>                  # ejecutar un módulo
cargo run --release -p concurrencia     # compilado con optimizaciones
cargo check                             # comprobar que todo compila (rápido, sin generar binarios)
cargo test --workspace --exclude retos  # todos los tests (los retos sin resolver fallan a propósito)
cargo clippy --workspace --exclude retos --all-targets   # linter: cientos de sugerencias idiomáticas
cargo fmt                               # formatear el código (estilo oficial)
cargo doc --open                        # documentación de tu código y de las dependencias
rustup doc --std                        # documentación de la biblioteca estándar, sin conexión
rustc --explain E0382                   # explicación larga de un código de error
```

## Ejecutar todos los módulos de una vez

```bash
for d in [0-9][0-9]_*/; do [ -f "$d/Cargo.toml" ] || continue; p=$(sed -n 's/^name = "\(.*\)"/\1/p' "$d/Cargo.toml" | head -1); echo "######## $d"; cargo run -q -p "$p" -- --demo < /dev/null; done
```

(El flag `--demo` solo lo usa el módulo 17 para no quedarse esperando como servidor; los demás lo ignoran.
El módulo 18 no tiene código Rust, sino Docker: el `[ -f Cargo.toml ]` lo salta.)

## De Go a Rust en una tabla

| Go | Rust |
|----|------|
| Recolector de basura | Ownership + `Drop` (sin GC, liberación determinista) |
| `nil` | `Option<T>` |
| `if err != nil { return err }` | `?` |
| Interfaces implícitas | Traits explícitos (`impl Trait for T`) |
| Goroutines | Hilos del SO (`std::thread`) o tareas async (`tokio::spawn`) |
| `go run -race` | Las carreras de datos no compilan |
| `defer` | `Drop` |
| `go.mod` / `go get` | `Cargo.toml` / `cargo add` |
| `gofmt`, `go vet` | `cargo fmt`, `cargo clippy` |

## Siguientes pasos

- [The Rust Programming Language ("el libro")](https://doc.rust-lang.org/book/) — [versión en español](https://book.rustlang-es.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) — ejercicios pequeños para corregir errores de compilación
- [Documentación de la biblioteca estándar](https://doc.rust-lang.org/std/)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial) — async en profundidad
- [blessed.rs](https://blessed.rs) — los crates recomendados por la comunidad
