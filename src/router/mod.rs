use axum::{Router, http::StatusCode, routing::get};
use chrono::{NaiveDate, Utc};

pub fn init() -> Router {
    Router::new()
        .route("/", get(async || "root"))
        .route("/discord/member-count", get(get_member_count))
        .route("/days/community", get(get_days_since_community_formation))
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

async fn get_days_since_community_formation() -> String {
    let formation = NaiveDate::from_ymd_opt(2020, 6, 7).unwrap();
    let today = Utc::now().date_naive();
    let days = today.signed_duration_since(formation).num_days();

    days.to_string()
}
