use sea_orm::{Database, DatabaseConnection, DbErr};

use crate::migrate;

pub async fn setup_test_db(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(db_url).await?;
    migrate(&db).await?;
    Ok(db)
}

/// `TEST_DATABASE_URL` runs the same tests against a real server.
pub fn test_db_url(default: &str) -> String {
    std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| default.to_string())
}
