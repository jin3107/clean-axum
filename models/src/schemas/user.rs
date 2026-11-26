use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domains::{role::Role, user};

// `password_hash` is deliberately absent.
#[derive(Serialize, ToSchema)]
pub struct UserSchema {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub username: String,
    pub role: Role,
}

impl From<user::Model> for UserSchema {
    fn from(user: user::Model) -> Self {
        Self {
            id: user.id,
            tenant_id: user.tenant_id,
            username: user.username,
            role: user.role,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct UserListSchema {
    pub users: Vec<UserSchema>,
}

impl From<Vec<user::Model>> for UserListSchema {
    fn from(users: Vec<user::Model>) -> Self {
        Self {
            users: users.into_iter().map(UserSchema::from).collect(),
        }
    }
}
