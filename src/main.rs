use clap::Parser;

use file_store_with_metadata::config::Config;
use file_store_with_metadata::services::db::connect;
use file_store_with_metadata::context::TheGoods;
use file_store_with_metadata::http::organization;
// use file_store_with_metadata::services::gcs::GCSClient;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load env vars
    _ = dotenvy::dotenv();
    let config = Config::parse();

    let db = connect(&config.database_url).await?;
    // let gcs = GCSClient::new().await?;

    let goodies = TheGoods::new(db);

    Ok(())
}


// TODO:
// - Make database functions
    // - CRUD user

// TODO:

// Idea:
// Each organization stores its user assets in different buckets
//      An organization bucket name is something like "[org name]_user_asset_bucket"
// Each org is differentiated by the first part of the path. I.e. www.website.com/{org}/requested endpoint
//      Can then extract and get org obj from db
