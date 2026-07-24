mod routes;
mod views;

use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    let app = routes::api::router().nest_service("/static", ServeDir::new("static"));

    axum::serve(listener, app).await?;

    Ok(())
}
