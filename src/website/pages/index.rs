use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::{
    SHRUG, days::days_of_community_existence, discordbot::get_member_count,
    website::pages::INTERNAL_SERVER_ERROR_MSG,
};

#[derive(Template)]
#[template(path = "index.html")]
struct PageIndex {
    /// approximate discord server member count
    dsc_members: String,
    days_community: i64,
}

pub async fn page_index() -> Response {
    let a = PageIndex {
        dsc_members: match get_member_count().await {
            Some(count) => match count {
                ..=512 => format!("{count}"),
                _ => format!("~{count}"),
            },
            None => SHRUG.into(),
        },
        days_community: days_of_community_existence(),
    };
    match a.render() {
        Ok(res) => (StatusCode::OK, Html(res)).into_response(),
        Err(_e) => (StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR_MSG).into_response(),
    }
}
