use crate::models::user::account::UpdateAccountReq;
use crate::models::user::User;
use axum::{extract::State, http::StatusCode, Extension, Json};
use serde_json::json;
use sqlx::PgPool;

pub async fn get_account(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_account\n", "HANDLER");

    let row = sqlx::query_as!(
        User,
        "SELECT id, first_name, last_name, image_url FROM app_user WHERE id = $1",
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
        json!({ "success": true, "data": { "account": row } }).to_string(),
    ))
}

pub async fn update_account(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<UpdateAccountReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - update_account\n", "HANDLER");

    sqlx::query!(
        "
        UPDATE app_user SET
            first_name = $2,
            last_name = $3
        WHERE id = $1
        ",
        user_id,
        credentials.first_name,
        credentials.last_name,
    )
    .execute(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((StatusCode::OK, json!({ "success": true }).to_string()))
}
