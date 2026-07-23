use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route_service("/", ServeFile::new("static/index.html"))
    .nest_service("/css", ServeDir::new("static/css"))
    .nest_service("/js", ServeDir::new("static/js"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .expect("Impossible d'écouter sur le port 3000");

    println!("Serveur lancé sur http://127.0.0.1:3000");

    axum::serve(listener, app)
    .await
    .expect("Erreur du serveur");
}
