use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Mangas::Table)
                    .col(pk_auto(Mangas::Id).borrow_mut())
                    .col(string(Mangas::Name).borrow_mut())
                    .col(string(Mangas::Path).borrow_mut())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Mangas::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Mangas {
    Table,
    Id,
    Name,
    Path,
    
}


