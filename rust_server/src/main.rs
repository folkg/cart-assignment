use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    axum::Server::bind(&"127.0.0.1:4001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("server should not fail.")
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
