use axum::{Router, response::Html, routing::get};

pub fn router() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <title>Project 2607</title>
</head>
<body>
    <h1>Project 2607</h1>
    <p>Le serveur Rust fonctionne.</p>
</body>
</html>"#,
    )
}
