use dotenvy::Error;
use tracing::{Level, error, info, subscriber, warn};
use tracing_subscriber::FmtSubscriber;

pub fn dotenv_and_tracing() {
    let dotenv = match dotenvy::dotenv() {
        Ok(_) => DotenvStatus::LoadedFine,
        Err(e) => match e.not_found() {
            true => DotenvStatus::NotFound,
            false => DotenvStatus::Error(e),
        },
    };
    let lvl = match std::env::var("RUST_LOG") {
        Ok(s) => Level::try_from_str(&s),
        Err(e) => match e {
            std::env::VarError::NotPresent => Level::INFO,
            std::env::VarError::NotUnicode(s) => panic!("{:?} is not a valid log level.", s),
        },
    };
    let s = FmtSubscriber::builder().with_max_level(lvl).finish();
    subscriber::set_global_default(s).expect(SETGLOBAL_FAIL);
    info!("tracing initialised ({}).", lvl);
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

trait StrToLevel {
    fn try_from_str(value: &str) -> Level;
}
impl StrToLevel for Level {
    fn try_from_str(value: &str) -> Level {
        match value.to_lowercase().as_str() {
            "error" => Level::ERROR,
            "warn" => Level::WARN,
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => panic!("{value} is not a valid log level."),
        }
    }
}
