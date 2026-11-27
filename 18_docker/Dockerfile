# syntax=docker/dockerfile:1
# =============================================================================
# MÓDULO 18 — IMAGEN DOCKER OPTIMIZADA PARA UNA APP RUST REAL
# Empaqueta la API del módulo 17 (HTTP + SQLite).
#
# Construir (desde la RAÍZ del tutorial, porque necesita el Cargo.toml del workspace):
#   docker build -f 18_docker/Dockerfile -t tareas-api .
#
# Resultado: un binario estático y unos pocos archivos sobre `scratch`
# (ver los números medidos en el README), frente a la versión ingenua
# (Dockerfile.ingenuo), que lleva dentro el toolchain completo de Rust.
# =============================================================================

# Versión de Rust como argumento: se cambia sin editar el archivo
#   docker build --build-arg RUST_VERSION=1.98 ...
ARG RUST_VERSION=1.99

# -----------------------------------------------------------------------------
# ETAPA 1: builder — tiene TODO lo necesario para compilar (~1 GB),
# pero nada de esto llegará a la imagen final.
# -----------------------------------------------------------------------------
# Alpine usa musl en vez de glibc, y el target nativo de la imagen
# (x86_64-unknown-linux-musl) genera binarios 100% ESTÁTICOS: no dependen de
# ninguna librería del sistema, así que pueden correr en `scratch`.
# La imagen trae gcc y musl-dev, necesarios para compilar SQLite (feature
# `bundled` de rusqlite, que es código C).
FROM rust:${RUST_VERSION}-alpine AS builder

WORKDIR /src

# 1) Preparar un workspace con SOLO el paquete que vamos a compilar.
#    El Cargo.toml raíz lista los 17 módulos del tutorial, y Cargo exige que
#    todos los miembros existan. En vez de enviar el tutorial entero al build,
#    reescribimos la lista de miembros. (En un proyecto normal, con un único
#    paquete o un workspace propio, este paso no hace falta.)
#    Cargo.lock se copia igualmente: las dependencias que quedan conservan sus
#    versiones exactas (Cargo solo poda las que ya no usa ningún miembro).
COPY Cargo.toml Cargo.lock ./
RUN sed -i '/^members = \[/,/^\]/c\members = ["17_proyecto_api_rest"]' Cargo.toml

# 2) Preparar el sistema de archivos mínimo que necesita la imagen final.
#    `scratch` está VACÍA: no hay usuarios, ni /tmp, ni certificados.
#    - /etc/passwd y /etc/group: para ejecutar como usuario sin privilegios
#      (65532 es la convención "nonroot" de las imágenes distroless)
#    - /data: donde vivirá la base de datos SQLite, propiedad de ese usuario
#    - /tmp: SQLite puede necesitar archivos temporales
RUN mkdir -p /rootfs/etc /rootfs/data /rootfs/tmp \
 && echo 'app:x:65532:65532:app:/nonexistent:/sbin/nologin' > /rootfs/etc/passwd \
 && echo 'app:x:65532:' > /rootfs/etc/group \
 && chown 65532:65532 /rootfs/data \
 && chmod 1777 /rootfs/tmp

# 3) Compilar.
#    --mount=type=bind   : monta el código (filtrado por Dockerfile.dockerignore)
#                          sin copiarlo a una capa
#    --mount=type=cache  : el registro de crates y target/ SOBREVIVEN entre
#                          builds. Si solo cambias código, Cargo no vuelve a
#                          descargar ni a compilar las ~150 dependencias: solo
#                          recompila tu crate. (Es lo que en Go hacía el paso
#                          separado de `go mod download`; con Cargo la caché de
#                          target/ es aún más importante, porque compilar
#                          dependencias es lo que más tarda.)
#                          Ojo: con LTO "fat" el enlazado final re-optimiza
#                          TODO el programa, así que aun con caché un rebuild
#                          tarda casi lo mismo que el primero (ver README).
#    Como target/ es una caché (no forma parte de la capa), el binario se
#    copia fuera en el MISMO paso.
#
#    --profile dist  : el perfil del Cargo.toml raíz con LTO, codegen-units=1,
#                      strip y panic=abort (binario más pequeño y rápido)
#    --locked        : NO; ver el punto 1 (el lock cambia al quitar miembros)
#    APP_VERSION     : se graba en el binario (option_env! en main.rs)
ARG VERSION=dev
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    --mount=type=bind,source=17_proyecto_api_rest,target=17_proyecto_api_rest \
    APP_VERSION=${VERSION} cargo build --profile dist -p tareas_api \
 && cp target/dist/tareas_api /usr/local/bin/app

# -----------------------------------------------------------------------------
# ETAPA OPCIONAL: test — ejecuta los tests dentro del mismo entorno de build.
# No forma parte de la imagen final; se invoca explícitamente en CI:
#   docker build -f 18_docker/Dockerfile --target test .
# -----------------------------------------------------------------------------
FROM builder AS test
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    --mount=type=bind,source=17_proyecto_api_rest,target=17_proyecto_api_rest \
    cargo test -p tareas_api

# -----------------------------------------------------------------------------
# ETAPA FINAL: scratch — imagen VACÍA. Solo copiamos lo imprescindible.
#   + tamaño mínimo (descarga y arranque instantáneos)
#   + superficie de ataque mínima: no hay shell, ni gestor de paquetes, ni
#     utilidades que un atacante pueda usar; los escáneres de CVE no
#     encuentran nada porque no hay nada
#   - no puedes hacer `docker exec -it ... sh` (no hay sh). Para depurar,
#     usa `docker debug` o cambia temporalmente a una imagen con shell.
#
# Alternativa casi igual de pequeña y algo más cómoda:
#   FROM gcr.io/distroless/static-debian12:nonroot
#   (ya trae certificados, zona horaria y el usuario nonroot)
# -----------------------------------------------------------------------------
FROM scratch AS final

# Certificados raíz: necesarios si la app hiciera peticiones HTTPS salientes
# (con reqwest + rustls/native-tls). Esta API no las hace, pero cuestan 200 KB
# y evitan un error desconcertante el día que se añadan.
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

COPY --from=builder /rootfs/ /
COPY --from=builder /usr/local/bin/app /app

# NUNCA ejecutar como root: si alguien explota la app, no tendrá privilegios.
USER 65532:65532

# Configuración por variables de entorno (la app las lee: ver leer_opciones)
ENV ADDR=:8080 \
    DB_PATH=/data/tareas.db

EXPOSE 8080
VOLUME ["/data"]

# Docker marca el contenedor como healthy/unhealthy. Como no hay curl,
# el propio binario hace la comprobación (flag --healthcheck).
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/app", "--healthcheck"]

# Forma "exec" (array JSON), NO "shell": la app es el PID 1 y recibe SIGTERM
# directamente en `docker stop` → se ejecuta el apagado ordenado de main.rs.
# (Además, en scratch no hay /bin/sh con el que usar la forma shell.)
ENTRYPOINT ["/app"]

# Metadatos estándar OCI (los muestran registries como GHCR o Docker Hub)
ARG VERSION=dev
LABEL org.opencontainers.image.title="tareas-api" \
      org.opencontainers.image.description="API REST de tareas del tutorial de Rust" \
      org.opencontainers.image.version="${VERSION}"
