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
                table_auto(Series::Table)
                    .col(pk_auto(Series::Id).borrow_mut())
                    .col(string_null(Series::Title).borrow_mut())
                    .col(string_null(Series::Description).borrow_mut())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Series::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Series {
    Table,
    Id,
    Title,
    Description,
    
}


