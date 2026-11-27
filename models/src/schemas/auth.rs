use serde::Serialize;
use utoipa::ToSchema;

use super::user::UserSchema;

#[derive(Serialize, ToSchema)]
pub struct TokenSchema {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    /// Single use: each refresh returns the next one.
    pub refresh_token: String,
    pub refresh_expires_in: u64,
    pub user: UserSchema,
}
