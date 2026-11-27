use sqlx::{migrate::MigrateDatabase, Postgres};

/// Creates the database named in the URL when the server does not have it yet,
/// so that migrations can run on first start.
///
/// Failures are only logged: the account may lack the right to create or list
/// databases while the target already works, and connecting reports real errors.
pub async fn create_database_if_missing(db_url: &str) {
    if let Err(err) = create(db_url).await {
        tracing::warn!(%err, "could not check or create the database:");
    }
}

async fn create(db_url: &str) -> Result<(), sqlx::Error> {
    if !Postgres::database_exists(db_url).await? {
        Postgres::create_database(db_url).await?;
        tracing::info!("Created the missing database");
    }
    Ok(())
}
