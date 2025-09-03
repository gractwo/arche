use std::convert::Infallible;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    response::{IntoResponse, Response},
};

use crate::website::pages::index::page_index;

mod pages;

const STYLES_CSS: &str = include_str!("../../web/styles.css");

pub async fn website_service(req: Request<Body>) -> Result<Response, Infallible> {
    let path = req.uri().path().trim_start_matches("/");

    Ok(match path {
        "" | "index" | "index.html" | "index.htm" => page_index().await,
        "styles.css" => ([(header::CONTENT_TYPE, "text/css")], STYLES_CSS).into_response(),
        _ => StatusCode::NOT_FOUND.into_response(),
    })
}
