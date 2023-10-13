use axum::{routing::MethodRouter, Router};

/// Helper method to create a new route with its path and handler method
pub fn create_route(path: &str, method_router: MethodRouter<()>) -> Router {
    return Router::new().route(path, method_router);
}
