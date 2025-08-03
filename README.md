# Kline

A fast, reliable, and thread-safe key-value database written in Rust with HTTP API and interactive CLI.

## Architecture

```
kline/
├── storage/           # Core database engine with WAL
├── http/             # HTTP API with JSON responses
├── cli/              # Interactive REPL interface
├── config/           # Configuration management
└── error/            # Custom error types
```

## Installation

### From Source
```bash
git clone <repository>
cd kline
cargo build --release
```

### Quick Start
```bash
# Generate default config
cargo run config-init

# Start server
cargo run

# Or start with custom settings
cargo run -- --port 8080 --data-dir /var/lib/kline
```

## Configuration

Kline uses a hierarchical configuration system with the following priority order:

1. **CLI Arguments** (highest priority)
2. **Environment Variables** 
3. **Config File** (`kline.conf`)
4. **Built-in Defaults** (lowest priority)

### Generate Default Config
```bash
cargo run config-init --output kline.conf
```

### Sample Configuration (`kline.conf`)
```toml
[server]
port = 3000
bind_address = "127.0.0.1"
max_connections = 1000

[storage]
data_dir = "./data"
compaction_interval_secs = 60
max_log_size_mb = 100

[limits]
max_key_size = 1024        # 1KB
max_value_size = 10485760  # 10MB
max_keys = 1000000         # 1M keys

[ttl]
cleanup_interval_secs = 30
default_ttl_secs = 3600    # 1 hour
max_ttl_secs = 31536000    # 1 year max
```

### Environment Variables
```bash
export KLINE_PORT=8080
export KLINE_DATA_DIR=/var/lib/kline
export KLINE_BIND_ADDRESS=0.0.0.0
```

### CLI Overrides
```bash
# Override config file settings
cargo run -- --port 9000 --data-dir /tmp/kline

# Use custom config file
cargo run -- --config /etc/kline/production.conf
```

## HTTP API

### Endpoints

| Method | Endpoint | Description | Example |
|--------|----------|-------------|---------|
| `PUT` | `/key/{key}` | Store a key-value pair | `PUT /key/user:123` |
| `GET` | `/key/{key}` | Retrieve a value | `GET /key/user:123` |
| `DELETE` | `/key/{key}` | Delete a key | `DELETE /key/user:123` |
| `GET` | `/keys` | List all keys | `GET /keys` |

### Example Usage

```bash
# Store data
curl -X PUT http://localhost:3000/key/user:123 \
  -H "Content-Type: application/stream" \
  -d "john_doe"

# Retrieve data  
curl http://localhost:3000/key/user:123

# Delete data
curl -X DELETE http://localhost:3000/key/user:123

# List all keys
curl http://localhost:3000/keys
```

### Response Format

#### Successful Get
```json
{
  "key": "user:123",
  "value": "john_doe",
  "found": true
}
```

#### Key Not Found
```json
{
  "key": "user:123", 
  "value": null,
  "found": false
}
```

#### Operation Status
```json
{
  "status": "OK"
}
```

#### List Keys
```json
{
  "keys": ["user:123", "session:abc"],
  "count": 2
}
```

## Interactive CLI (REPL)

Start the interactive shell:
```bash
cargo run
```

### Available Commands

```bash
kline> put user:123 john_doe
kline> get user:123
john_doe
kline> delete user:123
kline> keys
user:456
session:abc
kline> help
kline> exit
```

### Custom Error Handling
```rust
use kline::{Kline, KlineError, Result};

let db = Kline::open("my.db")?;

match db.put(key, value) {
    Ok(_) => println!("Stored successfully"),
    Err(KlineError::KeyTooLarge { size, max }) => {
        println!("Key too large: {} bytes (max: {})", size, max);
    }
    Err(KlineError::DatabaseFull { current, max }) => {
        println!("Database full: {}/{} keys", current, max);
    }
    Err(err) => println!("Error: {}", err),
}
```

### Resource Limits
Kline enforces configurable limits to prevent resource exhaustion:

- **Max key size**: Default 1KB
- **Max value size**: Default 10MB  
- **Max keys**: Default 1M keys
- **Database size**: Controlled by max keys × avg value size

### Data Persistence
- **Write-Ahead Log**: All operations logged before execution
- **Crash Recovery**: Database state rebuilt from log on startup
- **Auto-Compaction**: Periodic cleanup of obsolete log entries
- **Atomic Operations**: Each operation is atomic and durable

## Thread Safety

Kline is fully thread-safe:
- **Concurrent Reads**: Multiple readers can access data simultaneously
- **Exclusive Writes**: Write operations are serialized for consistency
- **HTTP + CLI**: Both interfaces can be used concurrently
- **Background Tasks**: Auto-compaction runs safely in background

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
