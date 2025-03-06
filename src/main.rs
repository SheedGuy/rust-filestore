use clap::Parser;

use file_store_with_metadata::config::Config;
use file_store_with_metadata::services::db::connect;
use file_store_with_metadata::services::gcs::GCSClient;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenvy::dotenv();

    let config = Config::parse();

    let db = connect(&config.database_url).await?;

    let gcs = GCSClient::new().await?;

    let buckets = gcs.list_buckets(&config.project_id).await?;

    for buck in buckets {
        println!("{}", buck.name);
    }

    Ok(())
}
