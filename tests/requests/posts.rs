use insta::{assert_debug_snapshot, with_settings};
use lazy_static::lazy_static;
use blog::app::App;
use loco_rs::testing;
use serial_test::serial;
use blog::views::post::CreatePostResponse;

use super::prepare_data;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("post_request");
        let _guard = settings.bind_to_scope();
    };
}
// Lazy-static constants for data cleanup patterns
lazy_static! {
    pub static ref CLEANUP_AUTHOR_ID: Vec<(&'static str, &'static str)> = vec![
        (r#"\\"author\\":\{\\"id\\":\\"([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})"#, r#"\"author\":\{\"id\":\"AUTHOR_ID"#)
    ];
}

#[tokio::test]
#[serial]
async fn can_create_post() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let payload = serde_json::json!({
            "title": "loco",
            "description": "loco post test description",
            "content": "loco post test",
        });

        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        with_settings!({
            filters => {
                 let mut combined_filters = testing::CLEANUP_DATE.to_vec();
                    combined_filters.extend(vec![
                    (r#"\"id\\":\d+"#, r#""id\":ID"#)]);
                    combined_filters.extend(CLEANUP_AUTHOR_ID.iter().copied());
                    combined_filters
            }
        }, {
            assert_debug_snapshot!(
            (add_post_request.status_code(), add_post_request.text())
        );
        });
    })
        .await;
}

#[tokio::test]
#[serial]
async fn cannot_create_post_without_login() {
    configure_insta!();

    testing::request::<App, _, _>(|request, _ctx| async move {
        let payload = serde_json::json!({
            "title": "loco",
            "description": "loco post test description",
            "content": "loco post test",
        });

        let add_post_request = request
            .post("/api/posts")
            .json(&payload)
            .await;

        assert_debug_snapshot!(
            (add_post_request.status_code(), add_post_request.text())
        );
    })
        .await;
}

// Get all posts if exist
#[tokio::test]
#[serial]
async fn can_get_posts() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();
        // Login user to create a post
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let payload = serde_json::json!({
            "title": "loco",
            "description": "loco post test description",
            "content": "loco post test",
        });

        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        let posts = request.get("/api/posts").await;

        with_settings!({
            filters => {
                 let mut combined_filters = testing::CLEANUP_DATE.to_vec();
                    combined_filters.extend(vec![(r#"\"id\\":\d+"#, r#""id\":ID"#)]);
                    combined_filters.extend(CLEANUP_AUTHOR_ID.iter().copied());
                    combined_filters
            }
        }, {
            assert_debug_snapshot!(
            (posts.status_code(), posts.text())
        );
        });
    })
        .await;
}

// Get no posts if not exist
#[tokio::test]
#[serial]
async fn can_get_no_posts() {
    configure_insta!();

    testing::request::<App, _, _>(|request, _ctx| async move {
        let posts = request.get("/api/posts").await;

        assert_debug_snapshot!(
            (posts.status_code(), posts.text())
        );
    })
        .await;
}

// Get one post if exist
#[tokio::test]
#[serial]
async fn can_get_one_post() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();
        // Login user to create a post
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let payload = serde_json::json!({
            "title": "loco",
            "description": "loco post test description",
            "content": "loco post test",
        });

        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        // Added post id
        let added_post = add_post_request.json::<CreatePostResponse>();
        let post = request.get(&format!("/api/posts/{}", added_post.id)).await;

        with_settings!({
            filters => {
                 let mut combined_filters = testing::CLEANUP_DATE.to_vec();
                    combined_filters.extend(vec![(r#"\"id\\":\d+"#, r#""id\":ID"#)]);
                    combined_filters.extend(CLEANUP_AUTHOR_ID.iter().copied());
                    combined_filters
            }
        }, {
            assert_debug_snapshot!(
            (post.status_code(), post.text())
        );
        });
    })
        .await;
}

// Get no post if not exist
#[tokio::test]
#[serial]
async fn can_get_no_post() {
    configure_insta!();

    testing::request::<App, _, _>(|request, _ctx| async move {
        let post = request.get("/api/posts/1").await;

        assert_debug_snapshot!(
            (post.status_code(), post.text())
        );
    })
        .await;
}

// Update post if exist and log in
#[tokio::test]
#[serial]
async fn can_update_post() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();
        // Login user to create a post
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let payload = serde_json::json!({
            "title": "loco",
            "description": "loco post test description",
            "content": "loco post test",
        });

        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        // Added post id
        let added_post = add_post_request.json::<CreatePostResponse>();
        let payload = serde_json::json!({
            "title": "loco updated",
            "description": "loco post test description updated",
            "content": "loco post test updated",
        });

        let post = request
            .put(&format!("/api/posts/{}", added_post.id))
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        with_settings!({
            filters => {
                let mut combined_filters = testing::CLEANUP_DATE.to_vec();
                combined_filters.extend(vec![(r#"\"id\\":\d+"#, r#""id\":ID"#)]);
                combined_filters.extend(CLEANUP_AUTHOR_ID.iter().copied());
                combined_filters
            }
        }, {
            assert_debug_snapshot!(
            (post.status_code(), post.text())
        );
        });
    })
        .await;
}

// Cannot update post if not login
#[tokio::test]
#[serial]
async fn cannot_update_post() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();
        // Login user to create a post
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let payload = serde_json::json!({
            "title": "loco",
            "description": "loco post test description",
            "content": "loco post test",
        });

        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        // Added post id
        let added_post = add_post_request.json::<CreatePostResponse>();
        let payload = serde_json::json!({
            "title": "loco updated",
            "description": "loco post test description updated",
            "content": "loco post test updated",
        });

        let post = request
            .put(&format!("/api/posts/{}", added_post.id))
            .json(&payload)
            .await;

        assert_debug_snapshot!(
            (post.status_code(), post.text())
        );
    })
        .await;
}

// Cannot update post if not exist
#[tokio::test]
#[serial]
async fn cannot_update_post_if_not_exist() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();
        // Login user to create a post
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let payload = serde_json::json!({
            "title": "loco",
            "description": "loco post test description",
            "content": "loco post test",
        });

        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        // Added post id
        let added_post = add_post_request.json::<CreatePostResponse>();
        let payload = serde_json::json!({
            "title": "loco updated",
            "description": "loco post test description updated",
            "content": "loco post test updated",
        });

        let post = request
            .put(&format!("/api/posts/{}", 100))
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        assert_debug_snapshot!(
            (post.status_code(), post.text())
        );
    })
        .await;
}

// Cannot update post if not owner
#[tokio::test]
#[serial]
async fn cannot_update_post_if_not_owner() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();

        // Login user to create a post
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let user2 = prepare_data::init_random_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let (auth_key2, auth_value2) = prepare_data::auth_header(&user2.token);


        let payload = serde_json::json!({
            "title": "loco",
            "description": "loco post test description",
            "content": "loco post test",
        });

        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        // Added post id
        let added_post = add_post_request.json::<CreatePostResponse>();

        let payload = serde_json::json!({
            "title": "loco updated",
            "description": "loco post test description updated",
            "content": "loco post test updated",
        });

        let post = request
            .put(&format!("/api/posts/{}", added_post.id))
            .add_header(auth_key2.clone(), auth_value2.clone())
            .json(&payload)
            .await;

        assert_debug_snapshot!(
            (post.status_code(), post.text())
        );
    })
        .await;
}

// Delete post if exist and log in
#[tokio::test]
#[serial]
async fn can_delete_post() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();
        // Login user to create a post
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let payload = serde_json::json!({
            "title": "loco",
            "description": "loco post test description",
            "content": "loco post test",
        });

        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        // Added post id
        let added_post = add_post_request.json::<CreatePostResponse>();

        let post = request
            .delete(&format!("/api/posts/{}", added_post.id))
            .add_header(auth_key.clone(), auth_value.clone())
            .await;

        assert_debug_snapshot!(
            (post.status_code(), post.text())
        );
    })
        .await;
}

// Cannot delete post if not login
#[tokio::test]
#[serial]
async fn cannot_delete_post() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();
        // Login user to create a post
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let payload = serde_json::json!({
            "title": "loco",
            "description": "loco post test description",
            "content": "loco post test",
        });

        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        // Added post id
        let added_post = add_post_request.json::<CreatePostResponse>();

        let post = request
            .delete(&format!("/api/posts/{}", added_post.id))
            .await;

        assert_debug_snapshot!(
            (post.status_code(), post.text())
        );
    })
        .await;
}

// Cannot delete post if not exist
#[tokio::test]
#[serial]
async fn cannot_delete_post_if_not_exist() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();
        // Login user to create a post
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);

        let post = request
            .delete(&format!("/api/posts/{}", 100))
            .add_header(auth_key.clone(), auth_value.clone())
            .await;

        assert_debug_snapshot!(
            (post.status_code(), post.text())
        );
    })
        .await;
}

// Cannot delete post if not owner
#[tokio::test]
#[serial]
async fn cannot_delete_post_if_not_owner() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();

        // Login user to create a post
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let user2 = prepare_data::init_random_user_login(&request, &ctx).await;


        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let (auth_key2, auth_value2) = prepare_data::auth_header(&user2.token);


        let payload = serde_json::json!({
            "title": "loco",
            "description": "loco post test description",
            "content": "loco post test",
        });

        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        // Added post id
        let added_post = add_post_request.json::<CreatePostResponse>();

        let post = request
            .delete(&format!("/api/posts/{}", added_post.id))
            .add_header(auth_key2.clone(), auth_value2.clone())
            .await;

        assert_debug_snapshot!(
            (post.status_code(), post.text())
        );
    })
        .await;
}