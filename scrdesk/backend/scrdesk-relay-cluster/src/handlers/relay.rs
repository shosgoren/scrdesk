use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::AppState;

pub async fn get_relay_status(State(_state): State<Arc<AppState>>) -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({
        "status": "running",
        "port": 21117,
        "active_connections": 0,
        "protocol": "RustDesk compatible"
    })))
}
