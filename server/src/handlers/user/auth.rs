use crate::models::user::auth::{LogInReq, UserLogged};
use crate::models::user::AutoLoggedUser;
use axum::Extension;
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::PgPool;

use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};

use crate::utils::authentication::{create_session, generate_session_token};

pub async fn log_in(
    cookies: Cookies,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<LogInReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - log_in\n", "HANDLER");

    let row = sqlx::query_as!(
        UserLogged,
        "SELECT id FROM app_user WHERE username = $1",
        credentials.username
    )
    .fetch_optional(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    match row {
        Some(user) => {
            let token = generate_session_token();

            let result = create_session(&token, user.id, &pg_pool).await;

            match result {
                Ok(expires_at) => {
                    let mut cookie = Cookie::new("session", token);

                    let expires_at = OffsetDateTime::parse(&expires_at.to_rfc3339(), &Rfc3339)
                        .expect("Failed to convert chrono DateTime<Utc> to OffsetDateTime");

                    cookie.set_expires(expires_at);
                    cookie.set_path("/");
                    cookie.set_http_only(true);
                    // cookie.set_secure(true);
                    cookie.set_same_site(SameSite::Lax);

                    cookies.add(cookie);

                    Ok((StatusCode::OK, json!({ "success": true }).to_string()))
                }
                Err(message) => Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({ "success": false, "message": message }).to_string(),
                )),
            }
        }
        None => Ok((
            StatusCode::NOT_FOUND,
            json!({ "success": false, "message": "No user for this username or password" })
                .to_string(),
        )),
    }
}

pub async fn auto_log_in(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - auto_log_in\n", "HANDLER");

    let user = sqlx::query_as!(
        AutoLoggedUser,
        "
        SELECT
            app_user.id,
            first_name,
            last_name,
            image_url,
            role.id AS role_id,
            role.label AS role_label,
            ARRAY_AGG(permission.label) AS permissions
        FROM app_user
        INNER JOIN role ON role.id = app_user.role_id
        LEFT JOIN role_permission ON role_permission.role_id = role.id
        LEFT JOIN permission ON role_permission.permission_id = permission.id
        WHERE app_user.id = $1
        GROUP BY app_user.id, first_name, last_name, image_url, role.id, role.label;
        ",
        user_id
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": { "autoLoggedUser": user } }).to_string(),
    ))
}

pub async fn log_out(
    cookies: Cookies,
    Extension(session_id): Extension<String>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - log_out\n", "HANDLER");

    sqlx::query!("DELETE FROM session WHERE id = $1", session_id)
        .execute(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

    let mut cookie = Cookie::new("session", "");
    let past_time = Utc::now() - Duration::days(1);

    let expires_at = OffsetDateTime::parse(&past_time.to_rfc3339(), &Rfc3339)
        .expect("Failed to convert chrono DateTime<Utc> to OffsetDateTime");

    cookie.set_expires(expires_at);
    // cookie.set_secure(true);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);

    cookies.add(cookie);

    Ok((StatusCode::OK, json!({ "success": true }).to_string()))
}
