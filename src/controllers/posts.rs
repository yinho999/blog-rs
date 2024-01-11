#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::models::_entities::posts::{Entity, Model};
use crate::models::_entities::prelude::Users;
use crate::models::_entities::users;
use crate::models::posts::PostParams;
use crate::views::post::{CreatePostResponse, GetPostResponse, UpdatePostResponse};
use loco_rs::prelude::*;
use uuid::Uuid;

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<GetPostResponse>>> {
    let items = Entity::find().all(&ctx.db).await?;
    let mut posts = Vec::new();
    for item in items {
        let author = users::Entity::find_by_id(item.user_id).one(&ctx.db).await?;
        if let Some(author) = author {
            let post = GetPostResponse::from_model(item, author);
            posts.push(post);
        } else {
            tracing::error!("Author with id {} not found", item.user_id);
        }
    }
    format::json(posts)
}

pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<PostParams>,
) -> Result<Json<CreatePostResponse>> {
    tracing::debug!("params {:?}", params);
    // pid from string to uuid
    let pid = Uuid::parse_str(&auth.claims.pid)
        .map_err(|_| Error::BadRequest("Invalid JWT".to_string()))?;
    let item = Model::create(&ctx.db, &params, pid).await?;
    let author = item.find_related(Users).one(&ctx.db).await?;
    let author = author.ok_or_else(|| Error::NotFound)?;
    let item = CreatePostResponse::from_model(item, author);
    format::json(item)
}

pub async fn update(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<PostParams>,
) -> Result<Json<UpdatePostResponse>> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id).await?;
    let item = item.update(&ctx.db, &params, current_user.id).await?;
    let author = users::Entity::find_by_id(item.user_id).one(&ctx.db).await?;
    let author = author.ok_or_else(|| Error::NotFound)?;
    let item = UpdatePostResponse::from_model(item, author);
    format::json(item)
}

pub async fn remove(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<()> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    delete_post(current_user, id, ctx).await
}

pub async fn delete_post(current_user: users::Model, id: i32, ctx: AppContext) -> Result<()> {
    let item = load_item(&ctx, id).await?;
    if current_user.id != item.user_id {
        return unauthorized("unauthorized!");
    }
    item.delete(&ctx.db).await?;
    format::empty()
}

pub async fn get_one(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<GetPostResponse>> {
    let post = load_item(&ctx, id).await?;
    let author = users::Entity::find_by_id(post.user_id).one(&ctx.db).await?;
    let author = author.ok_or_else(|| Error::NotFound)?;
    let post = GetPostResponse::from_model(post, author);
    format::json(post)
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
