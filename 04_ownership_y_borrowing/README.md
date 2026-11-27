# 04 — Ownership y borrowing

```bash
cargo run -p ownership
```

Este es **el** módulo de Rust: equivale a los punteros del tour de Go, pero va mucho más lejos.

## Las tres reglas

1. Cada valor tiene **un único dueño**.
2. Cuando el dueño sale del ámbito, el valor se **libera** (`Drop`).
3. Puedes **prestarlo**: muchas `&T` **o** una sola `&mut T`, nunca ambas; y una referencia no puede vivir más que el valor.

## Temas

| Tema | Idea clave |
|------|-----------|
| Ámbito y `Drop` | Liberación determinista al salir del bloque (sustituye a `defer` y al GC) |
| Move | `let b = a;` con `String`/`Vec` **mueve**: `a` queda inválida |
| `Copy` | Enteros, `bool`, `char`, `f64`... se copian; no se mueven |
| `Clone` | Copia profunda **explícita**: `a.clone()` |
| Funciones | Pasar por valor mueve; pasar `&x` presta |
| `&T` | Referencias compartidas (solo lectura) |
| `&mut T` | Referencia exclusiva (lectura y escritura) |
| NLL | El préstamo termina en su **último uso**, no al cerrar el bloque |
| Slices | `&s[1..4]`, `&str`, `&[T]`, `&mut [T]` |
| `Box<T>` | Valor en el heap con un único dueño |

## Cómo leer los errores del borrow checker

| Error | Significa | Solución típica |
|-------|-----------|-----------------|
| `borrow of moved value` | Usaste algo después de cederlo | Presta con `&`, o `.clone()` si de verdad necesitas dos copias |
| `cannot borrow as mutable more than once` | Dos `&mut` vivas a la vez | Acorta el uso de la primera |
| `cannot borrow as mutable because it is also borrowed as immutable` | Lees y modificas a la vez | Copia el dato que lees, o reordena |
| `does not live long enough` | La referencia sobrevive al valor | Devuelve un valor propio (`String`) en vez de una referencia |

> Los errores del compilador de Rust son excepcionalmente buenos: léelos enteros, suelen incluir la solución.
