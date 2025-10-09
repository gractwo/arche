use axum::{Router, http::StatusCode, routing::get};
use tower::service_fn;

use crate::{
    days::days_of_community_existence, router::redirects::redirects, website::website_service,
};

mod redirects;

pub fn init() -> Router {
    Router::new()
        .merge(redirects())
        .nest("/api/", api())
        .fallback_service(service_fn(website_service))
}

fn api() -> Router {
    Router::new()
        .route("/", get(async || StatusCode::OK))
        .route("/live", get(async || StatusCode::OK))
        .route("/discord-member-count", get(get_member_count))
        .route("/days-community", get(get_days_since_community_formation))
        .fallback(api_fallback)
}

async fn api_fallback() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "Resource not found. Sorry!".into())
}

async fn get_member_count() -> (StatusCode, String) {
    match crate::discordbot::get_member_count().await {
        Some(count) => (StatusCode::OK, count.to_string()),
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error occured - could not fetch discord member count."),
        ),
    }
}

async fn get_days_since_community_formation() -> String {
    days_of_community_existence().to_string()
}
