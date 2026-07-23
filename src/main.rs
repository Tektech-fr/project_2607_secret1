use axum::{Json, Router, routing::get};
use tower_http::services::{ServeDir, ServeFile};

mod game;
use game::GameState;

async fn state() -> Json<GameState> {
    Json(GameState::new(10))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/state", get(state))
        .fallback_service(
            ServeDir::new("static").not_found_service(ServeFile::new("static/index.html")),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
