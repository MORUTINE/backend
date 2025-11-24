use crate::bootstrap::config::AppConfig;
use tracing_subscriber::EnvFilter;

pub fn init_logger(config: &AppConfig) {
    let level = config.logging.level.to_string();

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    tracing_subscriber::fmt()
        .compact() // 로그 형태 짧게
        .with_target(false) // 모듈 경로 포함 안함
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_env_filter(filter)
        .init();
}
