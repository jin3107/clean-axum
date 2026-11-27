use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use models::domains::{role::Role, user};
use models::queries::user::UserQuery;

use crate::tenant::TenantId;

pub async fn create_user<C: ConnectionTrait>(
    db: &C,
    tenant: TenantId,
    username: String,
    password_hash: String,
    role: Role,
) -> Result<user::Model, DbErr> {
    user::ActiveModel {
        tenant_id: Set(tenant.0),
        id: Set(Uuid::new_v4()),
        username: Set(username),
        password_hash: Set(password_hash),
        role: Set(role),
    }
    .insert(db)
    .await
}

pub async fn search_users(
    db: &DbConn,
    tenant: TenantId,
    query: UserQuery,
) -> Result<Vec<user::Model>, DbErr> {
    user::Entity::find()
        .filter(user::Column::TenantId.eq(tenant.0))
        .filter(user::Column::Username.contains(query.username.unwrap_or_default()))
        .all(db)
        .await
}

pub async fn get_user<C: ConnectionTrait>(
    db: &C,
    tenant: TenantId,
    id: Uuid,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id((tenant.0, id)).one(db).await
}

pub async fn find_user_by_username(
    db: &DbConn,
    tenant: TenantId,
    username: &str,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
        .filter(user::Column::TenantId.eq(tenant.0))
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
}
