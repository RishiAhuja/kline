use std::sync::Arc;
use kline::{Kline, repl, constants::db::DEFAULT_DB_FILE};
use tokio::task;

mod http;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = Arc::new(Kline::open(DEFAULT_DB_FILE)?);

    // Spawn HTTP server
    let http_db = db.clone();
    task::spawn(async move {
        let app = http::create_router(http_db);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
        println!("HTTP server running at http://127.0.0.1:3000");
        axum::serve(listener, app).await.unwrap();
    });

    // Start REPL
    repl(db)?;

    Ok(())
}
