use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

/// Creates a new tenant together with its first admin.
#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterParams {
    #[validate(length(min = 2, max = 100))]
    pub tenant_name: String,

    #[validate(length(min = 2, max = 100))]
    pub username: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginParams {
    #[validate(length(min = 2, max = 100))]
    pub tenant_name: String,

    #[validate(length(min = 2, max = 100))]
    pub username: String,

    #[validate(length(min = 1, max = 128))]
    pub password: String,
}
