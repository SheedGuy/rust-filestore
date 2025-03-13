use clap::Parser;
use tracing_subscriber;

use file_store_with_metadata::config::Config;
use file_store_with_metadata::context::TheGoods;
use file_store_with_metadata::http;
use file_store_with_metadata::services::db::connect;
// use file_store_with_metadata::services::gcs::GCSClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load env vars
    _ = dotenvy::dotenv();
    let config = Config::parse();

    tracing_subscriber::fmt::init();

    let db = connect(&config.database_url).await?;
    // let gcs = GCSClient::new().await?;

    let goodies = TheGoods::new(db);

    http::serve(goodies, 3000).await?;

    Ok(())
}

// TODO:
// - Test User/Org CRUD endpoints w/ Postman
// - Create 3 permanent test orgs
//   - "Need" to be permanent so I can create the buckets and leave them alone afterwards
// - Start planning media endpoints/storage
// - OpenApi (utoipa)

// Idea:
// Each organization stores its user assets in different buckets
//      An organization bucket name is something like "[org name]_user_asset_bucket"
// Each org is differentiated by the first part of the path. I.e. www.website.com/{org}/requested endpoint
//      Can then extract and get org obj from db
