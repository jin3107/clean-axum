use utoipa::OpenApi;

use models::params::auth::{LoginParams, RegisterParams};
use models::schemas::auth::TokenSchema;

use api::models::{ApiErrorResponse, ParamsErrorResponse};
use api::routers::auth::*;

#[derive(OpenApi)]
#[openapi(
    paths(register_post, login_post),
    components(schemas(
        RegisterParams,
        LoginParams,
        TokenSchema,
        ApiErrorResponse,
        ParamsErrorResponse,
    ))
)]
pub(super) struct AuthApi;
