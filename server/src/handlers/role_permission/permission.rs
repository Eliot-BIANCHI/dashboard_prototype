use crate::models::role_permission::permission::CreatePermissionReq;
use crate::utils::authorization::{has_permission_to, Action, Table};
use axum::{extract::State, http::StatusCode, Extension, Json};
use serde_json::json;
use sqlx::PgPool;

pub async fn create_permission(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<CreatePermissionReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - create_permission\n", "HANDLER");

    has_permission_to(&[Action::Create], Table::Permission, user_id, &pg_pool).await?;

    let permission_id = sqlx::query_scalar!(
        "INSERT INTO permission (label) VALUES ($1) RETURNING id",
        credentials.label,
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
        json!({ "success": true, "data": { "permissionId": permission_id } }).to_string(),
    ))
}
