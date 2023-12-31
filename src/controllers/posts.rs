#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
use sea_orm::ConnectionTrait;
use serde::{Deserialize, Serialize};

use crate::models::_entities::posts::{ActiveModel, Entity, Model};
use crate::models::_entities::users;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub title: String,
    pub description: String,
    pub content: String,
}

impl Params {
    fn create(&self, current_user: users::Model) -> ActiveModel {
        ActiveModel {
            title: Set(self.title.clone()),
            description: Set(self.description.clone()),
            content: Set(self.content.clone()),
            user_id: Set(current_user.id),
            ..Default::default()
        }
    }
    fn update(&self, item: &mut ActiveModel, current_user: users::Model) -> Result<()>{
        if &current_user.id != item.user_id.as_ref() {
            return Err(Error::Unauthorized("You are not the owner of this post".to_string()));
        }
        item.title = Set(self.title.clone());
        item.description = Set(self.description.clone());
        item.content = Set(self.content.clone());
        Ok(())
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    format::json(Entity::find().all(&ctx.db).await?)
}

pub async fn add(auth: auth::JWT, State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Json<Model>> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = params.create(current_user);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

pub async fn update(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Json<Model>> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item,current_user)?;
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

pub async fn remove(auth:auth::JWT, Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<()> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    delete_post(current_user, id, ctx).await
}

pub async fn delete_post(current_user: users::Model, id:i32, ctx: AppContext) -> Result<()> {
    let item = load_item(&ctx, id).await?;
    if current_user.id != item.user_id {
        return Err(Error::Unauthorized("You are not the owner of this post".to_string()));
    }
    item.delete(&ctx.db).await?;
    format::empty()
}

pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Json<Model>> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("posts")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", put(update))
}
