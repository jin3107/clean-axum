use api::{setup_db, setup_router};
use doc::ApiDoc;
use utils::migrate;

pub async fn run(db_url: &str) -> shuttle_axum::ShuttleAxum {
    tracing::info!("Starting with shuttle");

    let conn = setup_db(&db_url, false).await;
    migrate(&conn).await.expect("Migration failed!");

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is not set");
    let router = setup_router(conn, &jwt_secret).attach_doc();
    Ok(router.into())
}
