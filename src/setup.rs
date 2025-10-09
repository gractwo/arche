use dotenvy::Error;
use tracing::{error, info, subscriber, warn};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn dotenv_and_tracing() {
    let dotenv = match dotenvy::dotenv() {
        Ok(_) => DotenvStatus::LoadedFine,
        Err(e) => match e.not_found() {
            true => DotenvStatus::NotFound,
            false => DotenvStatus::Error(e),
        },
    };
    let filter = match std::env::var("RUST_LOG") {
        Ok(s) => EnvFilter::new(s),
        Err(e) => match e {
            std::env::VarError::NotPresent => EnvFilter::new("info,serenity=warn"),
            std::env::VarError::NotUnicode(s) => panic!("{:?} is not a valid log level.", s),
        },
    };
    let s = FmtSubscriber::builder()
        .with_env_filter(filter.clone())
        .finish();
    subscriber::set_global_default(s).expect(SETGLOBAL_FAIL);
    info!("tracing initialised.");
    match dotenv {
        DotenvStatus::LoadedFine => info!(".env loaded successfully."),
        DotenvStatus::NotFound => warn!(".env not found; skipping..."),
        DotenvStatus::Error(e) => error!(".env error: {e}"),
    }
}

const SETGLOBAL_FAIL: &str = "tracing subscriber setup failed.";
enum DotenvStatus {
    LoadedFine,
    NotFound,
    Error(Error),
}
