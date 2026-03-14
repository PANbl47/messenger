pub mod router;

use axum::{body::Body, http::Request, response::Response};
use tower::ServiceExt;

pub fn app() -> axum::Router {
    router::build_router()
}

pub mod test_support {
    use super::*;

    pub async fn health() -> Response {
        app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("health request should be valid"),
            )
            .await
            .expect("health route should respond")
    }
}
