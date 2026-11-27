# 01 — Fundamentos

```bash
cargo run -p fundamentos
```

## Temas

| Tema | Idea clave |
|------|-----------|
| Estructura de un programa | `fn main()` en `src/main.rs`; `println!` es una **macro** |
| Variables | `let x = 1` es **inmutable**; `let mut x` para poder cambiarla |
| Shadowing | `let x = x.trim();` crea una variable nueva con el mismo nombre (puede cambiar de tipo) |
| Tipos escalares | `i8..i128`, `u8..u128`, `isize/usize`, `f32/f64`, `bool`, `char` (4 bytes, Unicode) |
| Desbordamiento | Panic en debug; usa `checked_*`, `wrapping_*`, `saturating_*` para elegir |
| Tipos compuestos | Tuplas `(a, b)` y arrays `[T; N]` de tamaño fijo |
| Conversiones | `as` (puede perder datos), `From/Into` (infalibles), `TryFrom` y `parse` (devuelven `Result`) |
| Constantes | `const` (en compilación, tipo obligatorio) y `static` |
| `String` vs `&str` | Texto propio en el heap vs vista prestada. `len()` cuenta bytes |
| Formato | `{}` Display, `{:?}` Debug, `{:#?}`, `{x}` en línea, alineación, bases |

## Reglas que sorprenden al principio

- Las variables son **inmutables por defecto**.
- No hay "valores cero": usar una variable sin inicializar **no compila**.
- No hay conversiones implícitas, ni siquiera de `i32` a `i64`.
- `s[0]` sobre un `&str` **no compila**: un byte no es un carácter.
- `a + b` con `String` **mueve** `a`: después ya no puedes usarla.
