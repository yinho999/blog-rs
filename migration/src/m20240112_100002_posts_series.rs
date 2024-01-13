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
                table_auto(PostsSeries::Table)
                    .primary_key(
                        Index::create()
                            .name("idx-posts_series-pk")
                            .table(PostsSeries::Table)
                            .col(PostsSeries::PostId)
                            .col(PostsSeries::SeriesId)
                            .borrow_mut(),
                    )
                    .col(integer(PostsSeries::PostId).borrow_mut())
                    .col(integer(PostsSeries::SeriesId).borrow_mut())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-posts_series-posts")
                            .from(PostsSeries::Table, PostsSeries::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-posts_series-series")
                            .from(PostsSeries::Table, PostsSeries::SeriesId)
                            .to(Series::Table, Series::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostsSeries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PostsSeries {
    Table,
    PostId,
    SeriesId,
    
}


#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Series {
    Table,
    Id,
}
