use kline::{Kline, repl};


fn main() -> std::io::Result<()> {
    let db = Kline::open("kline.db")?;
    repl(db)?;
    Ok(())
}


