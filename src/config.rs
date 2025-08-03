use serde::{Deserialize, Serialize};
use crate::error::{KlineError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlineConfig {
    pub storage: StorageConfig,
    pub server: ServerConfig,
    pub limits: LimitsConfig,
    pub ttl: TtlConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub data_dir: String,
    pub compaction_interval_secs: u64,
    pub max_log_size_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub bind_address: String,
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    pub max_key_size: usize,      
    pub max_value_size: usize,   
    pub max_keys: usize,        
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtlConfig {
    pub cleanup_interval_secs: u64, 
    pub default_ttl_secs: Option<u64>,
    pub max_ttl_secs: u64,          
}

impl Default for KlineConfig {
    fn default() -> Self {
        Self {
            storage: StorageConfig {
                data_dir: "./data".to_string(),
                compaction_interval_secs: 60,
                max_log_size_mb: 100,
            },
            server: ServerConfig {
                port: 3000,
                bind_address: "127.0.0.1".to_string(),
                max_connections: 1000,
            },
            limits: LimitsConfig {
                max_key_size: 1024,      
                max_value_size: 10_485_760, 
                max_keys: 1_000_000,      
            },
            ttl: TtlConfig {
                cleanup_interval_secs: 30,
                default_ttl_secs: None,
                max_ttl_secs: 86400 * 365, 
            },
        }
    }
}

impl KlineConfig {

    pub fn load() -> Result<Self> {
        let mut config = Self::default();
        
        if let Ok(file_config) = Self::from_file("kline.conf") {
            config = file_config;
        }
        
        config.apply_env_vars();
        
        Ok(config)
    }
    
    pub fn from_file(path: &str) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: KlineConfig = toml::from_str(&contents)
            .map_err(|e| KlineError::ConfigParse { reason: e.to_string() })?;
        Ok(config)
    }
    
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .map_err(|e| KlineError::ConfigSerialize { reason: e.to_string() })?;
        std::fs::write(path, contents)?;
        Ok(())
    }
    
    pub fn apply_env_vars(&mut self) {
        // KLINE_PORT=3000 -> server.port = 3000
        if let Ok(port) = std::env::var("KLINE_PORT") {
            if let Ok(port) = port.parse() {
                self.server.port = port;
            }
        }
        
        // KLINE_DATA_DIR=/data -> storage.data_dir = "/data"
        if let Ok(data_dir) = std::env::var("KLINE_DATA_DIR") {
            self.storage.data_dir = data_dir;
        }
        
        // KLINE_BIND_ADDRESS=0.0.0.0 -> server.bind_address = "0.0.0.0"
        if let Ok(bind_addr) = std::env::var("KLINE_BIND_ADDRESS") {
            self.server.bind_address = bind_addr;
        }
    }
}