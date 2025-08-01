use kline::{Kline, repl};


fn main() -> std::io::Result<()> {
    let db = Kline::open("kline.db")?;
    repl(db)?;
    // db.compact()?;
    // db.put("rishi".to_string(), "hello".to_string())?;
    // db.put("foo".to_string(), "bar".to_string())?;
    // println!("{:?}", db.get("rishi")); // Some("hello")

    // db.delete("rishi")?;
    // println!("{:?}", db.get("rishi")); // None

    Ok(())
}


