use axum::{routing::get, Router};

pub async fn build_api_router() -> Router {
    Router::new().route("/hello", get(hello_from_the_server))
}

async fn hello_from_the_server() -> &'static str {
    "Hello!"
}
