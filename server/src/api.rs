use axum::http::{Method, StatusCode, Uri};
use axum::middleware;
use axum::{extract::WebSocketUpgrade, response::Response, routing::get};
use axum::{routing::get_service, Router};
use serde_json::json;
use sqlx::PgPool;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::protocol::Message;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
mod calendar;
mod invitation;
mod kanban;
mod role_permission;
mod user;

use calendar::schedule;
use invitation::kanban as kanbanInvitation;
use kanban::{list, task};
use role_permission::{permission, role};
use user::{account, auth};

use crate::middlewares::mw_require_auth;

pub fn create_routes(db_pool: PgPool) -> Router {
    let allowed_methods = [
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
    ];

    let allowed_origins = ["http://127.0.0.1:5173".parse().unwrap()];

    let allowed_headers = [
        "content-type".parse().unwrap(),
        "authorization".parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_methods(allowed_methods)
        .allow_origin(allowed_origins)
        .allow_headers(allowed_headers)
        .allow_credentials(true);

    let api_routes = Router::new()
        .route("/ws/kanban", get(ws_kanban))
        .nest("/invitations", invitation::routes())
        .nest("/invitations/kanbans", kanbanInvitation::routes())
        .nest("/users", user::routes())
        .nest("/calendars", calendar::routes())
        .nest("/calendars/:calendar_id/schedules", schedule::routes())
        .nest("/kanbans", kanban::routes())
        .nest("/kanbans/:kanban_id/lists", list::routes())
        .nest("/kanbans/:kanban_id/lists/:list_id/tasks", task::routes())
        .nest("/account", account::routes())
        .nest("/auth", auth::routes())
        .nest("/roles-permissions", role_permission::routes())
        .nest("/roles", role::routes())
        .nest("/permissions", permission::routes())
        .route_layer(middleware::from_fn_with_state(
            db_pool.clone(),
            mw_require_auth,
        ));

    Router::new()
        .nest("/api", api_routes)
        .nest_service("/assets", get_service(ServeDir::new("./assets")))
        .with_state(db_pool)
        .layer(CookieManagerLayer::new())
        .fallback(not_found_route)
        .layer(cors)
}

async fn not_found_route(uri: Uri) -> (StatusCode, String) {
    println!("->> {:<12} - not_found_route\n", "HANDLER");

    (
        StatusCode::NOT_FOUND,
        json!({ "success": false, "message": format!("No route for {uri}") }).to_string(),
    )
}

pub async fn ws_kanban(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_kanban_socket)
}

async fn handle_kanban_socket(mut socket: axum::extract::ws::WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            axum::extract::ws::Message::Text(text) => {
                println!("Received from client: {}", text);

                // Parse the message and respond
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                    if json["action"] == "update" {
                        println!("Update action received with payload: {:?}", json["payload"]);

                        // Respond to the client
                        if let Err(err) = socket
                            .send(axum::extract::ws::Message::Text(
                                "Calendar updated successfully".to_string(),
                            ))
                            .await
                        {
                            eprintln!("Failed to send message: {}", err);
                        }
                    }
                }
            }
            axum::extract::ws::Message::Close(_) => {
                println!("Client disconnected");
                break;
            }
            _ => {}
        }
    }
}
