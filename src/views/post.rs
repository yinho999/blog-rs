use crate::models::_entities::{posts, users};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::users::Model;
use crate::views::author::Author;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetPostResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub author: Option<Author>,
}
impl GetPostResponse {
    #[must_use]
    pub fn from_model(item: posts::Model, author: Option<users::Model>) -> Self {
        match author{
            None => {
                Self {
                    id: item.id,
                    description: item.description,
                    title: item.title,
                    content: item.content,
                    created_at: item.created_at,
                    updated_at: item.updated_at,
                    author: None,
                }
            }
            Some(author) => {
                Self {
                    id: item.id,
                    description: item.description,
                    title: item.title,
                    content: item.content,
                    created_at: item.created_at,
                    updated_at: item.updated_at,
                    author: Some(author.into()),
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserPostResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
impl GetUserPostResponse {
    #[must_use]
    pub fn from_model(item: posts::Model) -> Self {
        Self {
            id: item.id,
            description: item.description,
            title: item.title,
            content: item.content,
            created_at: item.created_at,
            updated_at: item.updated_at,
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
    pub fn from_model(item: posts::Model, author: users::Model) -> Self {
        Self {
            id: item.id,
            description: item.description,
            title: item.title,
            content: item.content,
            created_at: item.created_at,
            updated_at: item.updated_at,
            author: author.into(),
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
    pub fn from_model(item: posts::Model, author: users::Model) -> Self {
        Self {
            id: item.id,
            description: item.description,
            title: item.title,
            content: item.content,
            created_at: item.created_at,
            updated_at: item.updated_at,
            author: author.into(),
        }
    }
}
