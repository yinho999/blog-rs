use std::result;
use loco_rs::model::ModelError;
use sea_orm::entity::prelude::*;
use sea_orm::{Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use crate::models::_entities::{posts, series, users};
use crate::models::ModelsError;
use super::_entities::series::ActiveModel;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeriesParams {
    pub title: String,
    pub description: String,
    pub posts: Vec<i32>,
}

impl SeriesParams {
    pub fn create(&self) -> ActiveModel {
        ActiveModel {
            title: Set(self.title.clone()),
            description: Set(self.description.clone()),
            ..Default::default()
        }
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
        // find all posts that is within the posts vec
        let posts = posts::Entity::find()
            .filter(posts::Column::Id.is_in(params.posts.clone()))
            .all(&txn)
            .await?;
        let item = params.create();
        let item = item.insert(&txn).await?;
        txn.commit().await?;
        Ok(item)
    }
}
