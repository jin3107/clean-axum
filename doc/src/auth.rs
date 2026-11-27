use utoipa::OpenApi;

use models::params::auth::{LoginParams, RefreshParams, RegisterParams};
use models::schemas::auth::TokenSchema;

use api::models::{ApiErrorResponse, ParamsErrorResponse};
use api::routers::auth::*;

#[derive(OpenApi)]
#[openapi(
    paths(register_post, login_post, refresh_post, logout_post),
    components(schemas(
        RegisterParams,
        LoginParams,
        RefreshParams,
        TokenSchema,
        ApiErrorResponse,
        ParamsErrorResponse,
    ))
)]
pub(super) struct AuthApi;
