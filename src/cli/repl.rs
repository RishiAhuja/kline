use crate::storage::Kline;
use crate::error::{Result};
use base64::Engine as _;
use base64::engine::general_purpose;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::sync::Arc;

pub fn repl(db: Arc<Kline>) -> Result<()> {
    let mut rl = DefaultEditor::new().expect("Failed to initialize rustyline");

    loop {
        let input = match rl.readline("kline> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).ok();
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted (CTRL+C)");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting (CTRL+D)");
                break;
            }
            Err(err) => {
                eprintln!("Error reading line: {:?}", err);
                break;
            }
        };

        let tokens: Vec<&str> = input.trim().splitn(3, ' ').collect();
        match tokens.as_slice() {
            ["put", key, value] => {
                if let Err(err) = db.put(key.as_bytes().to_vec(), value.as_bytes().to_vec()) {
                    println!("Error storing key: {}", err);
                }
            }
            ["get", key] => {
                match db.get(key.as_bytes()) {
                    Ok(Some(val)) => println!("{}", String::from_utf8_lossy(&val)),
                    Ok(None) => println!("(null)"),
                    Err(err) => println!("Error: {}", err),
                }
            }
            ["delete", key] => {
                if let Err(err) = db.delete(key.as_bytes()) {
                    println!("Error deleting key: {}", err);
                }
            }
            ["keys"] => {
                match db.keys() {
                    Ok(key_list) => {
                        for key in key_list {
                            match std::str::from_utf8(&key) {
                                Ok(k) => println!("{}", k),
                                Err(_) => println!("{}", general_purpose::STANDARD.encode(&key)),
                            }
                        }
                    }
                    Err(err) => println!("Error getting keys: {}", err),
                }
            }
            ["help"] => {
                println!("Available commands:");
                println!("  put <key> <value> - Store a key-value pair");
                println!("  get <key> - Retrieve a value by key");
                println!("  delete <key> - Remove a key-value pair");
                println!("  keys - List all keys in the database");
                println!("  exit - Exit the REPL");
            }
            ["exit"] => break,
            _ => println!("Unknown command. Use put/get/delete/exit."),
        }
    }

    Ok(())
}
