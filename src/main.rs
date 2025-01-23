use axum::{Router, routing::get};
use std::net::SocketAddr;

mod routes;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let app = Router::new()
        .route("/health", get(routes::health::health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    log::info!("Running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
