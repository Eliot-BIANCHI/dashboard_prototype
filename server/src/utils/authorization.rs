use axum::http::StatusCode;
use serde_json::json;

use sqlx::PgPool;

pub enum Action {
    ViewOwn(i32, i32),
    Create,
    UpdateOwn(i32, i32),
    DeleteOwn(i32, i32),
    ViewInvitedTo(i32, i32),
    ViewAll,
    UpdateAll,
    DeleteAll,
}

enum As {
    Owner(i32, i32),
    Invitee(i32, i32),
}

pub enum Table {
    Calendar,
    Kanban,
    Role,
    Permission,
    RolePermission,
}

pub async fn has_permission_to(
    actions: &[Action],
    table: Table,
    user_id: i32,
    pg_pool: &PgPool,
) -> Result<(), (StatusCode, String)> {
    for action in actions {
        let (action_name, action_as) = get_action_name_and_as(action);

        let table_name = get_table_name(&table);

        let permission = format!("{}:{}", table_name, action_name);

        let has_permission = sqlx::query_scalar!(
            "
                SELECT EXISTS (
                    SELECT 1
                    FROM app_user
                    INNER JOIN role ON app_user.role_id = role.id
                    INNER JOIN role_permission ON role_permission.role_id = role.id
                    INNER JOIN permission ON permission.id = role_permission.permission_id
                    WHERE app_user.id = $1
                    AND permission.label = $2
                );
            ",
            user_id,
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
            As::Invitee(invitee_id, table_id) => {
                if is_invited_to(table_name, table_id, invitee_id, pg_pool).await? {
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

async fn is_invited_to(
    table_name: &str,
    table_id: i32,
    user_id: i32,
    pg_pool: &PgPool,
) -> Result<bool, (StatusCode, String)> {
    let shared_table_name = format!("shared_{}", table_name);

    let query = format!(
        "
            SELECT EXISTS (
                SELECT 1 FROM {} WHERE id = $1 AND user_id = $2
            );
        ",
        shared_table_name
    );

    let is_invited: Option<bool> = sqlx::query_scalar(&query)
        .bind(table_id)
        .bind(user_id)
        .fetch_optional(pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

    Ok(is_invited.unwrap_or(false))
}

fn get_action_name_and_as(action: &Action) -> (&str, Option<As>) {
    match action {
        Action::ViewOwn(owner_id, table_id) => ("view-own", Some(As::Owner(*owner_id, *table_id))),
        Action::Create => ("create", None),
        Action::UpdateOwn(owner_id, table_id) => {
            ("update-own", Some(As::Owner(*owner_id, *table_id)))
        }
        Action::DeleteOwn(owner_id, table_id) => {
            ("delete-own", Some(As::Owner(*owner_id, *table_id)))
        }
        Action::ViewInvitedTo(invitee_id, table_id) => {
            ("view-invited-to", Some(As::Invitee(*invitee_id, *table_id)))
        }
        Action::ViewAll => ("view-all", None),
        Action::UpdateAll => ("update-all", None),
        Action::DeleteAll => ("delete-all", None),
    }
}

fn get_table_name(table: &Table) -> &str {
    match table {
        Table::Calendar => "calendar",
        Table::Kanban => "kanban",
        Table::Role => "role",
        Table::Permission => "permission",
        Table::RolePermission => "role-permission",
    }
}
