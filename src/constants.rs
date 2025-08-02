use std::time::Duration;

/// Database configuration constants
pub mod db {
    use super::*;
    
    pub const COMPACTION_INTERVAL_SECS: u64 = 60;
    pub const COMPACTION_INTERVAL: Duration = Duration::from_secs(COMPACTION_INTERVAL_SECS);
    pub const DEFAULT_DB_FILE: &str = "kline.db";
    pub const TEMP_FILE_SUFFIX: &str = ".tmp";
    pub const MAX_OPS_BEFORE_COMPACTION: usize = 1000;
}

/// CLI configuration constants
pub mod cli {
    pub const PROMPT: &str = "kline> ";
    pub const NULL_DISPLAY: &str = "(null)";
    pub const UNKNOWN_COMMAND_MSG: &str = "Unknown command. Use put/get/delete/clear/keys/exit.";
}

/// Storage configuration constants
pub mod storage {
    pub const INITIAL_HASHMAP_CAPACITY: usize = 1024;
    pub const IO_BUFFER_SIZE: usize = 8192;
}
