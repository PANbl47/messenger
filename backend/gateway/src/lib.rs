pub mod router;

use std::sync::OnceLock;

use axum::{
    body::{to_bytes, Body},
    http::Request,
};
use router::{build_router_with_state, AppState};
use serde_json::Value;
use tower::ServiceExt;

pub fn app() -> axum::Router {
    router::build_router()
}

pub mod test_support {
    use super::*;

    static TEST_APP: OnceLock<tokio::sync::Mutex<axum::Router>> = OnceLock::new();

    pub struct JsonResponse {
        pub status: u16,
        pub body: Value,
    }

    fn shared_app() -> &'static tokio::sync::Mutex<axum::Router> {
        TEST_APP.get_or_init(|| {
            tokio::sync::Mutex::new(build_router_with_state(AppState::new(
                persistence::InMemoryStore::default(),
            )))
        })
    }

    pub async fn health() -> axum::response::Response {
        shared_app()
            .lock()
            .await
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("health request should be valid"),
            )
            .await
            .expect("health route should respond")
    }

    pub async fn json(method: &str, uri: &str, body: Value) -> JsonResponse {
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(if body.is_null() {
                Body::empty()
            } else {
                Body::from(
                    serde_json::to_vec(&body).expect("json test body should serialize"),
                )
            })
            .expect("json request should be valid");

        let response = shared_app()
            .lock()
            .await
            .clone()
            .oneshot(request)
            .await
            .expect("app should return a response");

        let status = response.status().as_u16();
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");
        let body = if bytes.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&bytes).expect("response body should be valid json")
        };

        JsonResponse { status, body }
    }
}
