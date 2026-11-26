use axum::{extract::FromRequestParts, http::header::AUTHORIZATION, http::request::Parts};
use uuid::Uuid;

use app::error::AuthError;
use app::state::AppState;
use app::tenant::TenantId;
use models::domains::role::Role;

use crate::error::ApiError;

/// The authenticated caller. Adding it to a handler's arguments is what makes
/// the route require a valid bearer token.
pub struct AuthUser {
    pub user_id: Uuid,
    pub tenant_id: TenantId,
    pub role: Role,
}

impl AuthUser {
    pub fn require(&self, role: Role) -> Result<(), ApiError> {
        if self.role == role {
            Ok(())
        } else {
            Err(AuthError::Forbidden.into())
        }
    }
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or(AuthError::InvalidToken)?;
        let claims = state.jwt.verify(token)?;

        Ok(Self {
            user_id: claims.sub,
            tenant_id: TenantId(claims.tenant_id),
            role: claims.role,
        })
    }
}
