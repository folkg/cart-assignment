use std::fs;

use axum::{extract::Query, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use serde_json::{json, Value};

    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn test_delivery_handler_success_1() {
        let app = super::super::super::app();

        let req = Request::builder()
            .uri("/api/delivery?postalCode=M2R0N7&lineItemId=1")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), 200);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let expected_body = json!("Dec 19, 2021");

        assert_eq!(body, expected_body);
    }

    #[tokio::test]
    async fn test_delivery_handler_success_2() {
        let app = super::super::super::app();

        let req = Request::builder()
            .uri("/api/delivery?postalCode=K2R0N7&lineItemId=3")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), 200);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let expected_body = json!("Dec 24, 2021");

        assert_eq!(body, expected_body);
    }

    #[tokio::test]
    async fn test_delivery_handler_success_tba() {
        let app = super::super::super::app();

        let req = Request::builder()
            .uri("/api/delivery?postalCode=A&lineItemId=4")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), 200);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let expected_body = json!("TBD");

        assert_eq!(body, expected_body);
    }

    #[tokio::test]
    async fn test_delivery_handler_missing_postal_code() {
        let app = super::super::super::app();

        let req = Request::builder()
            .uri("/api/delivery?lineItemId=3")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), 400);
    }

    #[tokio::test]
    async fn test_delivery_handler_missing_line_item_id() {
        let app = super::super::super::app();

        let req = Request::builder()
            .uri("/api/delivery?postalCode=K2R0N7")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), 400);
    }

    #[tokio::test]
    async fn test_delivery_handler_incorrect_uri() {
        let app = super::super::super::app();

        let req = Request::builder()
            .uri("/delivery")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), 404);
    }
}
