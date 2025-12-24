use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};

#[derive(Clone, Debug, Copy)]
pub struct UserId(pub i64);

/// FIXME: 개발용 Mock Auth Middleware: user_id = 1 주입
pub async fn mock_auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // 실제로는 헤더 검증 로직이 들어가야 함
    // Authorization: Bearer {token} -> Validate -> Extract UserID

    // 일단 인증 작업 이전이라 유저 1로 간주
    req.extensions_mut().insert(UserId(1));

    Ok(next.run(req).await)
}
