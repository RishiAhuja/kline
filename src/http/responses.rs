use serde::Serialize;

/// Response for operations that return a simple status
#[derive(Serialize)]
pub struct StatusResponse {
    pub status: &'static str,
}

/// Response for get operations that return a value
#[derive(Serialize)]
pub struct ValueResponse {
    pub key: String,
    pub value: Option<String>,
    pub found: bool,
}

/// Response for listing all keys
#[derive(Serialize)]
pub struct KeysResponse {
    pub keys: Vec<String>,
    pub count: usize,
}

/// Error response with more detailed information
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: &'static str,
    pub message: String,
}

impl StatusResponse {
    pub fn ok() -> Self {
        Self { status: "OK" }
    }
    
    pub fn deleted() -> Self {
        Self { status: "Deleted" }
    }
    
    pub fn error(msg: &'static str) -> Self {
        Self { status: msg }
    }
}

impl ValueResponse {
    pub fn found(key: String, value: String) -> Self {
        Self {
            key,
            value: Some(value),
            found: true,
        }
    }
    
    pub fn not_found(key: String) -> Self {
        Self {
            key,
            value: None,
            found: false,
        }
    }
}

impl KeysResponse {
    pub fn new(keys: Vec<String>) -> Self {
        let count = keys.len();
        Self { keys, count }
    }
}