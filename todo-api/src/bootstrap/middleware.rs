use axum::body::Body;
use http::HeaderName;
use std::time::Duration;
use tower::{limit::ConcurrencyLimitLayer, timeout::TimeoutLayer};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::{
    classify::ServerErrorsFailureClass, compression::CompressionLayer, trace::TraceLayer,
};
use tracing::Level;

pub fn build_timeout_layer() -> TimeoutLayer {
    TimeoutLayer::new(Duration::from_secs(10))
}

pub fn build_concurrency_limit_layer() -> ConcurrencyLimitLayer {
    ConcurrencyLimitLayer::new(100)
}

pub fn build_request_id_layer() -> impl tower::Layer<http::Request<Body>> + Clone {
    let header = HeaderName::from_static("x-request-id");

    tower::layer::util::Stack::new(
        PropagateRequestIdLayer::new(header.clone()),
        SetRequestIdLayer::new(header, MakeRequestUuid),
    )
}

pub fn build_compression_layer() -> CompressionLayer {
    CompressionLayer::new()
}

pub fn build_trace_layer() -> impl tower::Layer<http::Request<Body>> + Clone {
    TraceLayer::new_for_http()
        .make_span_with(|req: &http::Request<Body>| {
            let method = req.method().as_str();
            let uri = req.uri().path();

            tracing::span!(Level::DEBUG, "http_request", method, uri,)
        })
        .on_failure(
            |err: ServerErrorsFailureClass, latency: Duration, span: &tracing::Span| {
                tracing::error!(
                    parent: span,
                    "요청 실패: {:?}, latency: {:?}",
                    err,
                    latency
                );
            },
        )
}
