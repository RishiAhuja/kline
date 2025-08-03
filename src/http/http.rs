use axum::{
    body::Bytes, 
    extract::{Path, State}, 
    response::IntoResponse, 
    routing::{delete, get, put}, 
    Json, 
    Router
};
use std::sync::Arc;
use crate::Kline;
use base64::{Engine as _};
use super::responses::*;

pub fn create_router(db: Arc<Kline>) -> Router {
    Router::new()
        .route("/key/{key}", get(get_key))
        .route("/key/{key}", put(put_key))
        .route("/key/{key}", delete(delete_key))
        .route("/keys", get(get_all_keys))
        .with_state(db)
}

async fn get_key(Path(key): Path<String>, State(db): State<Arc<Kline>>) -> impl IntoResponse {
    let key_bytes = key.as_bytes();
    match db.get(key_bytes) {
        Ok(Some(value)) => {
            let value_str = String::from_utf8_lossy(&value).to_string();
            Json(ValueResponse::found(key, value_str))
        }
        Ok(None) => Json(ValueResponse::not_found(key.clone())),
        Err(_) => Json(ValueResponse::not_found(key)),
    }
}

async fn put_key(Path(key): Path<String>, State(db): State<Arc<Kline>>, body: Bytes) -> impl IntoResponse {
    match db.put(key.into_bytes(), body.to_vec()) {
        Ok(_) => Json(StatusResponse::ok()),
        Err(err) => Json(StatusResponse::error(format!("Error storing key: {}", err))),
    }
}

async fn delete_key(Path(key): Path<String>, State(db): State<Arc<Kline>>) -> impl IntoResponse {
    match db.delete(&key.into_bytes()) {
        Ok(_) => Json(StatusResponse::deleted()),
        Err(err) => Json(StatusResponse::error(format!("Error deleting key: {}", err))),
    }
}

async fn get_all_keys(State(db): State<Arc<Kline>>) -> impl IntoResponse {
    match db.keys() {
        Ok(key_list) => {
            let mut keys = vec![];
            for key in key_list {
                match std::str::from_utf8(&key) {
                    Ok(k) => keys.push(k.to_string()),
                    Err(_) => keys.push(base64::engine::general_purpose::STANDARD.encode(&key)),
                }
            }
            Json(KeysResponse::new(keys))
        }
        Err(err) => Json(KeysResponse::error(&format!("Error getting keys: {}", err))),
    }
}
