pub mod user;
pub mod organization;

use axum::Router;
use anyhow::Result;


use crate::context::TheGoods;

fn new(goodies: TheGoods) -> Router<TheGoods> {
    Router::new()
    .merge(organization::router())
    .merge(user::router())
    .with_state(goodies)
}

pub async fn serve(goodie_bag: TheGoods, port: u16) -> Result<()> {
    let app = new(goodie_bag);

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await?;

    axum::serve(listener, app);

    Ok(())
}