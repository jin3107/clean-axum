use axum::{extract::rejection::JsonRejection, http::StatusCode};
use sea_orm::{DbErr, SqlErr};

use app::error::{AuthError, UserError};

use super::traits::HTTPError;

impl HTTPError for JsonRejection {
    fn to_status_code(&self) -> StatusCode {
        match self {
            JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

impl HTTPError for DbErr {
    fn to_status_code(&self) -> StatusCode {
        match self.sql_err() {
            Some(SqlErr::UniqueConstraintViolation(_)) => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR, // TODO:: more granularity
        }
    }
}

impl HTTPError for UserError {
    fn to_status_code(&self) -> StatusCode {
        match self {
            UserError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl HTTPError for AuthError {
    fn to_status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials | AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::Forbidden => StatusCode::FORBIDDEN,
            AuthError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
