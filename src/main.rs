use axum::{
    response::Html,
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await.expect("Impossible d'écouter sur le port 3000");

    println!("Serveur lancé sur http://127.0.0.1:3000");

    axum::serve(listener, app)
    .await
    .expect("Erreur du serveur");
}

async fn index() -> Html<&'static str> {
    Html("<h1>Bienvenue chez riri,fifi et loulou</h1>")
}