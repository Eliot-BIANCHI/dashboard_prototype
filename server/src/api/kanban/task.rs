use crate::handlers::kanban::task::{
    assign_user, create_task, delete_task, patch_task_completion, patch_task_list, unassign_user,
    update_task,
};
use axum::{
    routing::{patch, post, put},
    Router,
};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", post(create_task))
        .route("/:task_id", put(update_task).delete(delete_task))
        .route("/:task_id/completion", patch(patch_task_completion))
        .route("/:task_id/list", patch(patch_task_list))
        .route("/:task_id/assign-user", post(assign_user))
        .route("/:task_id/unassign-user", post(unassign_user))
}
