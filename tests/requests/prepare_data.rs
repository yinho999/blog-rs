use std::convert::Into;
use axum::http::{HeaderName, HeaderValue};
use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use loco_rs::{app::AppContext, TestServer};
use blog::{models::users, views::auth::LoginResponse};
const USER_EMAIL: &str = "test@loco.com";
const USER_PASSWORD: &str = "12341234";
fn generate_user_email() -> String {
    SafeEmail().fake()
}

fn generate_user_password() -> String {
    fake::faker::internet::en::Password(8..16).fake()
}


pub struct LoggedInUser {
    pub user: users::Model,
    pub token: String,
}
pub async fn init_user_login(request: &TestServer, ctx: &AppContext) -> LoggedInUser {
    let register_payload = serde_json::json!({
        "name": "loco",
        "email": USER_EMAIL,
        "password": USER_PASSWORD
    });

    //Creating a new user
    request
        .post("/api/auth/register")
        .json(&register_payload)
        .await;
    let user = users::Model::find_by_email(&ctx.db, USER_EMAIL)
        .await
        .unwrap();

    let verify_payload = serde_json::json!({
        "token": user.email_verification_token,
    });

    request.post("/api/auth/verify").json(&verify_payload).await;

    let response = request
        .post("/api/auth/login")
        .json(&serde_json::json!({
            "email": USER_EMAIL,
            "password": USER_PASSWORD
        }))
        .await;

    let login_response: LoginResponse = serde_json::from_str(&response.text()).unwrap();

    LoggedInUser {
        user: users::Model::find_by_email(&ctx.db, USER_EMAIL)
            .await
            .unwrap(),
        token: login_response.token,
    }
}
pub async fn init_random_user_login(request: &TestServer, ctx: &AppContext) -> LoggedInUser {
    let user_email = generate_user_email();
    let user_password = generate_user_password();
    let register_payload = serde_json::json!({
        "name": "loco",
        "email": user_email,
        "password": user_password
    });

    //Creating a new user
    request
        .post("/api/auth/register")
        .json(&register_payload)
        .await;
    let user = users::Model::find_by_email(&ctx.db, &user_email)
        .await
        .unwrap();

    let verify_payload = serde_json::json!({
        "token": user.email_verification_token,
    });

    request.post("/api/auth/verify").json(&verify_payload).await;

    let response = request
        .post("/api/auth/login")
        .json(&serde_json::json!({
            "email": user_email,
            "password": user_password
        }))
        .await;

    let login_response: LoginResponse = serde_json::from_str(&response.text()).unwrap();

    LoggedInUser {
        user: users::Model::find_by_email(&ctx.db, &USER_EMAIL)
            .await
            .unwrap(),
        token: login_response.token,
    }
}

pub fn auth_header(token: &str) -> (HeaderName, HeaderValue) {
    let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap();

    (HeaderName::from_static("authorization"), auth_header_value)
}
