use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use models::domains::refresh_token;

use crate::tenant::TenantId;

pub async fn insert_token<C: ConnectionTrait>(
    db: &C,
    tenant: TenantId,
    user_id: Uuid,
    family_id: Uuid,
    token_hash: String,
    expires_at: i64,
) -> Result<(), DbErr> {
    refresh_token::ActiveModel {
        tenant_id: Set(tenant.0),
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        family_id: Set(family_id),
        token_hash: Set(token_hash),
        expires_at: Set(expires_at),
        revoked_at: Set(None),
    }
    .insert(db)
    .await?;
    Ok(())
}

pub async fn find_by_hash<C: ConnectionTrait>(
    db: &C,
    tenant: TenantId,
    token_hash: &str,
) -> Result<Option<refresh_token::Model>, DbErr> {
    refresh_token::Entity::find()
        .filter(refresh_token::Column::TenantId.eq(tenant.0))
        .filter(refresh_token::Column::TokenHash.eq(token_hash))
        .one(db)
        .await
}

/// Marks one live token as used. `false` means someone else got there first.
pub async fn revoke_token<C: ConnectionTrait>(
    db: &C,
    tenant: TenantId,
    id: Uuid,
    now: i64,
) -> Result<bool, DbErr> {
    let result = refresh_token::Entity::update_many()
        .col_expr(refresh_token::Column::RevokedAt, Expr::value(now))
        .filter(refresh_token::Column::TenantId.eq(tenant.0))
        .filter(refresh_token::Column::Id.eq(id))
        .filter(refresh_token::Column::RevokedAt.is_null())
        .exec(db)
        .await?;
    Ok(result.rows_affected == 1)
}

pub async fn revoke_family<C: ConnectionTrait>(
    db: &C,
    tenant: TenantId,
    family_id: Uuid,
    now: i64,
) -> Result<(), DbErr> {
    refresh_token::Entity::update_many()
        .col_expr(refresh_token::Column::RevokedAt, Expr::value(now))
        .filter(refresh_token::Column::TenantId.eq(tenant.0))
        .filter(refresh_token::Column::FamilyId.eq(family_id))
        .filter(refresh_token::Column::RevokedAt.is_null())
        .exec(db)
        .await?;
    Ok(())
}

pub async fn delete_expired<C: ConnectionTrait>(
    db: &C,
    tenant: TenantId,
    user_id: Uuid,
    now: i64,
) -> Result<(), DbErr> {
    refresh_token::Entity::delete_many()
        .filter(refresh_token::Column::TenantId.eq(tenant.0))
        .filter(refresh_token::Column::UserId.eq(user_id))
        .filter(refresh_token::Column::ExpiresAt.lte(now))
        .exec(db)
        .await?;
    Ok(())
}
