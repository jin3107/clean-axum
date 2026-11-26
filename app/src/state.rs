use sea_orm::DatabaseConnection;

use crate::auth::JwtKeys;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub jwt: JwtKeys,
}
