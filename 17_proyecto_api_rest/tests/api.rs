// =============================================================================
// TESTS DE LA API, ejecutados contra AMBOS adaptadores.
//   cargo test -p tareas_api
// =============================================================================
//
// No hace falta arrancar un servidor ni abrir puertos: un Router de axum es un
// `tower::Service`, y `oneshot` le pasa una petición directamente en memoria
// (como httptest en Go).

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::sync::Arc;
use tareas_api::{Repositorio, RepositorioMemoria, RepositorioSqlite, app};
use tower::ServiceExt; // trae .oneshot()

/// Los adaptadores a probar. Cada test se ejecuta con todos ellos.
fn adaptadores() -> Vec<(&'static str, Arc<dyn Repositorio>)> {
    vec![
        ("memoria", Arc::new(RepositorioMemoria::new())),
        ("sqlite", Arc::new(RepositorioSqlite::en_memoria().unwrap())),
    ]
}

/// Envía una petición y devuelve (estado, cabecera Location, cuerpo JSON).
async fn enviar(
    app: &Router,
    metodo: &str,
    ruta: &str,
    cuerpo: Option<Value>,
) -> (StatusCode, Option<String>, Value) {
    let mut peticion = Request::builder().method(metodo).uri(ruta);
    let body = match cuerpo {
        Some(json) => {
            peticion = peticion.header(header::CONTENT_TYPE, "application/json");
            Body::from(json.to_string())
        }
        None => Body::empty(),
    };
    // .clone() de un Router es barato (Arc por dentro); oneshot lo consume
    let respuesta = app
        .clone()
        .oneshot(peticion.body(body).unwrap())
        .await
        .unwrap();

    let estado = respuesta.status();
    let location = respuesta
        .headers()
        .get(header::LOCATION)
        .map(|v| v.to_str().unwrap().to_string());
    let bytes = respuesta.into_body().collect().await.unwrap().to_bytes();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (estado, location, json)
}

#[tokio::test]
async fn ciclo_de_vida_completo() {
    for (nombre, repo) in adaptadores() {
        let app = app(repo);

        let (estado, location, tarea) = enviar(
            &app,
            "POST",
            "/tareas",
            Some(json!({"titulo": "  Aprender Rust "})),
        )
        .await;
        assert_eq!(estado, StatusCode::CREATED, "[{nombre}]");
        assert_eq!(location.as_deref(), Some("/tareas/1"), "[{nombre}]");
        assert_eq!(
            tarea["titulo"], "Aprender Rust",
            "[{nombre}] el título se normaliza"
        );
        assert_eq!(tarea["completada"], false, "[{nombre}]");

        let (estado, _, tarea) = enviar(
            &app,
            "PATCH",
            "/tareas/1",
            Some(json!({"completada": true})),
        )
        .await;
        assert_eq!(estado, StatusCode::OK, "[{nombre}]");
        assert_eq!(tarea["completada"], true, "[{nombre}]");

        let (estado, _, tarea) = enviar(&app, "GET", "/tareas/1", None).await;
        assert_eq!(
            (estado, tarea["completada"].clone()),
            (StatusCode::OK, json!(true)),
            "[{nombre}]"
        );

        enviar(&app, "POST", "/tareas", Some(json!({"titulo": "Segunda"}))).await;
        let (_, _, lista) = enviar(&app, "GET", "/tareas", None).await;
        let ids: Vec<i64> = lista
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["id"].as_i64().unwrap())
            .collect();
        assert_eq!(ids, [1, 2], "[{nombre}] lista ordenada por id");

        let (estado, _, cuerpo) = enviar(&app, "DELETE", "/tareas/1", None).await;
        assert_eq!(
            (estado, cuerpo),
            (StatusCode::NO_CONTENT, Value::Null),
            "[{nombre}]"
        );

        let (estado, _, _) = enviar(&app, "GET", "/tareas/1", None).await;
        assert_eq!(estado, StatusCode::NOT_FOUND, "[{nombre}]");
    }
}

#[tokio::test]
async fn peticiones_invalidas() {
    let largo = "x".repeat(101);
    // (método, ruta, cuerpo, estado esperado, fragmento del mensaje de error)
    let casos: Vec<(&str, &str, Option<Value>, StatusCode, &str)> = vec![
        (
            "POST",
            "/tareas",
            Some(json!({"titulo": ""})),
            StatusCode::BAD_REQUEST,
            "obligatorio",
        ),
        (
            "POST",
            "/tareas",
            Some(json!({"titulo": largo})),
            StatusCode::BAD_REQUEST,
            "100",
        ),
        (
            "POST",
            "/tareas",
            Some(json!({"titlo": "typo"})),
            StatusCode::BAD_REQUEST,
            "unknown field",
        ),
        (
            "POST",
            "/tareas",
            None,
            StatusCode::BAD_REQUEST,
            "JSON inválido",
        ),
        (
            "PATCH",
            "/tareas/1",
            Some(json!({})),
            StatusCode::BAD_REQUEST,
            "completada",
        ),
        (
            "GET",
            "/tareas/abc",
            None,
            StatusCode::BAD_REQUEST,
            "id inválido",
        ),
        (
            "GET",
            "/tareas/0",
            None,
            StatusCode::BAD_REQUEST,
            "id inválido",
        ),
        (
            "GET",
            "/tareas/999",
            None,
            StatusCode::NOT_FOUND,
            "no encontrada",
        ),
        (
            "DELETE",
            "/tareas/999",
            None,
            StatusCode::NOT_FOUND,
            "no encontrada",
        ),
        ("GET", "/no/existe", None, StatusCode::NOT_FOUND, "ruta"),
    ];
    for (nombre, repo) in adaptadores() {
        let app = app(repo);
        for (metodo, ruta, cuerpo, esperado, fragmento) in &casos {
            let (estado, _, json) = enviar(&app, metodo, ruta, cuerpo.clone()).await;
            assert_eq!(estado, *esperado, "[{nombre}] {metodo} {ruta}");
            let mensaje = json["error"].as_str().unwrap_or_default();
            assert!(
                mensaje.contains(fragmento),
                "[{nombre}] {metodo} {ruta}: {mensaje:?} no contiene {fragmento:?}"
            );
        }
    }
}

#[tokio::test]
async fn metodo_no_permitido() {
    let app = app(Arc::new(RepositorioMemoria::new()));
    let (estado, _, _) = enviar(&app, "PUT", "/tareas", None).await;
    assert_eq!(estado, StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn un_fallo_de_la_bd_da_500_sin_filtrar_detalles() {
    let repo = Arc::new(RepositorioSqlite::en_memoria().unwrap());
    repo.romper_para_tests();
    let app = app(repo);
    let (estado, _, json) = enviar(&app, "GET", "/tareas", None).await;
    assert_eq!(estado, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        json,
        json!({"error": "error interno"}),
        "no debe filtrar el error de SQLite"
    );
}

#[tokio::test]
async fn peticiones_concurrentes() {
    for (nombre, repo) in adaptadores() {
        let app = app(repo);
        let mut tareas = tokio::task::JoinSet::new();
        for i in 0..50 {
            let app = app.clone();
            tareas.spawn(async move {
                enviar(
                    &app,
                    "POST",
                    "/tareas",
                    Some(json!({"titulo": format!("t{i}")})),
                )
                .await
                .0
            });
        }
        let estados = tareas.join_all().await;
        assert!(
            estados.iter().all(|e| *e == StatusCode::CREATED),
            "[{nombre}]"
        );
        let (_, _, lista) = enviar(&app, "GET", "/tareas", None).await;
        assert_eq!(
            lista.as_array().unwrap().len(),
            50,
            "[{nombre}] ninguna escritura perdida"
        );
    }
}
