use askama::Template;
use axum::{Router, response::Html, routing::get};

use crate::views::layout::LayoutTemplate;

pub fn router() -> Router {
    Router::new().route("/", get(home))
}

async fn home() -> Html<String> {
    let page = LayoutTemplate {
        title: "Projet Rust",
        rows: 12,
        cols: 12,
    };

    Html(page.render().expect("template Askama invalide"))
}
