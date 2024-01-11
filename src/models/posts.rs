use super::_entities::posts::ActiveModel;
use crate::models::_entities::users;
use loco_rs::errors;
use loco_rs::model::{ModelError, ModelResult};
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::TransactionTrait;
use serde::{Deserialize, Serialize};
use std::result;
#[derive(thiserror::Error, Debug)]
pub enum PostModelError {
    #[error(transparent)]
    FrameworkError(#[from] ModelError),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

impl From<DbErr> for PostModelError {
    fn from(err: DbErr) -> Self {
        Self::FrameworkError(err.into())
    }
}

impl From<PostModelError> for errors::Error {
    fn from(err: PostModelError) -> Self {
        match err {
            PostModelError::FrameworkError(err) => err.into(),
            PostModelError::PermissionDenied(msg) => Self::Unauthorized(msg),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostParams {
    pub title: String,
    pub description: String,
    pub content: String,
}

impl PostParams {
    fn create(&self, current_user: users::Model) -> ActiveModel {
        ActiveModel {
            title: Set(self.title.clone()),
            description: Set(self.description.clone()),
            content: Set(self.content.clone()),
            user_id: Set(current_user.id),
            ..Default::default()
        }
    }
    fn update(&self, item: &mut ActiveModel) {
        item.title = Set(self.title.clone());
        item.description = Set(self.description.clone());
        item.content = Set(self.content.clone());
    }
}

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl super::_entities::posts::Model {
    pub async fn create(
        db: &DatabaseConnection,
        params: &PostParams,
        pid: Uuid,
    ) -> result::Result<Self, PostModelError> {
        let txn = db.begin().await?;

        let user = users::Entity::find()
            .filter(users::Column::Pid.eq(pid))
            .one(&txn)
            .await?;
        let user = match user {
            Some(user) => user,
            None => return Err(ModelError::EntityNotFound.into()),
        };
        let item = params.create(user);
        let item = item.insert(&txn).await?;
        txn.commit().await?;
        Ok(item)
    }

    pub async fn update(
        &self,
        db: &DatabaseConnection,
        params: &PostParams,
        user_id: i32,
    ) -> result::Result<Self, PostModelError> {
        let txn = db.begin().await?;
        let mut item = self.clone().into_active_model();
        if item.user_id.as_ref() != &user_id {
            return Err(PostModelError::PermissionDenied(
                "Permission denied".to_string(),
            ));
        }
        params.update(&mut item);
        let item = item.update(&txn).await?;
        txn.commit().await?;
        Ok(item)
    }
}
