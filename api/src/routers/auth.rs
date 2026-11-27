use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Router};

use app::auth::{hash_password, verify_password, TOKEN_TTL_SECS};
use app::error::AuthError;
use app::persistence::tenant::{create_tenant_with_admin, find_tenant_by_name};
use app::persistence::user::find_user_by_username;
use app::session::{end_session, rotate_session, start_session, Rotation, REFRESH_TTL_SECS};
use app::state::AppState;
use app::tenant::TenantId;
use models::domains::user;
use models::params::auth::{LoginParams, RefreshParams, RegisterParams};
use models::schemas::auth::TokenSchema;
use models::schemas::user::UserSchema;

use crate::error::ApiError;
use crate::extractor::{Json, Valid};
use crate::models::{ApiErrorResponse, ParamsErrorResponse};

fn token_response(
    state: &AppState,
    user: user::Model,
    refresh_token: String,
) -> Result<TokenSchema, ApiError> {
    Ok(TokenSchema {
        access_token: state.jwt.issue(user.id, user.tenant_id, user.role)?,
        token_type: "Bearer".to_string(),
        expires_in: TOKEN_TTL_SECS,
        refresh_token,
        refresh_expires_in: REFRESH_TTL_SECS as u64,
        user: UserSchema::from(user),
    })
}

/// Tokens for someone who has just proved who they are.
async fn sign_in(state: &AppState, user: user::Model) -> Result<TokenSchema, ApiError> {
    let refresh = start_session(&state.conn, TenantId(user.tenant_id), user.id).await?;
    token_response(state, user, refresh)
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterParams,
    responses(
        (status = 201, description = "Tenant and admin created", body = TokenSchema),
        (status = 400, description = "Bad request", body = ApiErrorResponse),
        (status = 409, description = "Tenant name already exists", body = ApiErrorResponse),
        (status = 422, description = "Validation error", body = ParamsErrorResponse),
        (status = 429, description = "Too many requests", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn register_post(
    State(state): State<AppState>,
    Valid(Json(params)): Valid<Json<RegisterParams>>,
) -> Result<impl IntoResponse, ApiError> {
    let hash = hash_password(params.password).await?;
    let admin =
        create_tenant_with_admin(&state.conn, params.tenant_name, params.username, hash).await?;

    Ok((StatusCode::CREATED, Json(sign_in(&state, admin).await?)))
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginParams,
    responses(
        (status = 200, description = "Logged in", body = TokenSchema),
        (status = 401, description = "Invalid credentials", body = ApiErrorResponse),
        (status = 422, description = "Validation error", body = ParamsErrorResponse),
        (status = 429, description = "Too many requests", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn login_post(
    State(state): State<AppState>,
    Valid(Json(params)): Valid<Json<LoginParams>>,
) -> Result<impl IntoResponse, ApiError> {
    let user = match find_tenant_by_name(&state.conn, &params.tenant_name).await? {
        Some(tenant) => {
            find_user_by_username(&state.conn, TenantId(tenant.id), &params.username).await?
        }
        None => None,
    };

    let Some(user) = user else {
        // spend the same hashing time so unknown accounts are not revealed
        hash_password(params.password).await?;
        return Err(AuthError::InvalidCredentials.into());
    };
    verify_password(params.password, user.password_hash.clone()).await?;

    Ok(Json(sign_in(&state, user).await?))
}

#[utoipa::path(
    post,
    path = "/refresh",
    request_body = RefreshParams,
    responses(
        (status = 200, description = "New access and refresh token", body = TokenSchema),
        (status = 401, description = "Unknown, expired or already used token", body = ApiErrorResponse),
        (status = 422, description = "Validation error", body = ParamsErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn refresh_post(
    State(state): State<AppState>,
    Valid(Json(params)): Valid<Json<RefreshParams>>,
) -> Result<impl IntoResponse, ApiError> {
    match rotate_session(&state.conn, &params.refresh_token).await? {
        Rotation::Rotated(user, next) => Ok(Json(token_response(&state, user, next)?)),
        Rotation::Invalid => Err(AuthError::InvalidToken.into()),
        Rotation::Reused => {
            tracing::warn!("a used refresh token was presented, session revoked");
            Err(AuthError::InvalidToken.into())
        }
    }
}

#[utoipa::path(
    post,
    path = "/logout",
    request_body = RefreshParams,
    responses(
        (status = 204, description = "Session ended; also returned for unknown tokens"),
        (status = 422, description = "Validation error", body = ParamsErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn logout_post(
    State(state): State<AppState>,
    Valid(Json(params)): Valid<Json<RefreshParams>>,
) -> Result<impl IntoResponse, ApiError> {
    end_session(&state.conn, &params.refresh_token).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Endpoints that take a password, where guessing must be limited.
pub fn create_password_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_post))
        .route("/login", post(login_post))
}

/// Endpoints that take a refresh token, which cannot be guessed.
pub fn create_session_router() -> Router<AppState> {
    Router::new()
        .route("/refresh", post(refresh_post))
        .route("/logout", post(logout_post))
}
