use axum::{Router, response::Redirect, routing::get};

pub fn redirects() -> Router {
    Router::new()
        .route(
            "/discord",
            get(Redirect::temporary("https://discord.gg/NBXq95C")),
        )
        .route(
            "/github",
            get(Redirect::temporary("https://github.com/gractwo")),
        )
        .route(
            "/youtube",
            get(Redirect::temporary("https://www.youtube.com/@gractwopl")),
        )
        .route(
            "/bsky",
            get(Redirect::temporary("https://bsky.app/profile/gractwo.pl")),
        )
}
