mod api;
mod handlers;
mod middlewares;
mod models;
mod utils;

use sqlx::postgres::PgPoolOptions;

use api::create_routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let server_address =
        std::env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not found in the .env file");
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL not found in the .env file");

    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Cannot connect to database");

    let app = create_routes(db_pool);

    let listener = tokio::net::TcpListener::bind(server_address)
        .await
        .expect("Could not create TCP listener");

    println!(
        "->> {:<12} - server listening on {}\n",
        "AXUM",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
