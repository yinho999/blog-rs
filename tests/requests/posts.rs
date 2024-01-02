use insta::{assert_debug_snapshot, with_settings};
use blog::app::App;
use loco_rs::testing;
use serial_test::serial;
use super::prepare_data;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("post_request");
        let _guard = settings.bind_to_scope();
    };
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
                    (r#"\"id\\":\d+"#, r#""id\":ID"#)
                    , (r#"\"user_id\\":\d+"#, r#""user_id\":USER_ID"#)
                ]);
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