use api::{setup_db, setup_router, with_middleware, SecurityConfig};
use doc::ApiDoc;
use utils::migrate;

pub async fn run(db_url: &str) -> shuttle_axum::ShuttleAxum {
    tracing::info!("Starting with shuttle");

    let conn = setup_db(&db_url, false).await;
    migrate(&conn).await.expect("Migration failed!");

    // Shuttle serves the router itself, so all clients share one rate limit
    let security = SecurityConfig::from_env();
    let mut router = setup_router(conn, &security);
    if security.docs_enabled {
        router = router.attach_doc();
    }
    Ok(with_middleware(router, &security).into())
}
