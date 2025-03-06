use clap::Parser;

use file_store_with_metadata::config::Config;
use file_store_with_metadata::db::connect;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenvy::dotenv();

    let config = Config::parse();

    println!("db url is {}", config.database_url);

    println!("Attempting to connect");

    let db = connect(&config.database_url).await?;

    print!("Magic 8 ball, the db is connected: {}", db.is_closed());

    db.close().await;

    Ok(())
}
