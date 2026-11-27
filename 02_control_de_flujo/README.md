# 02 — Control de flujo

```bash
cargo run -p control_de_flujo
```

## Temas

| Tema | Idea clave |
|------|-----------|
| `if` como expresión | `let x = if c { a } else { b };` — no hay ternario |
| `loop` | Bucle infinito; `break valor` devuelve un valor |
| `while` / `for` | `for x in iterable` es el bucle idiomático (sin índices fuera de rango) |
| Rangos | `a..b` (excluye), `a..=b` (incluye), `.rev()`, `.step_by(n)` |
| Etiquetas | `'exterior: for ...` + `break 'exterior` |
| `match` | Patrones, `\|`, rangos, guardas `if`, `_`. **Exhaustivo** |
| `if let` / `let else` / `while let` | Atajos para un solo patrón |
| Cadenas de `let` | `if let Some(x) = a && x > 0 { }` (edición 2024) |
| Bloques | `{ ...; valor }` es una expresión; con `;` final vale `()` |

## Reglas que sorprenden al principio

- La condición del `if` debe ser `bool`: `if n {}` con un número no compila.
- Un `;` de más al final de una función o bloque cambia su valor a `()`.
- Un `match` que no cubre todos los casos **no compila**.
- No hay `defer`: la limpieza la hace `Drop` al salir del ámbito (módulo 04).
