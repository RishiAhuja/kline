use crate::storage::Kline;
use base64::Engine as _;
use base64::engine::general_purpose;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::sync::Arc;

pub fn repl(db: Arc<Kline>) -> std::io::Result<()> {
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
                db.put(key.as_bytes().to_vec(), value.as_bytes().to_vec())?;
            }
            ["get", key] => {
                match db.get(key.as_bytes()) {
                    Some(val) => println!("{}", String::from_utf8_lossy(&val)),
                    None => println!("(null)"),
                }
            }
            ["delete", key] => {
                db.delete(key.as_bytes())?;
            }
            ["keys"] => {
                for key in db.keys() {
                    match std::str::from_utf8(&key) {
                        Ok(k) => println!("{}", k),
                        Err(_) => println!("{}", general_purpose::STANDARD.encode(&key)),
                    }
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
