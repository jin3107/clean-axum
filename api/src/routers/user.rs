use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use uuid::Uuid;

use app::auth::hash_password;
use app::error::UserError;
use app::persistence::user::{create_user, get_user, search_users};
use app::state::AppState;
use models::domains::role::Role;
use models::params::user::CreateUserParams;
use models::queries::user::UserQuery;
use models::schemas::user::{UserListSchema, UserSchema};

use crate::error::ApiError;
use crate::extractor::{AuthUser, Json, Valid};
use crate::models::{ApiErrorResponse, ParamsErrorResponse};

#[utoipa::path(
    post,
    path = "",
    request_body = CreateUserParams,
    security(("bearer" = [])),
    responses(
        (status = 201, description = "User created", body = UserSchema),
        (status = 400, description = "Bad request", body = ApiErrorResponse),
        (status = 401, description = "Missing or invalid token", body = ApiErrorResponse),
        (status = 403, description = "Admin role required", body = ApiErrorResponse),
        (status = 409, description = "Username already exists", body = ApiErrorResponse),
        (status = 422, description = "Validation error", body = ParamsErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn users_post(
    State(state): State<AppState>,
    auth: AuthUser,
    Valid(Json(params)): Valid<Json<CreateUserParams>>,
) -> Result<impl IntoResponse, ApiError> {
    auth.require(Role::Admin)?;

    let hash = hash_password(params.password).await?;
    let user = create_user(
        &state.conn,
        auth.tenant_id,
        params.username,
        hash,
        params.role,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(UserSchema::from(user))))
}

#[utoipa::path(
    get,
    path = "",
    params(
        UserQuery
    ),
    security(("bearer" = [])),
    responses(
        (status = 200, description = "List users of the caller's tenant", body = UserListSchema),
        (status = 401, description = "Missing or invalid token", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn users_get(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<UserQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let users = search_users(&state.conn, auth.tenant_id, query).await?;
    Ok(Json(UserListSchema::from(users)))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = Uuid, Path, description = "User id")
    ),
    security(("bearer" = [])),
    responses(
        (status = 200, description = "Get user", body = UserSchema),
        (status = 401, description = "Missing or invalid token", body = ApiErrorResponse),
        (status = 404, description = "Not found", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn users_id_get(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let user = get_user(&state.conn, auth.tenant_id, id).await?;

    user.map(|user| Json(UserSchema::from(user)))
        .ok_or_else(|| UserError::NotFound.into())
}

pub fn create_user_router() -> Router<AppState> {
    Router::new()
        .route("/", post(users_post).get(users_get))
        .route("/{id}", get(users_id_get))
}
