use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use crate::views::author::Author;
use crate::views::post::GetUserPostResponse;


#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePostSeriesResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub author: Author,
    pub posts: Vec<GetUserPostResponse>,
}
