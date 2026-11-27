# 05 — Structs y enums

```bash
cargo run -p structs_y_enums
```

## Temas

| Tema | Idea clave |
|------|-----------|
| Structs | `struct Usuario { nombre: String, ... }`; mutabilidad de la variable completa |
| Actualización | `Usuario { nombre, ..otro }` (¡puede mover campos de `otro`!) |
| Structs tupla / newtypes | `struct Metros(f64)` evita mezclar unidades |
| `impl` | Funciones asociadas (`Tipo::new`) y métodos |
| Receptores | `&self` (leer), `&mut self` (modificar), `self` (consumir) |
| `#[derive]` | `Debug`, `Clone`, `Copy`, `PartialEq`, `Default`... gratis |
| `Display` | Implementarlo para usar `{}` |
| Enums con datos | Cada variante lleva sus propios datos: tipos suma |
| Modelar estados | Los estados imposibles no se pueden representar |
| `Option<T>` | `Some(x)` / `None`: el sustituto seguro de `null` |
| Builder | Encadenar `.metodo().timeout().build()` en vez de parámetros opcionales |

## Comparado con Go

| Go | Rust |
|----|------|
| `type T struct` + métodos | `struct T` + `impl T` |
| Receptor valor / puntero | `self` / `&self` / `&mut self` |
| Constructor `NewT()` | `T::new()` (convención) |
| Embedding | Composición explícita + traits |
| Struct tags | Atributos: `#[derive(Serialize)]`, `#[serde(rename = "...")]` |
| `iota` | `enum` de verdad, con datos y `match` exhaustivo |
| `nil` | `Option<T>` |
