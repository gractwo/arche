use std::convert::Infallible;

use axum::{
    body::Body,
    http::{Request, header},
    response::{IntoResponse, Response},
};

use crate::website::pages::{index::page_index, notfound::page_notfound};

mod pages;

const STYLES_CSS: &str = include_str!("../../web/styles.css");
const FAVICON_SVG: &str = include_str!("../../assets/logo.svg");
const FAVICON_X64: &[u8] = include_bytes!("../../assets/logo-x64.png");

pub async fn website_service(req: Request<Body>) -> Result<Response, Infallible> {
    let path = req.uri().path().trim_start_matches("/");

    Ok(match path {
        "" | "index" | "index.html" | "index.htm" => page_index().await,
        "styles.css" => ([(header::CONTENT_TYPE, "text/css")], STYLES_CSS).into_response(),
        "favicon.svg" => ([(header::CONTENT_TYPE, "image/svg+xml")], FAVICON_SVG).into_response(),
        "favicon-x64.png" => ([(header::CONTENT_TYPE, "image/png")], FAVICON_X64).into_response(),
        _ => page_notfound().await,
    })
}
