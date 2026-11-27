# 17 — Proyecto final: API REST de tareas

Una API HTTP completa con **dos adaptadores de persistencia intercambiables**:
memoria y SQLite con SQL puro.

```bash
cargo run -p tareas_api -- --demo                  # demo automática, en memoria
cargo run -p tareas_api -- --demo --db tareas.db   # demo con SQLite (ejecútala 2 veces)
cargo run -p tareas_api -- --db tareas.db          # servidor en :8080 con SQLite
cargo run -p tareas_api -- --addr :9000            # otro puerto, en memoria
cargo test -p tareas_api                           # tests contra ambos adaptadores
```

> Fíjate en el `--`: separa los argumentos de `cargo` de los de tu programa.

Con el servidor arrancado, en otra terminal:

```bash
curl -s localhost:8080/tareas
curl -s -X POST localhost:8080/tareas -H 'content-type: application/json' -d '{"titulo":"Aprender Rust"}'
curl -s localhost:8080/tareas/1
curl -s -X PATCH localhost:8080/tareas/1 -H 'content-type: application/json' -d '{"completada":true}'
curl -s -X DELETE localhost:8080/tareas/1 -i
```

## Endpoints

| Método | Ruta | Respuesta |
|--------|------|-----------|
| GET | `/salud` | 200 `{"estado":"ok"}` |
| GET | `/tareas` | 200 lista de tareas |
| POST | `/tareas` | 201 tarea creada + `Location` / 400 si el título o el JSON son inválidos |
| GET | `/tareas/{id}` | 200 / 404 / 400 |
| PATCH | `/tareas/{id}` | 200 tarea actualizada / 404 / 400 |
| DELETE | `/tareas/{id}` | 204 / 404 |

Todos los errores tienen la forma `{"error": "mensaje"}`. Los 500 nunca revelan el detalle interno.

## Arquitectura: puertos y adaptadores

```
          HTTP                    dominio                       persistencia
 ┌──────────────────┐     ┌───────────────────────┐     ┌──────────────────────────┐
 │   handlers.rs    │────▶│      modelo.rs        │◀────│ repositorio_memoria.rs   │
 │ (solo conoce     │     │  Tarea, ErrorDominio, │     │   BTreeMap + RwLock      │
 │ Arc<dyn Repo>)   │     │  trait Repositorio    │◀────│ repositorio_sqlite.rs    │
 └──────────────────┘     └───────────────────────┘     │   rusqlite + SQL puro    │
                                    ▲                   └──────────────────────────┘
                          main.rs elige el adaptador con --db
```

## Archivos

| Archivo | Contenido |
|---------|-----------|
| `src/main.rs` | Flags + variables de entorno, `run() -> Result`, elección de adaptador, apagado ordenado (Ctrl+C / SIGTERM), demo con un cliente HTTP mínimo sobre TCP, `--healthcheck` |
| `src/lib.rs` | Declara los módulos y re-exporta la API pública (para los tests) |
| `src/modelo.rs` | **Dominio**: `Tarea` (`Serialize`), `ErrorDominio`, trait `Repositorio`, validación |
| `src/repositorio_memoria.rs` | **Adaptador** en memoria con `RwLock<BTreeMap>` |
| `src/repositorio_sqlite.rs` | **Adaptador** SQLite: esquema, `query_row`, `query_map`, `execute`, `RETURNING`, `optional()` + tests |
| `src/handlers.rs` | Router de axum, extractores, `IntoResponse` para errores, `spawn_blocking`, middleware de log |
| `tests/api.rs` | Tests de la API ejecutados contra **ambos** adaptadores con `tower::ServiceExt::oneshot` |

## SQL que se usa

```sql
CREATE TABLE IF NOT EXISTS tareas (...);                                 -- migración al arrancar
SELECT id, titulo, completada, creada_en FROM tareas ORDER BY id;        -- listar
SELECT ... FROM tareas WHERE id = ?1;                                    -- obtener
INSERT INTO tareas (titulo, completada, creada_en) VALUES (?1, 0, ?2);   -- crear + last_insert_rowid
UPDATE tareas SET completada = ?1 WHERE id = ?2 RETURNING ...;           -- actualizar en 1 query
DELETE FROM tareas WHERE id = ?1;                                        -- eliminar + filas afectadas
```

## rusqlite: lo esencial

| Método | Para qué |
|--------|----------|
| `Connection::open(ruta)` / `open_in_memory()` | Abre (o crea) la base de datos |
| `execute_batch(sql)` | Varias sentencias sin parámetros (esquema) |
| `execute(sql, params![..])` | Sentencias sin filas → nº de filas afectadas |
| `query_row(sql, params, \|fila\| ..)` | Una fila; `.optional()` convierte "sin filas" en `Ok(None)` |
| `prepare_cached` + `query_map` | Varias filas como iterador de `Result` |
| `last_insert_rowid()` | Id generado por el último `INSERT` |
| Placeholders `?1, ?2` | Evitan la **inyección SQL**: nunca concatenes datos en la query |

## Dependencias

| Crate | Para qué |
|-------|----------|
| `axum` | Framework web (router, extractores, respuestas) |
| `tokio` | Runtime async, `TcpListener`, señales, `spawn_blocking` |
| `serde` + `serde_json` | JSON con `#[derive(Serialize, Deserialize)]` |
| `rusqlite` (feature `bundled`) | SQLite compilado dentro del binario |
| `tower`, `http-body-util` | Solo para tests (`oneshot`, leer el cuerpo) |

Alternativas habituales: `sqlx` (SQL async comprobado en compilación, PostgreSQL/MySQL/SQLite), `diesel` o `sea-orm` (ORMs), `reqwest` (cliente HTTP), `tower-http` (CORS, compresión, trazas), `tracing` (logs).

## Conceptos de módulos anteriores que se aplican

- **Structs y enums** (05): `Tarea`, `ErrorDominio`, `ErrorApi`, `Opciones`.
- **Traits** (07): `Repositorio`, `IntoResponse`, `From` para usar `?`, `Display`, `Error`.
- **Errores** (08): traducir `rusqlite::Error` a `ErrorDominio` y este a HTTP; `Box<dyn Error>` en `main`.
- **Genéricos** (09): `en_repo<T, F>` con `FnOnce + Send + 'static`.
- **Smart pointers** (11): `Arc<dyn Repositorio>` compartido por todas las peticiones.
- **Concurrencia** (12): `RwLock`, `Mutex<Connection>` (una `Connection` no es `Sync`).
- **Async** (13): handlers `async`, `select!` para señales, `oneshot` para apagar la demo, `spawn_blocking`.
- **Módulos** (14): lógica en `lib.rs`, binario fino en `main.rs`.
- **Testing** (15): tests de integración, tests por adaptador, `#[tokio::test]`, concurrencia.
