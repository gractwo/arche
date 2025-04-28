use axum::{Router, routing::get};

pub fn init() -> Router {
    Router::new().route("/", get(async || "root"))
}
