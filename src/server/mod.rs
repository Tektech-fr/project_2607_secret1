mod routes;

use axum::Router;

pub fn router() -> Router {
    Router::new().merge(routes::router())
}
