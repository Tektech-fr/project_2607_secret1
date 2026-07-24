use askama::Template;
use axum::{Router, response::Html, routing::get};

use crate::views::layout::LayoutTemplate;

pub fn router() -> Router {
    Router::new().route("/", get(home))
}

async fn home() -> Html<String> {
    let images = [
        "url('/static/assets/goat1.webp')",
        "url('/static/assets/goat2.avif')",
        "url('/static/assets/goat3.avif')",
        "url('/static/assets/goat4.jpg')",
    ];

    let page = LayoutTemplate {
        title: "Goat X",
        rows: 12,
        cols: 12,
        background_image: images[rand::random_range(0..images.len())].to_string(),
    };

    Html(page.render().expect("template Askama invalide"))
}
