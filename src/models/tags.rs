use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use super::_entities::tags::{ActiveModel};
use loco_rs::{validation, validator::Validate};
use serde::Deserialize;
use crate::models::_entities::{tags, users};

#[derive(Debug, Validate, Deserialize)]
struct ModelValidator {
    // add fields here
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    name: String,
    #[validate(length(min = 2, message = "Description must be at least 2 characters long."))]
    description: String,
}

impl From<&ActiveModel> for ModelValidator {
    fn from(value: &ActiveModel) -> Self {
        Self {
            name: value.name.as_ref().to_string(),
            description: value.description.as_ref().to_string(),
        }
    }
}

impl ActiveModel {
    async fn validate<C>(&self, db: &C) -> Result<(), DbErr> where C: ConnectionTrait, {
        let validator = ModelValidator::from(self);
        validator.validate().map_err(|e| validation::into_db_error(&e))?;
        let name = validator.name.clone();
        if tags::Entity::find()
            .filter(tags::Column::Name.eq(name))
            .one(db)
            .await?
            .is_some()
        {
            return Err(DbErr::Custom("Name already exists".to_string()));
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
        where
            C: ConnectionTrait,
    {
        {
            self.validate(db).await?;
            if insert {
                let mut this = self;
                Ok(this)
            } else {
                Ok(self)
            }
        }
    }
}

struct CreateParams {
    name: String,
    description: String,
}

impl super::_entities::tags::Model {
    // pub async fn create_with_pid<C>(db: &C, params: &CreateParams, user_pid: &str) -> Result<Self, DbErr>
    //     where
    //         C: ConnectionTrait,
    // {
    //     let user = users::Model::find_by_pid(db, &user_pid).await?;
    //
    //
    // }
}