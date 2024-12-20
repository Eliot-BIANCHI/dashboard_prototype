use axum::http::StatusCode;
use serde_json::json;

use sqlx::PgPool;

pub enum KanbanAction {
    InviteUser,
    DeleteUser,
    // KickUser,
    Create,
    UpdateOwn(i32, i32),
    DeleteOwn(i32, i32),
    MoveOwn(i32, i32),
    UpdateAll,
    DeleteAll,
    MoveAll,
    AssignSelf,
    UnassignSelf,
}

pub enum KanbanTable {
    Kanban,
    List,
    Task,
}

enum As {
    Owner(i32, i32),
}

pub async fn has_kanban_permission_to(
    actions: &[KanbanAction],
    table: KanbanTable,
    kanban_id: i32,
    user_id: i32,
    pg_pool: &PgPool,
) -> Result<(), (StatusCode, String)> {
    let has_access = sqlx::query_scalar!(
        "
            SELECT EXISTS (
                SELECT 1
                FROM kanban
                WHERE (kanban.id = $1 AND kanban.owner_id = $2)
                OR kanban.id IN (
                        SELECT shared_kanban.id
                        FROM shared_kanban
                        WHERE shared_kanban.id = $1 AND shared_kanban.user_id = $2
                )
            );
        ",
        kanban_id,
        user_id
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    if !has_access.unwrap_or(false) {
        return Err((
            StatusCode::UNAUTHORIZED,
            json!({ "success": false, "message": format!("User doesn't have the rights to access this section") })
                .to_string(),
        ));
    }

    for action in actions {
        let (action_name, action_as) = get_action_name_and_as(action);

        let table_name = get_table_name(&table);

        let permission = format!("{}:{}", table_name, action_name);

        let has_permission = sqlx::query_scalar!(
            "
                SELECT EXISTS (
                    SELECT 1
                    FROM user_kanban_role
                    INNER JOIN kanban_role_permission ON kanban_role_permission.kanban_role_id = user_kanban_role.kanban_role_id
                    INNER JOIN permission ON permission.id = kanban_role_permission.permission_id
                    WHERE user_kanban_role.user_id = $1 AND user_kanban_role.kanban_id = $2
                    AND permission.label = $3
                );
            ",
            user_id,
            kanban_id,
            permission
        )
        .fetch_one(pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

        if !has_permission.unwrap_or(false) {
            continue;
        };

        let Some(r#as) = action_as else { return Ok(()) };

        match r#as {
            As::Owner(owner_id, table_id) => {
                if is_owner_of(table_name, table_id, owner_id, pg_pool).await? {
                    return Ok(());
                }
            }
        }
    }

    Err((
        StatusCode::FORBIDDEN,
        json!({ "success": false, "message": format!("User doesn't have the rights to access this section") })
            .to_string(),
    ))
}

async fn is_owner_of(
    table_name: &str,
    table_id: i32,
    user_id: i32,
    pg_pool: &PgPool,
) -> Result<bool, (StatusCode, String)> {
    let query = format!("SELECT owner_id FROM {} WHERE id = $1", table_name);

    let row: Option<i32> = sqlx::query_scalar(&query)
        .bind(table_id)
        .fetch_optional(pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

    let Some(owner_id) = row else {
        return Err((
            StatusCode::NOT_FOUND,
            json!({ "success": false, "message": format!("No {} found for this ID", table_name) })
                .to_string(),
        ));
    };

    Ok(owner_id == user_id)
}

fn get_action_name_and_as(action: &KanbanAction) -> (&str, Option<As>) {
    match action {
        // KanbanAction::KickUser => ("kick-user", None),
        KanbanAction::InviteUser => ("invite-user", None),
        KanbanAction::DeleteUser => ("delete-user", None),
        KanbanAction::Create => ("create", None),
        KanbanAction::UpdateOwn(owner_id, table_id) => {
            ("update-own", Some(As::Owner(*owner_id, *table_id)))
        }
        KanbanAction::MoveOwn(owner_id, table_id) => {
            ("move-own", Some(As::Owner(*owner_id, *table_id)))
        }
        KanbanAction::DeleteOwn(owner_id, table_id) => {
            ("delete-own", Some(As::Owner(*owner_id, *table_id)))
        }
        KanbanAction::UpdateAll => ("update-all", None),
        KanbanAction::MoveAll => ("move-all", None),
        KanbanAction::DeleteAll => ("delete-all", None),
        KanbanAction::AssignSelf => ("assign-self", None),
        KanbanAction::UnassignSelf => ("unassign-self", None),
    }
}

fn get_table_name(table: &KanbanTable) -> &str {
    match table {
        KanbanTable::Kanban => "kanban",
        KanbanTable::List => "list",
        KanbanTable::Task => "task",
    }
}
