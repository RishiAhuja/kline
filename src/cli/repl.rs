use crate::storage::Kline;
use base64::Engine as _;
use base64::engine::general_purpose;
use rustyline::{DefaultEditor, error::ReadlineError};

pub fn repl(mut db: Kline) -> std::io::Result<()> {
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
                    Some(val) => println!("{}", String::from_utf8_lossy(val)),
                    None => println!("(null)"),
                }
            }
            ["delete", key] => {
                db.delete(key.as_bytes())?;
            }
            ["compact"] => {
                db.compact()?;
            }
            ["keys"] => {
                for key in db.keys() {
                    match std::str::from_utf8(key) {
                        Ok(k) => println!("{}", k),
                        Err(_) => println!("{}", general_purpose::STANDARD.encode(key)),
                    }
                }
            }
            ["clear"] => {
                if let Err(err) = db.clear() {
                    eprintln!("Failed to clear store: {}", err);
                } else {
                    println!("Store cleared.");
                }
            }
            ["help"] => {
                println!("Available commands:");
                println!("  put <key> <value> - Store a key-value pair");
                println!("  get <key> - Retrieve a value by key");
                println!("  delete <key> - Remove a key-value pair");
                println!("  compact - Compact the database");
                println!("  keys - List all keys in the database");
                println!("  clear - Clear the entire store");
                println!("  exit - Exit the REPL");
            }
            ["exit"] => break,
            _ => println!("Unknown command. Use put/get/delete/exit."),
        }
    }

    Ok(())
}
