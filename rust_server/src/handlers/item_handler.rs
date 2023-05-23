use std::fs;

use axum::{http::StatusCode, Json};
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
