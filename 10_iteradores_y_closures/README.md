# 10 — Iteradores

```bash
cargo run -p iteradores
```

Los closures se presentaron en el módulo 03; aquí se ven en su uso principal: como argumentos de los iteradores.

## Temas

| Tema | Idea clave |
|------|-----------|
| El trait `Iterator` | Solo hay que implementar `next() -> Option<Item>`; el resto viene gratis |
| `iter` / `iter_mut` / `into_iter` | `&T` / `&mut T` / `T` (consume) |
| Adaptadores (perezosos) | `map`, `filter`, `filter_map`, `zip`, `enumerate`, `chain`, `flat_map`, `flatten`, `skip`, `take_while`, `step_by`, `rev`, `scan`, `peekable` |
| Consumidores | `collect`, `sum`, `product`, `fold`, `reduce`, `count`, `min/max(_by_key)`, `any`, `all`, `find`, `position`, `partition`, `unzip` |
| `collect` | Construye `Vec`, `String`, `HashMap`, `BTreeSet`, `Result<Vec<_>,_>`... según el tipo anotado |
| Pereza | Nada se ejecuta hasta que se consume; permite iteradores infinitos `(1..)` |
| Iteradores propios | `impl Iterator for Fibonacci` |
| Generadores | `iter::from_fn`, `iter::successors`, `iter::repeat_n` |

## Para recordar

- Los iteradores son **coste cero**: igual de rápidos que un bucle a mano.
- Encadenar adaptadores es idiomático; si la cadena se vuelve ilegible, un `for` también lo es.
- `let [a, b, c] = slice[..] else { ... }` desestructura un slice de longitud conocida.
