use crate::migrator::m20220101_000001_create_user_table::User;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Developer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Developer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Developer::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-developer-user-id")
                            .from(Developer::Table, Developer::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Developer::Fullname).string().not_null())
                    .col(ColumnDef::new(Developer::About).string().not_null())
                    .col(ColumnDef::new(Developer::Level).small_integer().not_null())
                    .col(
                        ColumnDef::new(Developer::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Developer::ModifiedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Developer::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Developer {
    Table,
    Id,
    UserId,
    Fullname,
    About,
    Level,
    CreatedAt,
    ModifiedAt,
}
