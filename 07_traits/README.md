# 07 — Traits

```bash
cargo run -p traits
```

## Temas

| Tema | Idea clave |
|------|-----------|
| Definir / implementar | `trait Forma { fn area(&self) -> f64; }` + `impl Forma for Circulo` (explícito) |
| Métodos por defecto | El trait puede dar una implementación que se sobrescribe |
| Despacho estático | `fn f(x: &impl Forma)` o `fn f<T: Forma>(x: &T)` — coste cero |
| Despacho dinámico | `&dyn Forma`, `Box<dyn Forma>` — colecciones heterogéneas |
| Traits de std | `Display`, `Debug`, `Clone`, `PartialEq`, `Ord`, `Hash`, `Default`, `From/Into` |
| Operadores | `impl Add for Vector2` → `a + b`; tipos asociados (`type Output`) |
| Supertraits | `trait Resumible: Display` |
| Blanket impls | `impl<T: Display> Gritar for T` |
| `Any` | Downcasting en ejecución (el "type switch"), rara vez necesario |
| `io::Write` | El equivalente a `io.Writer`: stdout, archivos, `Vec<u8>`... |

## Comparado con las interfaces de Go

| Go | Rust |
|----|------|
| Implementación implícita | Implementación explícita (`impl T for X`) |
| Siempre dinámico (interfaz = puntero + tabla) | Estático por defecto, `dyn` cuando lo pides |
| `any` / `interface{}` | Genéricos, o `dyn Any` |
| Type switch | `enum` + `match` (preferido) o `downcast_ref` |
| `Stringer` | `Display` |
