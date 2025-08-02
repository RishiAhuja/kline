// src/http.rs

use axum::{
    routing::{get, put, delete},
    Router, extract::{Path, State}, body::Bytes, response::IntoResponse,
};
use std::sync::Arc;
use crate::Kline;
use base64::{Engine as _};

pub fn create_router(db: Arc<Kline>) -> Router {
    Router::new()
        .route("/key/{key}", get(get_key))
        .route("/key/{key}", put(put_key))
        .route("/key/{key}", delete(delete_key))
        .route("/keys", get(get_all_keys))
        .with_state(db)
}

async fn get_key(Path(key): Path<String>, State(db): State<Arc<Kline>>) -> impl IntoResponse {
    match db.get(&key.into_bytes()) {
        Some(value) => String::from_utf8_lossy(&value).to_string(),
        None => "(null)".into(),
    }
}

async fn put_key(Path(key): Path<String>, State(db): State<Arc<Kline>>, body: Bytes) -> impl IntoResponse {
    match db.put(key.into_bytes(), body.to_vec()) {
        Ok(_) => "OK",
        Err(_) => "Error storing key",
    }
}

async fn delete_key(Path(key): Path<String>, State(db): State<Arc<Kline>>) -> impl IntoResponse {
    match db.delete(&key.into_bytes()) {
        Ok(_) => "Deleted",
        Err(_) => "Error deleting key",
    }
}

async fn get_all_keys(State(db): State<Arc<Kline>>) -> impl IntoResponse {
    let mut keys = vec![];
    for key in db.keys() {
        match std::str::from_utf8(&key) {
            Ok(k) => keys.push(k.to_string()),
            Err(_) => keys.push(base64::engine::general_purpose::STANDARD.encode(&key)),
        }
    }
    keys.join("\n")
}
