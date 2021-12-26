use chrono::DateTime;
use sea_orm::entity::Set;
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
    let user = entity_expanded::users::ActiveModel {
        uuid: Set(String::from("test-user000")),
        name: Set(String::from("test-user")),
        email: Set(String::from("test-user@testmail.com")),
        image: Set(Some(String::from("test-user.png"))),
        description: Set(Some(String::from("I am test-user1"))),
        created_at: Set(DateTime::parse_from_rfc3339("2021-12-26T12:30:00+09:00").unwrap()), //DateTime::parse_from_rfc3339 だとparseされない
        updated_at: Set(Some(DateTime::parse_from_rfc3339(
            "2021-12-26T14:30:00+09:00",
        )?)),
        deleted_at: Set(Some(DateTime::parse_from_rfc3339(
            "2021-12-26T19:30:00+09:00",
        )?)),
    };
    user.insert(&db).await?;

    // Insert category
    let category = entity_expanded::categories::ActiveModel {
        uuid: Set(String::from("categories101")),
        sub_category_uuid: Set(None),
        name: Set(String::from("運動")),
        created_at: Set(DateTime::parse_from_rfc3339("2021-12-26T12:30:00+09:00").unwrap()),
        updated_at: Set(Some(
            DateTime::parse_from_rfc3339("2021-12-26T14:30:00+09:00").unwrap(),
        )),
        deleted_at: Set(Some(
            DateTime::parse_from_rfc3339("2021-12-26T19:30:00+09:00").unwrap(),
        )),
    };
    category.insert(&db).await?;

    let activity = entity_expanded::activities::ActiveModel {
        uuid: Set(String::from("activities102")),
        name: Set(String::from("ランニング 10km")),
        category_uuid: Set(String::from("categories1")),
        created_at: Set(DateTime::parse_from_rfc3339("2021-12-26T12:30:00+09:00").unwrap()),
        updated_at: Set(DateTime::parse_from_rfc3339("2021-12-26T14:30:00+09:00").unwrap()),
        deleted_at: Set(DateTime::parse_from_rfc3339("2021-12-26T19:30:00+09:00").unwrap()),
    };
    activity.insert(&db).await?;

    Ok(())
}
