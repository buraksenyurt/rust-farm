use crate::migrator::m20220101_000001_create_user_table::User;
use crate::migrator::m20230620_173959_create_developer_table::Developer;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-user-id")
                            .from(Game::Table, Game::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-developer-id")
                            .from(Game::Table, Game::DeveloperId)
                            .to(Developer::Table, Developer::Id),
                    )
                    .col(ColumnDef::new(Game::Title).string().not_null())
                    .col(ColumnDef::new(Game::Year).string().not_null())
                    .col(ColumnDef::new(Game::Summary).string().not_null())
                    .col(
                        ColumnDef::new(Game::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Game::ModifiedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Game {
    Table,
    Id,
    UserId,
    DeveloperId,
    Title,
    Year,
    Summary,
    CreatedAt,
    ModifiedAt,
}
