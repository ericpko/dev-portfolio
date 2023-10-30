use std::time::Duration;

use anyhow::Context;
use askama::Template;
use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, response::IntoResponse, routing::get,
    Router,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use dev_portfolio::HtmlTemplate;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "dev_portfolio=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");

    // We could also read our port in from the environment as well
    let dir_path = std::env::current_dir().unwrap();

    let api_router = Router::new().route("/hello", get(hello_from_the_server));
    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(index))
        .route("/about", get(about))
        // Add a tower service route to serve everything under the assets/ folder
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", dir_path.to_str().unwrap())),
        )
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );
    let port = 8000_u16;
    // let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

    info!("router initialized, now listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutPageTemplate;

async fn about() -> impl IntoResponse {
    let template = AboutPageTemplate {};
    HtmlTemplate(template)
}

async fn hello_from_the_server() -> &'static str {
    "Hello!"
}
