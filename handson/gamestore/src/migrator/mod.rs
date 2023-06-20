pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20230620_173959_create_developer_table;
mod m20230620_175011_create_game_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20230620_173959_create_developer_table::Migration),
            Box::new(m20230620_175011_create_game_table::Migration),
        ]
    }
}
