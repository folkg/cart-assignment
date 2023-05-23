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

// TODO: Extract the delivery date stuff to a new module
#[derive(Deserialize)]
pub struct DeliveryQueryParams {
    #[serde(rename = "postalCode")]
    postal_code: String,

    #[serde(rename = "lineItemId")]
    line_item_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeliveryDate {
    postal: String,
    ids: Vec<usize>,

    #[serde(rename = "estimatedDeliveryDate")]
    estimated_delivery_date: String,
}

pub async fn delivery_handler(params: Query<DeliveryQueryParams>) -> Json<impl Serialize> {
    let DeliveryQueryParams {
        postal_code,
        line_item_id,
        ..
    } = params.0;

    let line_item_id: usize = match line_item_id.parse() {
        Ok(line_item_id) => line_item_id,
        Err(_) => 0,
    };
    let postal_code_first_letter = if let Some(letter) = postal_code.chars().next() {
        letter.to_string()
    } else {
        "".to_string()
    };

    println!("{} {}", postal_code, line_item_id);

    // TODO: Better error handling to send back status 500 or something instead of panicing
    let data =
        fs::read_to_string("json/delivery_dates.json").expect("Should be able to read file.");
    let delivery_dates: Vec<DeliveryDate> =
        serde_json::from_str(&data).expect("Should have the correct JSON format.");

    let result = delivery_dates
        .iter()
        .find(|date| date.postal == postal_code_first_letter && date.ids.contains(&line_item_id));

    if let Some(date) = result {
        return Json(date.estimated_delivery_date.clone());
    } else {
        return Json("TBD".to_string());
    }
}
