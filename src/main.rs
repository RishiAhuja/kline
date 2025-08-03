use std::sync::Arc;
use kline::{Kline, repl, KlineConfig, Result};
use tokio::task;
use clap::{Parser, Subcommand};

mod http;

#[derive(Parser)]
#[command(name = "kline")]
#[command(about = "A high-performance key-value database")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Config file path
    #[arg(short, long, default_value = "kline.conf")]
    config: String,
    
    /// Server port (overrides config file)
    #[arg(short, long)]
    port: Option<u16>,
    
    /// Data directory (overrides config file)
    #[arg(short, long)]
    data_dir: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Server,
    ConfigInit {
        #[arg(short, long, default_value = "kline.conf")]
        output: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::ConfigInit { output }) => {
            let config = KlineConfig::default();
            config.save_to_file(&output)?;
            println!("Generated default config file: {}", output);
            return Ok(());
        }
        Some(Commands::Server) | None => {
            // Start the server (default behavior)
        }
    }
    
    let mut config = if std::path::Path::new(&cli.config).exists() {
        KlineConfig::from_file(&cli.config)?
    } else {
        println!("Config file '{}' not found, using defaults", cli.config);
        println!("Run 'kline config-init' to generate a default config file");
        KlineConfig::default()
    };
    
    if let Some(port) = cli.port {
        config.server.port = port;
    }
    if let Some(data_dir) = cli.data_dir {
        config.storage.data_dir = data_dir;
    }
    
    config.apply_env_vars();
    
    start_server(config).await
}

async fn start_server(config: KlineConfig) -> Result<()> {
    std::fs::create_dir_all(&config.storage.data_dir)?;
    
    let db_path = format!("{}/{}", config.storage.data_dir, "kline.db");
    let db = Arc::new(Kline::open_with_config(&db_path, config.clone())?);

    println!("Kline database started!");
    println!("Data directory: {}", config.storage.data_dir);
    println!("Max key size: {} bytes", config.limits.max_key_size);
    println!("Max value size: {} bytes", config.limits.max_value_size);
    println!("Max keys: {}", config.limits.max_keys);

    let http_db = db.clone();
    let server_config = config.server.clone();
    task::spawn(async move {
        let app = http::create_router(http_db);
        let bind_addr = format!("{}:{}", server_config.bind_address, server_config.port);
        let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
        println!("HTTP server running at http://{}", bind_addr);
        axum::serve(listener, app).await.unwrap();
    });

    // Start REPL
    repl(db)?;

    Ok(())
}
