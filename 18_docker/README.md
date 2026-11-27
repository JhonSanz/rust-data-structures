# 18 — Imagen Docker optimizada

Empaqueta la API del [módulo 17](../17_proyecto_api_rest/) (HTTP + SQLite) en una
imagen de producción **mínima, segura y rápida de construir**.

> Todos los comandos se ejecutan desde la **raíz del tutorial**: el build necesita
> el `Cargo.toml` y el `Cargo.lock` del workspace, y el código del módulo 17.

```bash
# Construir
docker build -f 18_docker/Dockerfile --build-arg VERSION=1.0.0 -t tareas-api:1.0.0 .

# Ejecutar (con los datos en un volumen)
docker run -d --name tareas -p 8080:8080 -v tareas-datos:/data tareas-api:1.0.0
curl -s -X POST localhost:8080/tareas -H 'content-type: application/json' -d '{"titulo":"Desde Docker"}'
docker ps                       # al cabo de unos segundos: (healthy)
docker stop tareas && docker rm tareas

# Ejecutar los tests dentro del entorno de build (útil en CI)
docker build -f 18_docker/Dockerfile --target test .

# "Producción" con Docker Compose y hardening
docker compose -f 18_docker/compose.yaml up --build -d
docker compose -f 18_docker/compose.yaml down        # añade -v para borrar los datos

# Comparar con la versión ingenua
docker build -f 18_docker/Dockerfile.ingenuo -t tareas-api:ingenua .
```

## Archivos

| Archivo | Contenido |
|---------|-----------|
| `Dockerfile` | La imagen optimizada, **explicada línea a línea en comentarios** |
| `Dockerfile.dockerignore` | Lista blanca de lo que se envía al build |
| `Dockerfile.ingenuo` | Anti-ejemplo para comparar: así **no** se hace |
| `Dockerfile.ingenuo.dockerignore` | Solo ignora `target/` (si no, enviaría GB al daemon) |
| `compose.yaml` | Ejecución con volumen, límites de recursos y hardening |

## Resultados medidos

| | Ingenua | Optimizada |
|---|---|---|
| Tamaño en disco | **2,68 GB** | **5,5 MB** |
| Descarga (comprimida) | **676 MB** | **1,9 MB** (~360× menos) |
| Contenido | Debian + toolchain de Rust + todo el tutorial + `target/` | Binario estático + certificados + `passwd`/`group` |
| Usuario | `root` | `65532` (sin privilegios) |
| Shell / gestor de paquetes | Sí | No existen |
| RAM en ejecución | — | ~3,7 MiB |
| `docker stop` | `CMD ./tareas_api` (forma shell): el PID 1 es `/bin/sh`, y que la señal llegue a la app depende del shell | ~0,3 s: la app es el PID 1, recibe `SIGTERM` y se apaga de forma ordenada |

Tiempos de build (máquina de 28 hilos): primer build ~58 s; tras cambiar código del módulo 17, ~51 s (las dependencias no se recompilan, pero el LTO *fat* re-optimiza todo el programa al enlazar); tras cambiar **otro** módulo del tutorial, ~2,4 s (la lista blanca hace que ni siquiera llegue al build).

## Las técnicas, una por una

| Técnica | Qué consigue |
|---------|--------------|
| **Multi-stage build** | Se compila en una imagen de más de 1 GB, pero solo el binario pasa a la final |
| **`rust:*-alpine` (musl)** | Binario 100 % estático, sin libc dinámica: puede correr en `scratch` |
| **`rusqlite` con `bundled`** | SQLite se compila **dentro** del binario: no hace falta `libsqlite3` en la imagen |
| **`FROM scratch`** | Imagen vacía: no hay nada que atacar ni que parchear |
| **Perfil `dist`** | `lto`, `codegen-units = 1`, `strip`, `panic = "abort"`: binario más pequeño y rápido |
| **`APP_VERSION` + `option_env!`** | Inyecta la versión al compilar (se ve en el log al arrancar) |
| **`--mount=type=cache`** | Registro de crates y `target/` persisten entre builds: no se recompilan las dependencias |
| **`--mount=type=bind`** | Usa el código sin copiarlo a una capa |
| **Workspace recortado** | `sed` deja en el build solo el miembro que se compila |
| **`.dockerignore` como lista blanca** | Contexto mínimo, sin secretos ni `.git` ni `target/`; tocar otros módulos no invalida la caché |
| **`USER 65532`** | Nunca root |
| **`ENTRYPOINT ["/app"]` (forma exec)** | La app es el PID 1 y recibe `SIGTERM` → apagado ordenado (`with_graceful_shutdown`) |
| **`HEALTHCHECK` con el propio binario** | Sin `curl`: `app --healthcheck` |
| **Configuración por `ENV`** | `ADDR` y `DB_PATH`, sin reconstruir la imagen (12-factor) |
| **Etapa `test`** | Los tests corren en el mismo entorno que el build, y solo cuando se piden |

### Extra en `compose.yaml`

| Opción | Para qué |
|--------|----------|
| `read_only: true` + `tmpfs: /tmp` | Nadie puede escribir en el sistema de archivos del contenedor, salvo en `/tmp` y `/data` |
| `cap_drop: [ALL]` | Quita todas las capacidades de Linux |
| `no-new-privileges` | Bloquea la escalada de privilegios |
| `memory: 32M` | Sin recolector de basura no hay nada que ajustar (en Go haría falta `GOMEMLIMIT`) |

## Diferencias con el Dockerfile de Go

- **Compilación más lenta**: Rust compila mucho más despacio que Go, por eso la caché de `target/` es lo más importante del Dockerfile.
- **Multi-arquitectura**: Go compila de forma cruzada sin esfuerzo. En Rust con código C (SQLite) hace falta un compilador de C para la otra arquitectura. Opciones: `docker buildx build --platform linux/arm64` (emulación QEMU, lenta), [`cargo-zigbuild`](https://github.com/rust-cross/cargo-zigbuild) (usa Zig como compilador cruzado de C) o compilar en runners nativos de cada arquitectura.
- **Caché de dependencias en capas**: si prefieres capas en vez de `--mount=type=cache` (p. ej. en CI sin caché de BuildKit), mira [`cargo-chef`](https://github.com/LukeMathWalker/cargo-chef).
