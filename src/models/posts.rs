use loco_rs::model::{ModelError, ModelResult};
use sea_orm::entity::prelude::*;
use sea_orm::TransactionTrait;
use serde::{Deserialize, Serialize};
use crate::models::_entities::users;
use super::_entities::posts::ActiveModel;
use loco_rs::prelude::*;

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

impl super::_entities::posts::Model{
    pub async fn create(db : &DatabaseConnection, params: &PostParams, pid:Uuid) -> ModelResult<Self> {
        let txn = db.begin().await?;

        let user =  users::Entity::find()
            .filter(users::Column::Pid.eq(pid))
            .one(&txn)
            .await?;
        let user = match user {
            Some(user) => user,
            None => return Err(ModelError::EntityNotFound),
        };
        let item = params.create(user);
        let item = item.insert(&txn).await?;
        txn.commit().await?;
        Ok(item)
    }

pub async fn update(&self, db : &DatabaseConnection, params: &PostParams, user_id: i32) -> ModelResult<Self> {
        let txn = db.begin().await?;
        let mut item = self.clone().into_active_model();
        if item.user_id.as_ref() != &user_id {
            return Err(ModelError::Any(Box::from("You are not the owner of this post".to_string())));
        }
        params.update(&mut item);
        let item = item.update(&txn).await?;
        txn.commit().await?;
        Ok(item)
    }
}