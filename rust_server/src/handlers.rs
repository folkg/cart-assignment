use axum::Json;

pub async fn item_handler() -> Json<&'static str> {
    Json("Item works!")
}

pub async fn delivery_handler() -> Json<&'static str> {
    Json("Delivery works!")
}
