use sea_orm::prelude::*;
use sea_orm::{ConnectOptions, Database};
use sea_ql::entity_expanded;
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    tracing_subscriber::fmt().init();

    // connect options
    let mut opt = ConnectOptions::new(env!("DATABASE_URL").to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    let db = Database::connect(opt).await?;

    // Insert
    let activity = entity_expanded::activities::ActiveModel {
        uuid: Set(String::from("activity")),
        name: Set(String::from("ランニング 10km")),
    };
    activity.insert(&db).await?;

    Ok(())
}
