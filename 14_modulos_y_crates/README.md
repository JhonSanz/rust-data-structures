# 14 — Módulos y crates

```bash
cargo run -p modulos
APP_ENV=produccion cargo run -p modulos --features cientifica
cargo test -p modulos                     # tests unitarios + doc-tests
cargo doc -p modulos --open               # documentación generada desde los ///
```

## Estructura

```
14_modulos_y_crates/
├── Cargo.toml                [features] cientifica = []
└── src/
    ├── lib.rs                crate biblioteca `modulos`: mod, pub use, macro_rules!
    ├── main.rs               crate binario: usa la biblioteca como si fuera externa
    ├── config.rs             módulo PRIVADO (como internal/ en Go)
    ├── calculadora.rs        módulo `calculadora` (pub)
    └── calculadora/
        └── historial.rs      submódulo `calculadora::historial`
```

## Temas

| Tema | Idea clave |
|------|-----------|
| Paquete vs crate | Un `Cargo.toml` = un paquete; puede tener una lib (`lib.rs`) y varios binarios |
| `mod x;` | Carga `src/x.rs`; sus submódulos van en `src/x/*.rs` |
| Módulos en línea | `mod formato { ... }` dentro del mismo archivo |
| Visibilidad | Privado por defecto; `pub`, `pub(crate)`, `pub(super)` |
| Campos privados | Un `struct` público con campos privados obliga a usar el constructor |
| Rutas | `crate::`, `super::`, `self::`, nombre de crate externo |
| `use` | Agrupar `{A, B}`, renombrar `as`, re-exportar `pub use` |
| Features | `#[cfg(feature = "x")]`, `cfg!(...)`, `--features x` |
| Macros | `macro_rules!` + `#[macro_export]` |
| Documentación | `///` para items, `//!` para el módulo; los ejemplos son **doc-tests** |
| `env!` | Variables de Cargo en compilación (`CARGO_PKG_VERSION`) |

## Cargo: comandos esenciales

```bash
cargo new mi_app            # binario      | cargo new --lib mi_lib  → biblioteca
cargo add serde --features derive          # añadir dependencia (edita Cargo.toml)
cargo remove serde
cargo update                # actualizar dentro de los rangos semver de Cargo.toml
cargo tree                  # árbol de dependencias
cargo build --release       # optimizado, en target/release/
cargo check                 # comprobar que compila, sin generar binario (rápido)
```

`Cargo.lock` fija las versiones exactas. Este tutorial es un **workspace** (ver el `Cargo.toml` de la raíz): todos los paquetes comparten `Cargo.lock`, `target/` y las versiones de `[workspace.dependencies]`.
