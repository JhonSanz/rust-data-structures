# 03 — Funciones

```bash
cargo run -p funciones
```

## Temas

| Tema | Idea clave |
|------|-----------|
| Declaración | `fn nombre(a: T) -> R { ... }`; tipos obligatorios en la firma |
| Retorno implícito | La última expresión **sin `;`** es el valor; `return` para salir antes |
| Varios retornos | Devuelve una tupla `(a, b)` y desestructúrala |
| Sin variádicas ni opcionales | Usa slices `&[T]`, `Option<T>` o un builder |
| Punteros a función | Tipo `fn(i32) -> i32` |
| Closures | `\|x\| x * factor` capturan su entorno |
| `Fn` / `FnMut` / `FnOnce` | Leer / modificar / consumir lo capturado |
| `move` | El closure toma posesión de lo que captura |
| Devolver closures | `-> impl Fn(i32) -> i32` |
| Recursión | Funciona igual que en otros lenguajes (sin optimización de cola garantizada) |
| Tipo `!` | Funciones que nunca vuelven (`panic!`, `process::exit`) |

## Reglas que sorprenden al principio

- No hay sobrecarga de funciones ni argumentos por defecto.
- Poner `;` tras la última expresión hace que la función devuelva `()` → error de tipos.
- Un closure con `move` se lleva las variables: fuera ya no las puedes usar.
