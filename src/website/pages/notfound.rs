use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::website::pages::INTERNAL_SERVER_ERROR_MSG;

#[derive(Template)]
#[template(path = "notfound.html")]
struct PageNotFound;

pub async fn page_notfound() -> Response {
    let a = PageNotFound;
    match a.render() {
        Ok(res) => (StatusCode::OK, Html(res)).into_response(),
        Err(_e) => (StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR_MSG).into_response(),
    }
}
