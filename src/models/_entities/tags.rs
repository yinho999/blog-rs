//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::posts_tags::Entity")]
    PostsTags,
}

impl Related<super::posts_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostsTags.def()
    }
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        super::posts_tags::Relation::Posts.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::posts_tags::Relation::Tags.def().rev())
    }
}
