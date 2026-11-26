use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use models::domains::{role::Role, tenant, user};

use crate::persistence::user::create_user;
use crate::tenant::TenantId;

/// Both rows are written in one transaction, so a failure leaves neither.
pub async fn create_tenant_with_admin(
    db: &DbConn,
    tenant_name: String,
    username: String,
    password_hash: String,
) -> Result<user::Model, DbErr> {
    let txn = db.begin().await?;

    let tenant = tenant::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(tenant_name),
    }
    .insert(&txn)
    .await?;

    let admin = create_user(
        &txn,
        TenantId(tenant.id),
        username,
        password_hash,
        Role::Admin,
    )
    .await?;

    txn.commit().await?;
    Ok(admin)
}

pub async fn find_tenant_by_name(db: &DbConn, name: &str) -> Result<Option<tenant::Model>, DbErr> {
    tenant::Entity::find()
        .filter(tenant::Column::Name.eq(name))
        .one(db)
        .await
}
