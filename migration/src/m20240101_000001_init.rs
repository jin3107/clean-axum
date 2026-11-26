use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

// Every tenant-owned table has `tenant_id` in its primary key and unique
// indexes, which Citus requires for distributed tables.
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tenant::Table)
                    .if_not_exists()
                    .col(uuid(Tenant::Id).primary_key())
                    .col(string(Tenant::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(uuid(User::TenantId))
                    .col(uuid(User::Id))
                    .col(string(User::Username))
                    .col(string(User::PasswordHash))
                    .col(string_len(User::Role, 32))
                    .primary_key(Index::create().col(User::TenantId).col(User::Id))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_tenant_id")
                            .from(User::Table, User::TenantId)
                            .to(Tenant::Table, Tenant::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("ux_user_tenant_username")
                    .table(User::Table)
                    .col(User::TenantId)
                    .col(User::Username)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tenant::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(super) enum Tenant {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub(super) enum User {
    Table,
    TenantId,
    Id,
    Username,
    PasswordHash,
    Role,
}
