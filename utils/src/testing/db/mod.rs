use sea_orm::{Database, DatabaseConnection, DbErr};
use tokio::sync::OnceCell;

use crate::{create_database_if_missing, migrate};

// tests of one binary share the schema, so only one of them may create it
static SCHEMA: OnceCell<()> = OnceCell::const_new();

/// Connects to the test database, creating and migrating it on first use.
pub async fn setup_test_db() -> Result<DatabaseConnection, DbErr> {
    let url = test_db_url();
    SCHEMA
        .get_or_try_init(|| async {
            create_database_if_missing(&url).await;
            let db = Database::connect(url.as_str()).await?;
            migrate(&db).await
        })
        .await?;
    Database::connect(url.as_str()).await
}

/// `TEST_DATABASE_URL`, or else the `DATABASE_URL` database with a `_test`
/// suffix, so that tests never write into the development database.
fn test_db_url() -> String {
    dotenvy::dotenv().ok();
    if let Ok(url) = std::env::var("TEST_DATABASE_URL") {
        return url;
    }
    let url = std::env::var("DATABASE_URL")
        .expect("Set DATABASE_URL in .env, or TEST_DATABASE_URL, to run the tests");
    with_test_suffix(&url)
}

fn with_test_suffix(url: &str) -> String {
    match url.split_once('?') {
        Some((base, query)) => format!("{base}_test?{query}"),
        None => format!("{url}_test"),
    }
}

#[cfg(test)]
mod tests {
    use super::with_test_suffix;

    #[test]
    fn suffix_goes_on_the_database_name() {
        assert_eq!(
            with_test_suffix("postgres://u:p@localhost/app"),
            "postgres://u:p@localhost/app_test"
        );
        assert_eq!(
            with_test_suffix("postgres://u@localhost/app?sslmode=disable"),
            "postgres://u@localhost/app_test?sslmode=disable"
        );
    }
}
