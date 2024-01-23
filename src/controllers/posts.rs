#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::models::_entities::posts;
use crate::models::_entities::prelude::Users;
use crate::models::_entities::users;
use crate::models::posts::PostParams;
use crate::views::post::{CreatePostResponse, GetPostResponse, GetUserPostResponse, UpdatePostResponse};
use loco_rs::prelude::*;
use uuid::Uuid;

async fn load_item(ctx: &AppContext, id: i32) -> Result<posts::Model> {
    let item = posts::Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[tracing::instrument(name = "List posts", skip(ctx))]
pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<GetPostResponse>>> {
    let items = posts::Model::get_all(&ctx.db).await?;
    let mut posts = Vec::new();
    for item in items {
        let author = users::Entity::find_by_id(item.user_id).one(&ctx.db).await?;
        if let Some(author) = author {
            let post = GetPostResponse::from_model(item, Some(author));
            posts.push(post);
        } else {
            tracing::error!("Author with id {} not found", item.user_id);
        }
    }
    format::json(posts)
}

#[tracing::instrument(name = "Create post by user", skip(ctx,auth))]
pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<PostParams>,
) -> Result<Json<CreatePostResponse>> {
    tracing::debug!("params {:?}", params);
    // pid from string to uuid
    let pid = Uuid::parse_str(&auth.claims.pid)
        .map_err(|_| Error::BadRequest("Invalid JWT".to_string()))?;
    let item = posts::Model::create(&ctx.db, &params, pid).await?;
    let author = item.find_related(Users).one(&ctx.db).await?;
    let author = author.ok_or_else(|| Error::NotFound)?;
    let item = CreatePostResponse::from_model(item, author);
    format::json(item)
}

#[tracing::instrument(name = "Update post by user", skip(ctx,auth))]
pub async fn update(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<PostParams>,
) -> Result<Json<UpdatePostResponse>> {
    // pid from string to uuid
    let pid = Uuid::parse_str(&auth.claims.pid)
        .map_err(|_| Error::BadRequest("Invalid JWT".to_string()))?;
    let item = load_item(&ctx, id).await?;
    let item = item.update(&ctx.db, &params, pid).await?;
    let author = users::Entity::find_by_id(item.user_id).one(&ctx.db).await?;
    let author = author.ok_or_else(|| Error::NotFound)?;
    let item = UpdatePostResponse::from_model(item, author);
    format::json(item)
}

#[tracing::instrument(name = "Delete post by user", skip(ctx,auth))]
pub async fn remove(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<()> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    delete_post(current_user, id, ctx).await
}

#[tracing::instrument(name = "Check if user are the same", skip(ctx))]
pub async fn delete_post(current_user: users::Model, id: i32, ctx: AppContext) -> Result<()> {
    let item = load_item(&ctx, id).await?;
    if current_user.id != item.user_id {
        return unauthorized("unauthorized!");
    }
    item.delete(&ctx.db).await?;
    format::empty()
}

#[tracing::instrument(name = "Get post by id", skip(ctx))]
pub async fn get_one(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<GetPostResponse>> {
    let post = load_item(&ctx, id).await?;
    let author = users::Entity::find_by_id(post.user_id).one(&ctx.db).await?;
    let author = author.ok_or_else(|| Error::NotFound)?;
    let post = GetPostResponse::from_model(post, Some(author));
    format::json(post)
}

#[tracing::instrument(name = "Get user owned posts", skip(ctx,auth))]
pub async fn get_user_posts(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<GetUserPostResponse>>> {
    let pid = Uuid::parse_str(&auth.claims.pid)
        .map_err(|_| Error::BadRequest("Invalid JWT".to_string()))?;
    let posts = posts::Model::get_user(&ctx.db, pid).await?;
    let mut return_posts = Vec::new();
    for post in posts {
        let post = GetUserPostResponse::from_model(post);
        return_posts.push(post);
    }
    format::json(return_posts)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("posts")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/current", get(get_user_posts))
        .add("/:id", delete(remove))
        .add("/:id", put(update))
}
