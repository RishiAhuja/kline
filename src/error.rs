use thiserror::Error;

#[derive(Error, Debug)]
pub enum KlineError {
    #[error("Key not found: {key}")]
    KeyNotFound { key: String },
    
    #[error("Key expired: {key}")]
    KeyExpired { key: String },
    
    #[error("Key too large: {size} bytes (max: {max})")]
    KeyTooLarge { size: usize, max: usize },
    
    #[error("Value too large: {size} bytes (max: {max})")]
    ValueTooLarge { size: usize, max: usize },
    
    #[error("Invalid TTL: {ttl} seconds")]
    InvalidTtl { ttl: u64 },
    
    #[error("Database full: {current}/{max} keys")]
    DatabaseFull { current: usize, max: usize },
    
    #[error("Config parse error: {reason}")]
    ConfigParse { reason: String },
    
    #[error("Config serialize error: {reason}")]
    ConfigSerialize { reason: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Invalid key format: must be valid UTF-8")]
    InvalidKeyFormat,
    
    #[error("Lock poisoned")]
    LockPoisoned,
}

pub type Result<T> = std::result::Result<T, KlineError>;