use std::fs;

use axum::{extract::Query, routing::post, Json};
use serde::Deserialize;
use serde_json::Value;

pub async fn item_handler() -> Json<Value> {
    let data = fs::read_to_string("json/line_items.json").expect("Should be able to read file.");
    let json = serde_json::from_str(&data).expect("Should have the correct JSON format.");
    Json(json)
}

#[derive(Deserialize)]
pub struct DeliveryQueryParams {
    postalCode: String,
    lineItemId: String,
}

pub async fn delivery_handler(Query(params): Query<DeliveryQueryParams>) -> Json<Value> {
    let postal_code = params.postalCode;
    let line_item_id = params.lineItemId;
    let data =
        fs::read_to_string("json/delivery_dates.json").expect("Should be able to read file.");
    let json = serde_json::from_str(&data).expect("Should have the correct JSON format.");
    println!("{} {}", postal_code, line_item_id);
    Json(json)
}
