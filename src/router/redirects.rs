use axum::{Router, response::Redirect, routing::get};

#[rustfmt::skip]
const REDIRECTS: &[(&[&str], &str)] = &[
    (
        &["/discord", "/dsc", "/dc"],
        "https://discord.gg/NBXq95C"
    ),(
        &["/github", "/gh"],
        "https://github.com/gractwo"
    ),(
        &["/youtube", "/yt"],
        "https://www.youtube.com/@gractwopl"
    ),(
        &["/bsky", "/bluesky"],
        "https://bsky.app/profile/gractwo.pl",
    ),
];

macro_rules! build_redirects {
    ($redirects:expr) => {{
        let mut router = Router::new();
        for (paths, url) in $redirects {
            for path in *paths {
                router = router.route(path, get(Redirect::temporary(*url)));
            }
        }
        router
    }};
}

pub fn redirects() -> Router {
    build_redirects!(REDIRECTS)
}
