use std::fmt::format;
use axum::body::Body;
use axum::response::IntoResponse;
use serde_json::json;

pub struct AxumGrantsResponse;

impl AxumGrantsResponse {
    pub fn get_into_response(msg: &str) -> impl IntoResponse {
        // axum::http::Response::builder()
        //     .status(axum::http::StatusCode::FORBIDDEN)
        //     .body(Body::from(msg.to_string())).unwrap()
        axum::Json(json!(
        {
            "cd": "403",
            "msg": msg,
        }))
    }
}
