use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use crate::models::_entities::{series, users};
use crate::views::author::Author;
use crate::views::post::GetUserPostResponse;


#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePostSeriesResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub author: Author,
    pub posts: Vec<GetUserPostResponse>,
}


impl CreatePostSeriesResponse {
    pub fn from_series(series: series::Model, author: users::Model, posts: Vec<GetUserPostResponse>) -> Self {
        Self {
            id: series.id,
            title: series.title,
            description: series.description,
            created_at: series.created_at,
            updated_at: series.updated_at,
            author: author.into(),
            posts,
        }
    }
}
