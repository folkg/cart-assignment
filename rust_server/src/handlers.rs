use std::fs;

use axum::Json;
use serde_json::Value;

pub async fn item_handler() -> Json<Value> {
    let data = fs::read_to_string("json/line_items.json").expect("Should be able to read file.");
    let json = serde_json::from_str(&data).expect("Should have the correct JSON format.");
    Json(json)
}

pub async fn delivery_handler() -> Json<Value> {
    let data =
        fs::read_to_string("json/delivery_dates.json").expect("Should be able to read file.");
    let json = serde_json::from_str(&data).expect("Should have the correct JSON format.");
    Json(json)
}
