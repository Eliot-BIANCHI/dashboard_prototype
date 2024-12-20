use crate::models::user::{
    CreateUserReq, PaginatedUsersOverviewsQuery, UpdateUserReq, User, UserOverview,
    UsersOverviewsQuery,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use sqlx::PgPool;

pub mod account;
pub mod auth;

pub async fn get_users_overviews(
    State(pg_pool): State<PgPool>,
    Query(params): Query<UsersOverviewsQuery>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_users_overviews\n", "HANDLER");

    if !["username", "first_name", "last_name"].contains(&params.by.as_str()) {
        return Ok((
            StatusCode::BAD_REQUEST,
            json!({ "success": false, "message": "Invalid 'by' parameter" }).to_string(),
        ));
    }

    if params.search.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            json!({ "success": false, "message": "'search' query cannot be empty" }).to_string(),
        ));
    }

    let query = format!(
        "SELECT id, first_name, last_name FROM app_user WHERE {} ILIKE $1 LIMIT 10;",
        params.by
    );

    let search_pattern = format!("{}%", params.search);

    let overviews = sqlx::query_as::<_, UserOverview>(&query)
        .bind(search_pattern)
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": { "overviews": overviews } }).to_string(),
    ))
}

const LIMIT: i64 = 2;

pub async fn get_paginated_users_overviews(
    State(pg_pool): State<PgPool>,
    Query(params): Query<PaginatedUsersOverviewsQuery>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_paginated_users_overviews\n", "HANDLER");

    let page_number = match params.page {
        Some(page_number) => page_number,
        None => 1,
    };

    let offset = if page_number == 1 {
        0
    } else {
        (page_number - 1) * LIMIT
    };

    let count;

    let overviews;

    if params.by == "newest" || params.by == "oldest" {
        let order = if params.by == "newest" { "ASC" } else { "DESC" };

        let query = format!(
            "
                SELECT id, first_name, last_name
                FROM app_user
                ORDER BY created_at {}
                LIMIT 2 OFFSET $1;
            ",
            order
        );

        overviews = sqlx::query_as::<_, UserOverview>(&query)
            .bind(offset)
            .fetch_all(&pg_pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({ "success": false, "message": e.to_string() }).to_string(),
                )
            })?;

        if params.page.is_none() {
            count = sqlx::query_scalar!("SELECT COUNT(id) FROM app_user;")
                .fetch_one(&pg_pool)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({ "success": false, "message": e.to_string() }).to_string(),
                    )
                })?;
        } else {
            count = Some(-1);
        }
    } else if params.by == "role" {
        overviews = sqlx::query_as!(
            UserOverview,
            "
                SELECT id, first_name, last_name
                FROM app_user
                WHERE role_id = $1
                LIMIT 2 OFFSET $2;
            ",
            params.role_id,
            offset
        )
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

        if params.page.is_none() {
            count = sqlx::query_scalar!(
                "SELECT COUNT(id) FROM app_user WHERE role_id = $1;",
                params.role_id
            )
            .fetch_one(&pg_pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({ "success": false, "message": e.to_string() }).to_string(),
                )
            })?;
        } else {
            count = Some(-1);
        }
    } else {
        let search_pattern = format!("{}%", params.search.unwrap());

        overviews = sqlx::query_as!(
            UserOverview,
            "
                SELECT id, first_name, last_name
                FROM app_user
                WHERE first_name ILIKE $1 OR last_name ILIKE $1
                LIMIT 2 OFFSET $2;
            ",
            search_pattern,
            offset
        )
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

        if params.page.is_none() {
            count = sqlx::query_scalar!(
                "SELECT COUNT(id) FROM app_user WHERE first_name ILIKE $1 OR last_name ILIKE $1;",
                search_pattern
            )
            .fetch_one(&pg_pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({ "success": false, "message": e.to_string() }).to_string(),
                )
            })?;
        } else {
            count = Some(-1);
        }
    }

    Ok((
        StatusCode::OK,
        json!({
            "success": true,
            "data": {
                "overviews": overviews,
                "count": count,
                "pageNumber": page_number
            }
        })
        .to_string(),
    ))
}

pub async fn get_user(
    Path(user_id): Path<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_user\n", "HANDLER");

    let user = sqlx::query_as!(
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
        json!({ "success": true, "data": { "user": user } }).to_string(),
    ))
}

pub async fn create_user(
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<CreateUserReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - create_user\n", "HANDLER");

    let user_id = sqlx::query_scalar!(
        "INSERT INTO app_user (first_name, last_name) VALUES ($1, $2) RETURNING id",
        credentials.first_name,
        credentials.last_name
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
        json!({ "success": true, "data": { "userId": user_id } }).to_string(),
    ))
}

pub async fn update_user(
    Path(user_id): Path<i32>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<UpdateUserReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - update_user\n", "HANDLER");

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

pub async fn delete_user(
    Path(user_id): Path<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - delete_user\n", "HANDLER");

    sqlx::query!("DELETE FROM app_user WHERE id = $1", user_id)
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
