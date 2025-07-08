use axum::{Router, http::StatusCode, routing::get};

pub fn init() -> Router {
    Router::new()
        .route("/", get(async || "root"))
        .route("/discord/member-count", get(get_member_count))
}

async fn get_member_count() -> (StatusCode, String) {
    match crate::discordbot::get_member_count().await {
        Some(count) => (StatusCode::OK, format!("{count}")),
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error occured - could not fetch discord member count."),
        ),
    }
}
