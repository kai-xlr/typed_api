use std::fmt;

use axum::{
    Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    InvalidConfig(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::InvalidConfig(msg) => write!(f, "Invalid Config: {}", msg),
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::InvalidConfig(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };

        let body = Json(serde_json::json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct StartRequest {
    pub config_id: String,
    pub active: bool,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub service: String,
}

async fn start_handler(Json(payload): Json<StartRequest>) -> Result<impl IntoResponse, ApiError> {
    if payload.config_id.is_empty() {
        return Err(ApiError::InvalidConfig(
            "Validation failed: 'config_id' cannot be empty.".to_string(),
        ));
    }

    if payload.config_id == "not-found-id" {
        return Err(ApiError::NotFound(format!(
            "Configuration profile '{}' was not found.",
            payload.config_id
        )));
    }

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "message": "Service started successfully",
            "config_id": payload.config_id,
            "active": payload.active

        })),
    ))
}

async fn status_handler() -> Result<impl IntoResponse, ApiError> {
    let response = StatusResponse {
        status: "operational".to_string(),
        service: "axum-core_api".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/start", post(start_handler))
        .route("/status", get(status_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
