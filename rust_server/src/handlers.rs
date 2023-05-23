use std::fs;

use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub async fn item_handler() -> Json<Value> {
    // TODO: Better error handling to send back status 500 or something instead of panicing
    let data = fs::read_to_string("json/line_items.json").expect("Should be able to read file.");
    let items = serde_json::from_str(&data).expect("Should have the correct JSON format.");
    Json(items)
}

#[derive(Deserialize)]
pub struct DeliveryQueryParams {
    #[serde(rename = "postalCode")]
    postal_code: String,

    #[serde(rename = "lineItemId")]
    line_item_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeliveryDate {
    #[serde(rename = "postal")]
    postal_code: String,

    #[serde(rename = "ids")]
    item_ids: Vec<usize>,

    #[serde(rename = "estimatedDeliveryDate")]
    estimated_delivery_date: String,
}

pub async fn delivery_handler(params: Query<DeliveryQueryParams>) -> Json<Vec<DeliveryDate>> {
    let DeliveryQueryParams {
        postal_code,
        line_item_id,
        ..
    } = params.0;

    // TODO: Better error handling to send back status 500 or something instead of panicing
    let data =
        fs::read_to_string("json/delivery_dates.json").expect("Should be able to read file.");
    let delivery_dates: Vec<DeliveryDate> =
        serde_json::from_str(&data).expect("Should have the correct JSON format.");

    println!("{} {}", postal_code, line_item_id);
    Json(delivery_dates)
}
