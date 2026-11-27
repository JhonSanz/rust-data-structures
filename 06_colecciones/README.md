# 06 — Colecciones

```bash
cargo run -p colecciones
```

## Temas

| Tema | Idea clave |
|------|-----------|
| `Vec<T>` | `vec![]`, `push`, `pop`, `extend`, `insert`, `remove`, `retain`, `len`/`capacity` |
| Acceso | `v[i]` (panic si no existe) vs `v.get(i)` (`Option`) |
| Recorrer | `for x in &v` (leer), `for x in &mut v` (modificar), `for x in v` (consumir) |
| Slices | `sort`, `sort_by`, `binary_search`, `chunks`, `windows`, `split_at`, `dedup`, `concat`, `join` |
| `String` | `push_str`, `split`, `split_once`, `trim`, `replace`, `lines` |
| `HashMap` | `insert`, `get` → `Option`, `get_mut`, `remove`; orden **no** definido |
| Entry API | `*m.entry(k).or_insert(0) += 1` y `.or_default().push(x)` |
| `BTreeMap` | Ordenado por clave; `range(80..)`, `first_key_value` |
| `HashSet` | `union`, `intersection`, `difference`; `insert` devuelve `bool` |
| `VecDeque` / `BinaryHeap` | Colas FIFO y de prioridad (`Reverse` para min-heap) |

## Reglas que sorprenden al principio

- `map.get(k)` devuelve `Option<&V>`: no existe el "valor cero" de Go.
- No puedes tener una referencia a un elemento de un `Vec` y hacer `push` a la vez: el borrow checker lo impide (en Go sería un bug silencioso de aliasing).
- `HashMap` no tiene orden; si lo necesitas, ordena las claves o usa `BTreeMap`.
