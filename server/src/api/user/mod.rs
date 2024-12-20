use crate::handlers::user::{
    create_user, delete_user, get_paginated_users_overviews, get_user, get_users_overviews,
    update_user,
};
use axum::{routing::get, Router};
use sqlx::PgPool;

pub mod account;
pub mod auth;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_users_overviews).post(create_user))
        .route("/paginated", get(get_paginated_users_overviews))
        .route(
            "/:user_id",
            get(get_user).put(update_user).delete(delete_user),
        )
}
