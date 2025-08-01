# Cline

A simple key-value database written in Rust with persistent storage and crash recovery.

## Features

- **In-memory storage** with HashMap for fast lookups
- **Persistent logging** with write-ahead log (WAL) pattern
- **Crash recovery** by replaying log files
- **Interactive REPL** for database operations
- **Log compaction** to prevent unbounded file growth

## Usage

Run the interactive shell:

```bash
cargo run
```

### Commands

- `put <key> <value>` - Store a key-value pair
- `get <key>` - Retrieve a value by key
- `delete <key>` - Delete a key
- `exit` - Exit the program

### Example

```
kline> put name alice
kline> put age 25
kline> get name
alice
kline> delete age
kline> get age
(null)
kline> exit
```

## Architecture

- **Storage Engine**: Core database functionality
- **Write-Ahead Log**: All operations are logged to disk before execution
- **In-Memory Index**: HashMap for fast key lookups
- **REPL Interface**: Interactive command-line interface

## Data Persistence

All operations are logged to `kline.db` in the following format:
- `put <key> <value>` - Insert or update operation
- `delete <key>` - Delete operation

On startup, the database replays the log file to rebuild the in-memory state.

## Building

```bash
cargo build --release
```
