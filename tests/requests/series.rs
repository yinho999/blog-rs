use axum::http::StatusCode;
use insta::{assert_debug_snapshot, with_settings};
use lazy_static::lazy_static;
use blog::app::App;
use loco_rs::testing;
use serial_test::serial;
use blog::views::post::GetPostResponse;
use crate::requests::prepare_data;
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
    pub static ref CLEANUP_AUTHOR_ID: Vec<(&'static str, &'static str)> = vec![(
        r#"\\"author\\":\{\\"id\\":\\"([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})"#,
        r#"\"author\":\{\"id\":\"AUTHOR_ID"#
    )];
}
#[tokio::test]
#[serial]
async fn can_create_series() {
    configure_insta!();
    testing::request::<App, _, _>(|request, ctx| async move {
        // Create a user
        let user = prepare_data::init_user_login(&request, &ctx).await;

        // Create auth header
        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);

        // Create a post 1 payload
        let payload = serde_json::json!({
            "title": "loco 1",
            "description": "loco post test description 1",
            "content": "loco post test 1",
        });

        // Create a post 2 payload
        let payload2 = serde_json::json!({
            "title": "loco 2",
            "description": "loco post test description 2",
            "content": "loco post test 2",
        });

        // Create a post 1 for the user
        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        // Create a post 2 for the user
        let add_post_request2 = request
            .post("/api/posts")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload2)
            .await;

        // Get the post 1 id
        let post_1 = add_post_request.json::<GetPostResponse>();
        let post_1_id = post_1.id;

        // Get the post 2 id
        let post_2 = add_post_request2.json::<GetPostResponse>();
        let post_2_id = post_2.id;

        // Create a series payload
        let payload = serde_json::json!({
            "title": "loco series",
            "description": "loco series test description",
            "posts": [
                post_1_id,
                post_2_id
            ]
        });

        // Create a series for the user
        let add_series_request = request
            .post("/api/series")
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
        },{
            assert_debug_snapshot!((add_series_request.status_code(), add_series_request.text()));
        });
    })
        .await;
}

#[tokio::test]
#[serial]
async fn cannot_create_series_with_posts_of_other_owner(){
    configure_insta!();
    testing::request::<App, _, _>(|request, ctx| async move {
        // Create a user
        let user = prepare_data::init_user_login(&request, &ctx).await;

        // Create auth header
        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);

        // Create a user 2
        let user2 = prepare_data::init_random_user_login(&request, &ctx).await;

        // Create auth header
        let (auth_key2, auth_value2) = prepare_data::auth_header(&user2.token);

        // Create a post 1 payload
        let payload = serde_json::json!({
            "title": "loco 1",
            "description": "loco post test description 1",
            "content": "loco post test 1",
        });

        // Create a post 2 payload
        let payload2 = serde_json::json!({
            "title": "loco 2",
            "description": "loco post test description 2",
            "content": "loco post test 2",
        });

        // Create a post 1 for the user
        let add_post_request = request
            .post("/api/posts")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        // Create a post 2 for the user
        let add_post_request2 = request
            .post("/api/posts")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload2)
            .await;

        // Get the post 1 id
        let post_1 = add_post_request.json::<GetPostResponse>();
        let post_1_id = post_1.id;

        // Get the post 2 id
        let post_2 = add_post_request2.json::<GetPostResponse>();
        let post_2_id = post_2.id;

        // Create a series payload
        let payload = serde_json::json!({
            "title": "loco series",
            "description": "loco series test description",
            "posts": [
                post_1_id,
                post_2_id
            ]
        });

        // Create a series for the user2
        let add_series_request = request
            .post("/api/series")
            .add_header(auth_key2.clone(), auth_value2.clone())
            .json(&payload)
            .await;

        with_settings!({
            filters => {
                let mut combined_filters = testing::CLEANUP_DATE.to_vec();
                combined_filters.extend(vec![(r#"\"id\\":\d+"#, r#""id\":ID"#)]);
                combined_filters.extend(CLEANUP_AUTHOR_ID.iter().copied());
                combined_filters
            }
        },{
            assert_debug_snapshot!((add_series_request.status_code(), add_series_request.text()));
        });

    })
        .await;
}