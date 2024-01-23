use super::_entities::posts::ActiveModel;
use crate::models::_entities::{posts, users};
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::TransactionTrait;
use serde::{Deserialize, Serialize};
use std::result;
use loco_rs::model::ModelError;
use crate::models::ModelsError;


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
    #[tracing::instrument(name="Create post by user",skip(db))]
    pub async fn create(
        db: &DatabaseConnection,
        params: &PostParams,
        pid: Uuid,
    ) -> result::Result<Self, ModelsError> {
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

    #[tracing::instrument(name="Update post by user",skip(db))]
    pub async fn update(
        &self,
        db: &DatabaseConnection,
        params: &PostParams,
        pid: Uuid,
    ) -> result::Result<Self, ModelsError> {
        let txn = db.begin().await?;
        let user = users::Entity::find()
            .filter(users::Column::Pid.eq(pid))
            .one(&txn)
            .await?;
        let user = match user {
            Some(user) => user,
            None => return Err(ModelError::EntityNotFound.into()),
        };
        let mut item = self.clone().into_active_model();
        if item.user_id.as_ref() != &user.id {
            return Err(ModelsError::PermissionDenied(
                "Permission denied".to_string(),
            ));
        }
        params.update(&mut item);
        let item = item.update(&txn).await?;
        txn.commit().await?;
        Ok(item)
    }

    #[tracing::instrument(name="Get user post",skip(db))]
    pub async fn get_user(
        db: &DatabaseConnection,
        pid: Uuid,
    ) -> result::Result<Vec<Self>, ModelsError> {
        let txn = db.begin().await?;
        let user = users::Entity::find()
            .filter(users::Column::Pid.eq(pid))
            .one(&txn)
            .await?;
        let user = match user {
            Some(user) => user,
            None => return Err(ModelError::EntityNotFound.into()),
        };
        let posts = posts::Entity::find()
            .filter(posts::Column::UserId.eq(user.id))
            .all(&txn)
            .await?;
        txn.commit().await?;
        Ok(posts)
    }

    #[tracing::instrument(name="Get all post",skip(db))]
    pub async fn get_all(db: &DatabaseConnection,) -> result::Result<Vec<Self>, ModelsError> {
        let txn = db.begin().await?;
        let posts = posts::Entity::find()
            .all(&txn)
            .await?;
        txn.commit().await?;
        Ok(posts)
    }
}
