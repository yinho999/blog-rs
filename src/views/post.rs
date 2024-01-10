use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::_entities::{posts, users};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetPostResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub author: Author,
}
impl GetPostResponse{
    #[must_use]
    pub fn from_model(item: posts::Model, author: users::Model) -> Self{
        Self {
            id: item.id,
            description: item.description,
            title: item.title,
            content: item.content,
            created_at: item.created_at,
            updated_at: item.updated_at,
            author:author.into()
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub id: uuid::Uuid,
    pub name: String,
}
impl From<users::Model> for Author{
    fn from(item: users::Model) -> Self {
        Self {
            id: item.pid,
            name: item.name,
        }
    }

}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePostResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub author: Author,
}

impl CreatePostResponse {
    #[must_use]
    pub fn from_model(item: posts::Model, author: users::Model) -> Self{
        Self {
            id: item.id,
            description: item.description,
            title: item.title,
            content: item.content,
            created_at: item.created_at,
            updated_at: item.updated_at,
            author:author.into()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePostResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub author: Author,
}
impl UpdatePostResponse {
    #[must_use]
    pub fn from_model(item: posts::Model, author: users::Model) -> Self{
        Self {
            id: item.id,
            description: item.description,
            title: item.title,
            content: item.content,
            created_at: item.created_at,
            updated_at: item.updated_at,
            author:author.into()
        }
    }
}