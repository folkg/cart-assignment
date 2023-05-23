use std::fs;

use axum::{extract::Query, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub async fn item_handler() -> Result<Json<Value>, (StatusCode, String)> {
    let data = fs::read_to_string("json/line_items.json").map_err(|e| {
        let error_message = format!("Failed to read file: {}", e);
        eprintln!("{}", error_message);
        (StatusCode::INTERNAL_SERVER_ERROR, error_message)
    })?;
    let items = serde_json::from_str(&data).map_err(|e| {
        let error_message = format!("Failed to parse JSON file: {}", e);
        eprintln!("{}", error_message);
        (StatusCode::INTERNAL_SERVER_ERROR, error_message)
    })?;

    Ok(Json(items))
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

pub async fn delivery_handler(
    params: Query<DeliveryQueryParams>,
) -> Result<Json<impl Serialize>, (StatusCode, String)> {
    let DeliveryQueryParams {
        postal_code,
        line_item_id,
        ..
    } = params.0;

    let line_item_id: usize = line_item_id.parse().unwrap_or_default();
    let postal_code_first_letter = postal_code.chars().next().unwrap_or_default().to_string();

    let data = fs::read_to_string("json/delivery_dates.json").map_err(|e| {
        let error_message = format!("Failed to read file: {}", e);
        eprintln!("{}", error_message);
        (StatusCode::INTERNAL_SERVER_ERROR, error_message)
    })?;
    let delivery_dates: Vec<DeliveryDate> = serde_json::from_str(&data).map_err(|e| {
        let error_message = format!("Failed to parse JSON file: {}", e);
        eprintln!("{}", error_message);
        (StatusCode::INTERNAL_SERVER_ERROR, error_message)
    })?;

    let result = delivery_dates
        .iter()
        .find(|date| date.postal == postal_code_first_letter && date.ids.contains(&line_item_id));

    if let Some(date) = result {
        return Ok(Json(date.estimated_delivery_date.clone()));
    } else {
        return Ok(Json("TBD".to_string()));
    }
}
