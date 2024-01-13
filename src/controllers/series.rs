#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
use crate::models::_entities::series;
use crate::models::series::SeriesParams;

async fn load_item(ctx: &AppContext, id: i32) -> Result<series::Model> {
    let item = series::Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<series::Model>>> {
    format::json(series::Entity::find().all(&ctx.db).await?)
}

pub async fn add(auth: auth::JWT, State(ctx): State<AppContext>, Json(params): Json<SeriesParams>) -> Result<Json<series::Model>> {
    let mut item = series::ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

pub async fn update(
    auth: auth::JWT, Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<SeriesParams>,
) -> Result<Json<series::Model>> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

pub async fn remove(auth: auth::JWT, Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<()> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Json<series::Model>> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("series")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
}