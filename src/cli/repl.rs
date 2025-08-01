use crate::storage::Kline;
use std::io::{self, Write};

pub fn repl(mut db: Kline) -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("kline> ");
        io::stdout().flush()?;
        stdin.read_line(&mut input)?;

        let tokens: Vec<&str> = input.trim().splitn(3, ' ').collect();
        match tokens.as_slice() {
            ["put", key, value] => {
                db.put(key.to_string(), value.to_string())?;
            }
            ["get", key] => {
                match db.get(key) {
                    Some(val) => println!("{}", val),
                    None => println!("(null)"),
                }
            }
            ["delete", key] => {
                db.delete(key)?;
            }
            ["exit"] => break,
            _ => println!("Unknown command. Use put/get/delete/exit."),
        }
    }

    Ok(())
}
