# Retos

92 problemas, uno o varios por cada módulo del tour, para comprobar si de verdad
has aprendido. Cada reto es una función con un `todo!()` que tienes que
implementar. Los tests ya escritos dicen si tu solución es correcta.

```bash
cargo run -p retos               # tabla con tu progreso en todos los módulos
cargo run -p retos -- r04        # detalle de un módulo: qué falla y por qué
cargo test -p retos --test r04   # lo mismo, con cargo test directamente
cargo test -p retos --test r04 reto_3   # un solo reto
```

```
       Módulo                   Progreso     Retos
  r01  Fundamentos              ████░░░░░░ 3/6   ✅1 ✅2 ✅3 ⬜4 ⬜5 ⬜6
  r02  Control de flujo         ░░░░░░░░░░ 0/5   ⬜1 ⬜2 ⬜3 ⬜4 ⬜5
  r04  Ownership y borrowing    ░░░░░░░░░░       🔧 no compila — mira los errores: cargo run -p retos -- r04
  ...
```

## Cómo trabajar

1. Haz primero el módulo del tour (`01_fundamentos/`, etc.) y luego abre su archivo de retos en `ejercicios/`.
2. Lee el enunciado de cada reto (el comentario `///` encima de la función) y sustituye el `todo!()` por tu código.
3. Ejecuta `cargo test -p retos --test rNN` hasta que pase. Lee los mensajes del test: dicen qué valor esperaban.
4. Cuando termines un módulo, compara tu solución con la de `soluciones/`. Suele haber varias formas válidas, y ver otra también enseña.

## Reglas

- **No modifiques los tests** (las funciones `#[test]`). Si un test te parece mal, probablemente no has entendido el enunciado.
- **Retos «ARREGLA»**: el código no compila a propósito y tienes que arreglarlo. Ahí sí puedes cambiar la firma o el cuerpo de la función. Pasa en **r04, r09 y r14**: el archivo entero no compila hasta que los arregles, y el comprobador lo marca con 🔧.
- Algunos módulos añaden una regla propia en su cabecera: sin `.clone()` (r04), sin `unwrap` (r08, r16), sin bucles (r10)... Los tests no siempre pueden comprobarlo. Cumplirla es parte del reto.
- En **r15** los papeles se invierten: tú escribes los tests. Un test que pasa sin comprobar nada no demuestra nada.
- No mires `soluciones/` hasta haberlo intentado de verdad. Atascarse y leer el error del compilador es justo lo que enseña.

## Qué se practica

| Archivo | Retos | Temas |
|---------|-------|-------|
| `r01_fundamentos.rs` | 6 | Aritmética, tuplas, `char`/`String`, `TryFrom`, `format!`, desbordamiento |
| `r02_control_de_flujo.rs` | 5 | `match` con tuplas y rangos, `loop` + `break valor`, etiquetas, `let else` |
| `r03_funciones.rs` | 5 | `impl Fn`, devolver closures, `FnMut` con estado, recursión (Hanói) |
| `r04_ownership.rs` | 5 | 🔧 move vs préstamo, 🔧 conflicto `&`/`&mut`, devolver slices, `mem::take` |
| `r05_structs_y_enums.rs` | 5 | Métodos, `self` por valor, enums con datos (robot), `Option` encadenado, máquina de estados |
| `r06_colecciones.rs` | 6 | Entry API, `BTreeMap`, `HashSet`, ordenar por varios criterios, invertir un mapa |
| `r07_traits.rs` | 6 | Métodos por defecto, `Display`, `Add`/`Mul`, `From`, `dyn Trait` (fracciones y formas) |
| `r08_errores.rs` | 6 | Errores propios, `Display`, `?`, `From` para convertir errores, `Box<dyn Error>`, `collect` en `Result` |
| `r09_genericos_y_lifetimes.rs` | 6 | Trait bounds, tipo genérico, 🔧 lifetimes en funciones y structs, `where`, `impl` con restricciones |
| `r10_iteradores.rs` | 6 | Adaptadores sin bucles, iterador propio (Collatz), `fold`, `windows`, iteradores infinitos |
| `r11_smart_pointers.rs` | 5 | Lista con `Box`, `Rc`, `RefCell`, `Rc<RefCell<T>>`, `Cow` |
| `r12_concurrencia.rs` | 5 | `thread::scope`, `Arc<Mutex>`, worker pool con canales, atómicos, `OnceLock` |
| `r13_async.rs` | 6 | `async fn`, `spawn`, `timeout`, `select!`, canales, cancelación (con reloj simulado) |
| `r14_modulos.rs` | 4 | 🔧 visibilidad (`pub`, `pub use`, `pub(in ...)`), `macro_rules!` y macros recursivas, `cfg!` |
| `r15_testing.rs` | 5 | Escribir tests table-driven, encontrar un bug, `should_panic`, tests con `Result`, `#[track_caller]` |
| `r16_biblioteca_estandar.rs` | 6 | `BufReader`, `BufWriter`, `Duration`, `sort_by` + `then_with`, `Command`, `env::var` |
| `r17_api.rs` | 5 | Handlers de axum con estado, `Path`, `Json` y errores, un **decorador** del `Repositorio` del módulo 17 |

## Estructura

```
retos/
├── Cargo.toml        un [[test]] por archivo: si uno no compila, los demás siguen funcionando
├── src/main.rs       el comprobador de progreso (cargo run -p retos)
├── ejercicios/       ← aquí trabajas tú
└── soluciones/       soluciones de referencia (no se compilan)
```

> Si quieres volver a empezar un módulo, copia el archivo original desde git o
> vacía tus funciones volviendo a poner `todo!()`.
