use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::domains::role::Role;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateUserParams {
    #[validate(length(min = 2, max = 100))]
    pub username: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,

    pub role: Role,
}
