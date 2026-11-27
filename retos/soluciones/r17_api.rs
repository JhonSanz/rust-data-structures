// =============================================================================
// RETOS 17 — UNA API CON AXUM                    (repasa: 17_proyecto_api_rest)
// Comprueba tu solución:   cargo test -p retos --test r17
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.
// El router ya está montado: te toca implementar los handlers.

use axum::body::Body;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::{Request, StatusCode, header};
use axum::routing::{get, post};
use axum::{Json, Router};
use http_body_util::BodyExt;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use tareas_api::{ErrorDominio, Repositorio, RepositorioMemoria, Tarea};
use tower::ServiceExt;

#[derive(Clone, Default)]
struct Estado {
    contador: Arc<AtomicU64>,
}

fn app() -> Router {
    Router::new()
        .route("/contador", get(ver_contador))
        .route("/contador/incrementar", post(incrementar))
        .route("/contador/reiniciar", post(reiniciar))
        .route("/saludo/{nombre}", get(saludo))
        .route("/eco", post(eco))
        .with_state(Estado::default())
}

/// Envía una petición al router en memoria y devuelve (estado, cuerpo JSON o Null).
async fn enviar(
    app: &Router,
    metodo: &str,
    ruta: &str,
    cuerpo: Option<Value>,
) -> (StatusCode, Value) {
    let mut p = Request::builder().method(metodo).uri(ruta);
    let body = match cuerpo {
        Some(j) => {
            p = p.header(header::CONTENT_TYPE, "application/json");
            Body::from(j.to_string())
        }
        None => Body::empty(),
    };
    let r = app.clone().oneshot(p.body(body).unwrap()).await.unwrap();
    let estado = r.status();
    let bytes = r.into_body().collect().await.unwrap().to_bytes();
    (
        estado,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

/// RETO 1 — Leer el estado compartido
/// GET /contador → 200 {"valor": n}
async fn ver_contador(State(estado): State<Estado>) -> Json<Value> {
    Json(json!({ "valor": estado.contador.load(Ordering::SeqCst) }))
}

/// RETO 2 — Modificar el estado compartido
/// POST /contador/incrementar → 200 {"valor": n + 1}  (devuelve el valor NUEVO)
/// POST /contador/reiniciar   → 204 sin cuerpo, y el contador vuelve a 0
async fn incrementar(State(estado): State<Estado>) -> Json<Value> {
    let nuevo = estado.contador.fetch_add(1, Ordering::SeqCst) + 1;
    Json(json!({ "valor": nuevo }))
}

async fn reiniciar(State(estado): State<Estado>) -> StatusCode {
    estado.contador.store(0, Ordering::SeqCst);
    StatusCode::NO_CONTENT
}

#[tokio::test]
async fn reto_1_ver_contador() {
    let app = app();
    assert_eq!(
        enviar(&app, "GET", "/contador", None).await,
        (StatusCode::OK, json!({"valor": 0}))
    );
}

#[tokio::test]
async fn reto_2_incrementar_y_reiniciar() {
    let app = app();
    assert_eq!(
        enviar(&app, "POST", "/contador/incrementar", None).await.1,
        json!({"valor": 1})
    );
    assert_eq!(
        enviar(&app, "POST", "/contador/incrementar", None).await.1,
        json!({"valor": 2})
    );
    assert_eq!(
        enviar(&app, "GET", "/contador", None).await.1,
        json!({"valor": 2})
    );
    assert_eq!(
        enviar(&app, "POST", "/contador/reiniciar", None).await,
        (StatusCode::NO_CONTENT, Value::Null)
    );
    assert_eq!(
        enviar(&app, "GET", "/contador", None).await.1,
        json!({"valor": 0})
    );
}

/// RETO 3 — Parámetros de ruta
/// GET /saludo/{nombre} → 200 {"mensaje": "¡Hola, {nombre}!"}
async fn saludo(Path(nombre): Path<String>) -> Json<Value> {
    Json(json!({ "mensaje": format!("¡Hola, {nombre}!") }))
}

#[tokio::test]
async fn reto_3_saludo() {
    let app = app();
    assert_eq!(
        enviar(&app, "GET", "/saludo/Ferris", None).await,
        (StatusCode::OK, json!({"mensaje": "¡Hola, Ferris!"}))
    );
}

/// RETO 4 — Cuerpo JSON y errores propios
/// POST /eco con {"texto": "..."}:
///   - 200 {"texto": EN MAYÚSCULAS, "longitud": nº de caracteres del original}
///   - 400 {"error": "texto vacío"}  si el texto está vacío o solo tiene espacios
///   - 400 {"error": "JSON inválido"} si el cuerpo no es JSON válido o falta el campo
/// Pista: recibe `Result<Json<EntradaEco>, JsonRejection>` para capturar los errores del extractor.
#[derive(Deserialize)]
struct EntradaEco {
    texto: String,
}

async fn eco(cuerpo: Result<Json<EntradaEco>, JsonRejection>) -> (StatusCode, Json<Value>) {
    let Ok(Json(entrada)) = cuerpo else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "JSON inválido" })),
        );
    };
    if entrada.texto.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "texto vacío" })),
        );
    }
    let respuesta =
        json!({ "texto": entrada.texto.to_uppercase(), "longitud": entrada.texto.chars().count() });
    (StatusCode::OK, Json(respuesta))
}

#[tokio::test]
async fn reto_4_eco() {
    let app = app();
    assert_eq!(
        enviar(&app, "POST", "/eco", Some(json!({"texto": "año"}))).await,
        (StatusCode::OK, json!({"texto": "AÑO", "longitud": 3}))
    );
    assert_eq!(
        enviar(&app, "POST", "/eco", Some(json!({"texto": "   "}))).await,
        (StatusCode::BAD_REQUEST, json!({"error": "texto vacío"}))
    );
    assert_eq!(
        enviar(&app, "POST", "/eco", Some(json!({"otro": 1}))).await,
        (StatusCode::BAD_REQUEST, json!({"error": "JSON inválido"}))
    );
}

// -----------------------------------------------------------------------------
/// RETO 5 — Un adaptador nuevo para el proyecto del módulo 17
/// Implementa el trait `Repositorio` de `tareas_api` con un DECORADOR: envuelve
/// otro repositorio, delega en él cada operación y cuenta cuántas operaciones
/// se han hecho (acaben bien o mal). Después se lo pasamos a la API real.
struct RepositorioContador {
    interno: Arc<dyn Repositorio>,
    operaciones: AtomicUsize,
}

impl RepositorioContador {
    fn new(interno: Arc<dyn Repositorio>) -> Self {
        Self {
            interno,
            operaciones: AtomicUsize::new(0),
        }
    }
    fn operaciones(&self) -> usize {
        self.operaciones.load(Ordering::SeqCst)
    }
    fn contar(&self) {
        self.operaciones.fetch_add(1, Ordering::SeqCst);
    }
}

impl Repositorio for RepositorioContador {
    fn listar(&self) -> Result<Vec<Tarea>, ErrorDominio> {
        self.contar();
        self.interno.listar()
    }
    fn obtener(&self, id: i64) -> Result<Tarea, ErrorDominio> {
        self.contar();
        self.interno.obtener(id)
    }
    fn crear(&self, titulo: &str) -> Result<Tarea, ErrorDominio> {
        self.contar();
        self.interno.crear(titulo)
    }
    fn actualizar(&self, id: i64, completada: bool) -> Result<Tarea, ErrorDominio> {
        self.contar();
        self.interno.actualizar(id, completada)
    }
    fn eliminar(&self, id: i64) -> Result<(), ErrorDominio> {
        self.contar();
        self.interno.eliminar(id)
    }
}

#[tokio::test]
async fn reto_5_repositorio_decorador() {
    let repo = Arc::new(RepositorioContador::new(
        Arc::new(RepositorioMemoria::new()),
    ));
    let api = tareas_api::app(repo.clone()); // Arc<RepositorioContador> → Arc<dyn Repositorio>
    assert_eq!(
        enviar(&api, "POST", "/tareas", Some(json!({"titulo": "decorar"})))
            .await
            .0,
        StatusCode::CREATED
    );
    assert_eq!(
        enviar(
            &api,
            "PATCH",
            "/tareas/1",
            Some(json!({"completada": true}))
        )
        .await
        .1["completada"],
        true
    );
    assert_eq!(
        enviar(&api, "GET", "/tareas/99", None).await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        enviar(&api, "DELETE", "/tareas/1", None).await.0,
        StatusCode::NO_CONTENT
    );
    assert_eq!(enviar(&api, "GET", "/tareas", None).await.1, json!([]));
    assert_eq!(enviar(&api, "GET", "/salud", None).await.0, StatusCode::OK); // no toca el repositorio
    assert_eq!(repo.operaciones(), 5);
}
