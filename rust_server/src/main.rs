use axum::{routing::get, Router};

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/item", get(handlers::item_handler))
        .route("/api/delivery", get(handlers::delivery_handler));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 4001));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server should not fail.");
}
