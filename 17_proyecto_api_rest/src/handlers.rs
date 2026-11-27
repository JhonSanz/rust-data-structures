// =============================================================================
// CAPA HTTP: router de axum, handlers, middleware y traducción de errores.
// =============================================================================
//
// axum (del equipo de Tokio) es el framework web más usado en Rust. Un handler
// es una función async cuyos parámetros son EXTRACTORES (Path, Json, State...)
// y cuyo resultado implementa IntoResponse. Si un extractor falla, axum
// responde solo; aquí capturamos esos fallos para responder siempre en JSON.

use crate::modelo::{ErrorDominio, Repositorio, Resultado, Tarea};
use axum::extract::rejection::JsonRejection;
use axum::extract::{DefaultBodyLimit, Path, Request, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use std::time::Instant;

/// Estado compartido por todos los handlers. `Arc<dyn Repositorio>`: un
/// puntero compartido a "algún tipo que implementa Repositorio".
#[derive(Clone)]
pub struct EstadoApp {
    repo: Arc<dyn Repositorio>,
}

/// Construye la aplicación completa. Es lo que usan main.rs y los tests.
pub fn app(repo: Arc<dyn Repositorio>) -> Router {
    Router::new()
        .route("/salud", get(salud))
        .route("/tareas", get(listar_tareas).post(crear_tarea))
        .route(
            "/tareas/{id}",
            get(obtener_tarea)
                .patch(actualizar_tarea)
                .delete(eliminar_tarea),
        )
        .fallback(no_encontrado)
        .layer(DefaultBodyLimit::max(1 << 20)) // límite de 1 MB por cuerpo
        .layer(middleware::from_fn(con_logging))
        .with_state(EstadoApp { repo })
}

// -----------------------------------------------------------------------------
// Errores de la API → respuestas HTTP
// -----------------------------------------------------------------------------
pub enum ErrorApi {
    Dominio(ErrorDominio),
    Peticion(String), // 400 con un mensaje para el cliente
}

impl From<ErrorDominio> for ErrorApi {
    fn from(e: ErrorDominio) -> Self {
        ErrorApi::Dominio(e)
    }
}

impl From<JsonRejection> for ErrorApi {
    fn from(e: JsonRejection) -> Self {
        ErrorApi::Peticion(format!("JSON inválido: {}", e.body_text()))
    }
}

impl IntoResponse for ErrorApi {
    fn into_response(self) -> Response {
        let (estado, mensaje) = match self {
            ErrorApi::Peticion(m) => (StatusCode::BAD_REQUEST, m),
            ErrorApi::Dominio(e) => match e {
                ErrorDominio::NoEncontrada => (StatusCode::NOT_FOUND, e.to_string()),
                ErrorDominio::TituloVacio | ErrorDominio::TituloLargo => {
                    (StatusCode::BAD_REQUEST, e.to_string())
                }
                ErrorDominio::Interno(detalle) => {
                    // El detalle va al log; al cliente, un mensaje genérico
                    // (nunca filtres rutas, SQL o versiones a un posible atacante).
                    eprintln!("ERROR interno: {detalle}");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "error interno".to_string(),
                    )
                }
            },
        };
        (estado, Json(json!({ "error": mensaje }))).into_response()
    }
}

// Ejecuta una operación SÍNCRONA del repositorio en el pool de hilos de
// bloqueo de Tokio, para no parar el runtime async mientras SQLite trabaja.
async fn en_repo<T, F>(estado: &EstadoApp, operacion: F) -> Result<T, ErrorApi>
where
    T: Send + 'static,
    F: FnOnce(&dyn Repositorio) -> Resultado<T> + Send + 'static,
{
    let repo = Arc::clone(&estado.repo);
    tokio::task::spawn_blocking(move || operacion(repo.as_ref()))
        .await
        .map_err(|e| ErrorDominio::Interno(format!("tarea abortada: {e}")))?
        .map_err(ErrorApi::from)
}

fn leer_id(texto: &str) -> Result<i64, ErrorApi> {
    match texto.parse::<i64>() {
        Ok(id) if id > 0 => Ok(id),
        _ => Err(ErrorApi::Peticion("id inválido".into())),
    }
}

// -----------------------------------------------------------------------------
// Handlers
// -----------------------------------------------------------------------------
async fn salud() -> Json<serde_json::Value> {
    Json(json!({ "estado": "ok" }))
}

async fn listar_tareas(State(estado): State<EstadoApp>) -> Result<Json<Vec<Tarea>>, ErrorApi> {
    Ok(Json(en_repo(&estado, |r| r.listar()).await?))
}

async fn obtener_tarea(
    State(estado): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Json<Tarea>, ErrorApi> {
    let id = leer_id(&id)?;
    Ok(Json(en_repo(&estado, move |r| r.obtener(id)).await?))
}

// deny_unknown_fields: rechaza campos que no esperamos (p. ej. un typo "titlo")
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EntradaCrear {
    titulo: String,
}

async fn crear_tarea(
    State(estado): State<EstadoApp>,
    // Result<Json<T>, JsonRejection>: capturamos el error del extractor
    // para responder con NUESTRO formato JSON de error.
    cuerpo: Result<Json<EntradaCrear>, JsonRejection>,
) -> Result<Response, ErrorApi> {
    let Json(entrada) = cuerpo?;
    let tarea = en_repo(&estado, move |r| r.crear(&entrada.titulo)).await?;
    let location = HeaderValue::from_str(&format!("/tareas/{}", tarea.id)).expect("ASCII válido");
    Ok((
        StatusCode::CREATED,
        [(header::LOCATION, location)],
        Json(tarea),
    )
        .into_response())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EntradaActualizar {
    // Option: distingue "no vino el campo" (None) de "vino false"
    completada: Option<bool>,
}

async fn actualizar_tarea(
    State(estado): State<EstadoApp>,
    Path(id): Path<String>,
    cuerpo: Result<Json<EntradaActualizar>, JsonRejection>,
) -> Result<Json<Tarea>, ErrorApi> {
    let id = leer_id(&id)?;
    let Json(entrada) = cuerpo?;
    let Some(completada) = entrada.completada else {
        return Err(ErrorApi::Peticion("falta el campo 'completada'".into()));
    };
    Ok(Json(
        en_repo(&estado, move |r| r.actualizar(id, completada)).await?,
    ))
}

async fn eliminar_tarea(
    State(estado): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<StatusCode, ErrorApi> {
    let id = leer_id(&id)?;
    en_repo(&estado, move |r| r.eliminar(id)).await?;
    Ok(StatusCode::NO_CONTENT) // 204: éxito sin cuerpo
}

async fn no_encontrado() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "ruta no encontrada" })),
    )
}

// -----------------------------------------------------------------------------
// Middleware: registra cada petición con su estado y duración.
// -----------------------------------------------------------------------------
// (En proyectos reales: tower_http::trace::TraceLayer + el crate `tracing`.)
async fn con_logging(peticion: Request, siguiente: Next) -> Response {
    let inicio = Instant::now();
    let metodo = peticion.method().clone();
    let ruta = peticion.uri().path().to_owned();
    let respuesta = siguiente.run(peticion).await;
    println!(
        "INFO petición metodo={metodo} ruta={ruta} estado={} duracion={:?}",
        respuesta.status().as_u16(),
        inicio.elapsed()
    );
    respuesta
}
