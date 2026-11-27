use std::time::Duration;

use axum::Router;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use app::auth::JwtKeys;
use app::config::{Config, SecurityConfig};
use app::state::AppState;
use utils::create_database_if_missing;

use crate::middleware::{cors_layer, RateLimiter};
use crate::routers::create_router;

pub fn setup_router(conn: DatabaseConnection, security: &SecurityConfig) -> Router {
    let state = AppState {
        conn,
        jwt: JwtKeys::new(
            &security.jwt_secret,
            &security.jwt_issuer,
            &security.jwt_audience,
        ),
    };
    create_router(state, RateLimiter::new(security.auth_rate_limit))
}

/// Adds CORS and request logging. Apply it after every route has been added,
/// because a layer does not cover routes merged later.
pub fn with_middleware(router: Router, security: &SecurityConfig) -> Router {
    router.layer(cors_layer(&security.cors_origins)).layer(
        // method, path, status and latency; headers and bodies are not logged
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    )
}

pub fn setup_config() -> Config {
    dotenvy::dotenv().ok();
    Config::from_env()
}

pub async fn setup_db(db_url: &str, prefork: bool) -> DatabaseConnection {
    create_database_if_missing(db_url).await;

    let mut opt = ConnectOptions::new(db_url);
    opt.max_lifetime(Duration::from_secs(60));

    if !prefork {
        opt.min_connections(10).max_connections(100);
    }

    Database::connect(opt)
        .await
        .expect("Database connection failed")
}
