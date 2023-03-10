use std::env;

use anyhow::Ok;
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

pub async fn init() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect(".env must have DATABASE_URL");

    let db: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&db, None).await.unwrap();

    Ok(())
}
