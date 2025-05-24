use axum::{extract::Request, routing::get};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::Span;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let server_url = std::env::var("SERVER_URL").unwrap();

    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = axum::Router::new()
        .route(
            "/",
            get(|| async { "this project made by create-axum-app!" }),
        )
        .layer(
            ServiceBuilder::new().layer(TraceLayer::new_for_http().on_request(
                |req: &Request, _span: &Span| tracing::debug!("{} {}", req.method(), req.uri()),
            )),
        );

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    tracing::info!("listening on {}", &server_url);
    axum::serve(listener, app).await.unwrap();
}
