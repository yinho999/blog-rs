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
                table_auto(PostsTags::Table)
                    .primary_key(
                        Index::create()
                            .name("idx-refs-pk")
                            .table(PostsTags::Table)
                            .col(PostsTags::PostId)
                            .col(PostsTags::TagId)
                            .borrow_mut(),
                    )
                    .col(integer(PostsTags::PostId).borrow_mut())
                    .col(integer(PostsTags::TagId).borrow_mut())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-posts_tags-posts")
                            .from(PostsTags::Table, PostsTags::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-posts_tags-tags")
                            .from(PostsTags::Table, PostsTags::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostsTags::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PostsTags {
    Table,
    PostId,
    TagId,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Tags {
    Table,
    Id,
}
