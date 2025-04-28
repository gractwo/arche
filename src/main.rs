use std::error::Error;
use tokio::net::TcpListener;

mod discordbot;
mod router;
mod setup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup::dotenv_and_tracing();

    let r = router::init();
    let l = TcpListener::bind("0.0.0.0:2020").await?;

    discordbot::init().await;
    axum::serve(l, r).await.unwrap();
    Ok(())
}
