use sqlx::{migrate::MigrateDatabase, MySql, Postgres};

/// Creates the database named in the URL when the server does not have it yet,
/// so that migrations can run on first start. SQLite files are handled elsewhere.
///
/// Failures are only logged: the account may lack the right to create or list
/// databases while the target already works, and connecting reports real errors.
pub async fn create_database_if_missing(db_url: &str) {
    let result = match db_url.split_once("://").map(|(scheme, _)| scheme) {
        Some("postgres" | "postgresql") => create::<Postgres>(db_url).await,
        Some("mysql" | "mariadb") => create::<MySql>(db_url).await,
        _ => return,
    };

    if let Err(err) = result {
        tracing::warn!(%err, "could not check or create the database:");
    }
}

async fn create<D: MigrateDatabase>(db_url: &str) -> Result<(), sqlx::Error> {
    if !D::database_exists(db_url).await? {
        D::create_database(db_url).await?;
        tracing::info!("Created the missing database");
    }
    Ok(())
}
