use std::result;
use loco_rs::model::ModelError;
use sea_orm::entity::prelude::*;
use sea_orm::{JoinType, QuerySelect, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use crate::models::_entities::{posts, posts_series, series, users};
use crate::models::ModelsError;
use super::_entities::series::ActiveModel;
use futures::future::try_join_all;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeriesParams {
    pub title: String,
    pub description: String,
    pub posts: Vec<i32>,
}

impl SeriesParams {
    pub fn create_series(&self, user_id: i32) -> series::ActiveModel {
        ActiveModel {
            title: Set(self.title.clone()),
            description: Set(self.description.clone()),
            user_id: Set(user_id),
            ..Default::default()
        }
    }
    pub fn create_posts_series(&self, series_id: i32) -> Vec<posts_series::ActiveModel> {
        self.posts.iter().map(|post_id| {
            posts_series::ActiveModel {
                post_id: Set(*post_id),
                series_id: Set(series_id),
                ..Default::default()
            }
        }).collect()
    }
    pub fn update(&self, item: &mut series::ActiveModel) {
        item.title = Set(self.title.clone());
        item.description = Set(self.description.clone());
    }
}

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl super::_entities::series::Model {
    pub async fn get_related_author(&self, db: &DatabaseConnection) -> result::Result<users::Model, ModelsError> {
        let txn = db.begin().await?;
        let author = users::Entity::find_by_id(self.user_id)
            .one(&txn)
            .await?;
        let author = match author {
            Some(author) => author,
            None => return Err(ModelError::EntityNotFound.into()),
        };
        txn.commit().await?;
        Ok(author)
    }
    pub async fn get_related_posts(&self, db: &DatabaseConnection) -> result::Result<Vec<posts::Model>, ModelsError> {
        let txn = db.begin().await?;
        // Perform a single query to join `posts_series` with `posts` and select related posts.
        let related_posts = posts_series::Entity::find()
            .filter(posts_series::Column::SeriesId.eq(self.id))
            .join(
                JoinType::InnerJoin,
                posts_series::Relation::Posts.def(),
            )
            .select_only()
            .column_as(posts::Column::Id, "id")
            .column_as(posts::Column::Title, "title")
            .column_as(posts::Column::Description, "description")
            .column_as(posts::Column::Content, "content")
            .column_as(posts::Column::UserId, "user_id")
            .column_as(posts::Column::CreatedAt, "created_at")
            .column_as(posts::Column::UpdatedAt, "updated_at")
            .into_model::<posts::Model>()
            .all(&txn)
            .await?;

        txn.commit().await?;
        Ok(related_posts)
    }
    pub async fn create(
        db: &DatabaseConnection,
        params: &SeriesParams,
        pid: Uuid,
    ) -> result::Result<Self, ModelsError> {
        let txn = db.begin().await?;
        // Get the user
        let user = users::Entity::find()
            .filter(users::Column::Pid.eq(pid))
            .one(&txn)
            .await?;
        let user = match user {
            Some(user) => user,
            None => return Err(ModelError::EntityNotFound.into()),
        };
        // find all posts that is owned by the user
        let posts = posts::Entity::find()
            .filter(posts::Column::UserId.eq(user.id))
            .all(&txn)
            .await?;
        // Check if post ids in params are owned by the user
        for post_id in &params.posts {
            if !posts.iter().any(|post| post.id == *post_id) {
                tracing::error!("Post with id {} not found", post_id);
                return Err(ModelError::EntityNotFound.into());
            }
        }
        let series = params.create_series(user.id);
        let series = series.insert(&txn).await.map_err(|err| {
            tracing::error!("Error creating series: {:?}", err);
            err
        })?;
        // Create posts_series and insert them concurrently into the database
        let posts_series = params.create_posts_series(series.id);
        try_join_all(posts_series.into_iter().map(|post_series| {
            post_series.insert(&txn)
        })).await.map_err(|err| {
            tracing::error!("Error creating posts_series: {:?}", err);
            err
        })?;

        // Get the series with posts
        let series = series::Entity::find_by_id(series.id).one(&txn).await?;
        let series = series.ok_or(ModelError::EntityNotFound)?;
        txn.commit().await?;
        Ok(series)
    }
}
