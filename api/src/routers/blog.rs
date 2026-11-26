use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

use app::persistence::blog::{create_blog, search_blogs};
use app::state::AppState;
use models::params::blog::CreateBlogParams;
use models::queries::blog::BlogQuery;
use models::schemas::blog::{BlogListSchema, BlogSchema};

use crate::error::ApiError;
use crate::extractor::{AuthUser, Json, Valid};
use crate::models::{ApiErrorResponse, ParamsErrorResponse};

#[utoipa::path(
    post,
    path = "",
    request_body = CreateBlogParams,
    security(("bearer" = [])),
    responses(
        (status = 201, description = "Blog created", body = BlogSchema),
        (status = 400, description = "Bad request", body = ApiErrorResponse),
        (status = 401, description = "Missing or invalid token", body = ApiErrorResponse),
        (status = 422, description = "Validation error", body = ParamsErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn blogs_post(
    State(state): State<AppState>,
    auth: AuthUser,
    Valid(Json(params)): Valid<Json<CreateBlogParams>>,
) -> Result<impl IntoResponse, ApiError> {
    let blog = create_blog(&state.conn, auth.tenant_id, auth.user_id, params).await?;

    Ok((StatusCode::CREATED, Json(BlogSchema::from(blog))))
}

#[utoipa::path(
    get,
    path = "",
    params(
        BlogQuery
    ),
    security(("bearer" = [])),
    responses(
        (status = 200, description = "List blogs of the caller's tenant", body = BlogListSchema),
        (status = 401, description = "Missing or invalid token", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
async fn blogs_get(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<BlogQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let blogs = search_blogs(&state.conn, auth.tenant_id, query).await?;
    Ok(Json(BlogListSchema::from(blogs)))
}

pub fn create_blog_router() -> Router<AppState> {
    Router::new().route("/", get(blogs_get).post(blogs_post))
}
