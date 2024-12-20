use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use sqlx::PgPool;
use tower_cookies::Cookies;

use crate::utils::authentication::validate_session;

pub async fn mw_require_auth(
    State(pg_pool): State<PgPool>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, String> {
    if req.uri().path() == "/auth/log-in" {
        return Ok(next.run(req).await);
    }
    println!("->> {:<12} - mw_require_auth\n", "MIDDLEWARE");

    let auth_token = cookies
        .get("session")
        .map(|cookie| cookie.value().to_string());

    if let Some(token) = auth_token {
        let result = validate_session(token, pg_pool).await;

        match result {
            Ok(session) => {
                if req.uri().path() == "/auth/log-out" {
                    req.extensions_mut().insert(session.id);
                    return Ok(next.run(req).await);
                } else {
                    req.extensions_mut().insert(session.user_id);
                    return Ok(next.run(req).await);
                }
            }
            Err(err) => {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from(err))
                    .unwrap();

                Ok(response)
            }
        }
    } else {
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized: No session found"))
            .unwrap();

        Ok(response)
    }
}
