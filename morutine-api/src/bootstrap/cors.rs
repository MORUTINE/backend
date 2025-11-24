use crate::bootstrap::config::AppConfig;
use http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

pub fn build_cors(config: &AppConfig) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(
            HeaderValue::from_str(&config.cors.allow_origin)
                .expect("CORS ALLOW ORIGIN이 유효하지 않습니다."),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
}
