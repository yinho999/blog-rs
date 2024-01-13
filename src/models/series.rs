use std::result;
use sea_orm::entity::prelude::*;
use sea_orm::{Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use crate::models::_entities::series;
use crate::models::ModelsError;
use super::_entities::series::ActiveModel;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeriesParams {
    pub title: String,
    pub description: String,
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

    ) -> result::Result<Self, ModelsError> {
        let txn = db.begin().await?;
        let item = params.create();
        let item = item.insert(&txn).await?;
        txn.commit().await?;
        Ok(item)
    }
}
