use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;


#[derive(DeriveIden)]
enum Series {
    Table,
    Title,
    Description,
}


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(Series::Table)
                .modify_column(ColumnDef::new(Series::Title).text().not_null())
                .modify_column(ColumnDef::new(Series::Description).text().not_null())
                .to_owned(),
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(Series::Table)
                .modify_column(ColumnDef::new(Series::Title).text().null())
                .modify_column(ColumnDef::new(Series::Description).text().null())
                .to_owned(),
        ).await?;

        Ok(())
    }
}

