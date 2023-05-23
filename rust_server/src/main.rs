use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

mod handlers;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/api/item", get(handlers::item_handler))
        .route("/api/delivery", get(handlers::delivery_handler))
        .layer(cors);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server should not fail.");
}
