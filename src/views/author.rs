use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::_entities::users;

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub id: uuid::Uuid,
    pub name: String,
}
impl From<users::Model> for Author {
    fn from(item: users::Model) -> Self {
        Self {
            id: item.pid,
            name: item.name,
        }
    }
}