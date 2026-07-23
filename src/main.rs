use tokio::net::TcpListener;

mod game;
mod server;
// mod view;

#[tokio::main]
async fn main() {
    let app = server::router();

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Impossible de lier l'écouteur TCP");

    println!("En écoute sur http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("Erreur serveur");
}
