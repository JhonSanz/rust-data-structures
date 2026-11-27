# 09 — Genéricos y lifetimes

```bash
cargo run -p genericos
```

## Temas

| Tema | Idea clave |
|------|-----------|
| Funciones genéricas | `fn mayor<T: PartialOrd + Copy>(xs: &[T]) -> Option<T>` |
| Restricciones (bounds) | Traits que debe cumplir `T`: `Display`, `Add<Output = T>`, `Default`... |
| `where` | Firmas legibles cuando hay muchas restricciones |
| `IntoIterator` | Aceptar arrays, `Vec`, iteradores... con una sola firma |
| Tipos genéricos | `struct Punto<T>`, `enum Arbol<T>`; `impl Punto<f64>` solo para un tipo |
| Tipos asociados | `trait Contenedor { type Item; }` (como `Iterator::Item`) |
| Const generics | `struct Matriz<const F: usize, const C: usize>` |
| Monomorfización | Una copia por tipo: coste cero en ejecución |
| Lifetimes | `fn mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str` |
| Elisión | Con una sola referencia de entrada (o `&self`) no se escriben |
| Structs con referencias | `struct Extracto<'a> { parte: &'a str }` |
| `'static` | Vive todo el programa (literales) |

## Para recordar

- Un lifetime **no cambia** cuánto vive nada: solo **describe** relaciones para que el compilador las compruebe.
- Si un struct con lifetimes te complica la vida, haz que sea dueño de sus datos (`String`, `Vec`).
