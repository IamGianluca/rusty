use actix_web::http;
use actix_web::{test, App};
use rusty::service_layer::authenticate::{create_token, decode_token, get_secret_key};
use rusty::{adapters::channel_repository::ChannelRepository, utils::create_test_user_in_db};
use serde_json::json;

fn create_and_login_user() -> (i32, String) {
    let user_id = create_test_user_in_db();
    let token = create_token(&user_id.to_string(), get_secret_key().as_bytes(), 60).unwrap();
    (user_id, token)
}

fn create_test_channel() -> i32 {
    let conn = &mut rusty::adapters::utils::get_db_conn();
    let repo = &mut rusty::adapters::channel_repository::DbChannelRepository { conn };

    rusty::service_layer::service::create_channel("test channel", "a test channel", repo);
    let channel = repo.get_channel_by_id(&1).unwrap();
    channel.id
}

#[actix_web::test]
async fn test_server_is_running() {
    // given
    let app = test::init_service(App::new().service(rusty::hello)).await;

    // when
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    // then
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_add_user_endpoint() {
    // given
    rusty::adapters::utils::rebuild_db();
    let app = test::init_service(App::new().service(rusty::create_user_endpoint)).await;

    // when
    let payload = json!({
            "username": "John Doe",
            "password": "password",
            "email": "johndoe@example.com"
    });
    let req = test::TestRequest::post()
        .uri("/user")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    // then
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_add_channel_endpoint() {
    // given
    rusty::adapters::utils::rebuild_db();
    let app = test::init_service(App::new().service(rusty::create_channel_endpoint)).await;
    let (_, token) = create_and_login_user();

    // when
    let payload = json!({
            "name": "test channel",
            "description": "test channel"
    });
    let req = test::TestRequest::post()
        .uri("/channel")
        .set_json(&payload)
        .insert_header((http::header::AUTHORIZATION, format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // then
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_add_message_endpoint() {
    // given
    rusty::adapters::utils::rebuild_db();
    let app = test::init_service(App::new().service(rusty::create_message_endpoint)).await;
    let (user_id, token) = create_and_login_user();
    let channel_id = create_test_channel();

    // when
    let payload = json!({
            "user_id": user_id.to_string(),
            "channel_id": channel_id.to_string(),
            "content": "this is a test message"
    });
    let req = test::TestRequest::post()
        .uri("/message")
        .set_json(&payload)
        .insert_header((http::header::AUTHORIZATION, format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // then
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_authenticate_user_endpoint() {
    // given
    rusty::adapters::utils::rebuild_db();
    use rusty::AppState;
    let app = test::init_service(
        App::new()
            .app_data(AppState {
                // todo: this is not really being used - fix it!
                secret_key: get_secret_key(),
            })
            .service(rusty::authenticate_user_endpoint),
    )
    .await;
    let _ = create_test_user_in_db();

    // when
    let payload = json!({
            "username": "John Doe",
            "password": "password",
    });
    let req = test::TestRequest::post()
        .uri("/authenticate")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    // then
    assert!(resp.status().is_success());
    let token_bytes = test::read_body(resp).await;
    let token_string = String::from_utf8_lossy(&*token_bytes).to_string();
    let secret_string = get_secret_key();
    let result = decode_token(token_string, secret_string.as_bytes());
    assert!(result.is_ok())
}

#[actix_web::test]
async fn test_update_credentials_endpoint_fail_no_token() {
    // given
    rusty::adapters::utils::rebuild_db();
    let app = test::init_service(App::new().service(rusty::update_credentials_endpoint)).await;
    let user_id = create_test_user_in_db();

    // when
    let payload = json!({
            "user_id": user_id.to_string(),
            "old_password": "password",
            "new_password": "new_password",
    });
    let req = test::TestRequest::put()
        .uri("/credentials")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    // then
    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_update_credentials_endpoint() {
    // given
    rusty::adapters::utils::rebuild_db();
    let app = test::init_service(App::new().service(rusty::update_credentials_endpoint)).await;
    let (user_id, token) = create_and_login_user();

    // when
    let payload = json!({
            "user_id": user_id.to_string(),
            "old_password": "password",
            "new_password": "new_password",
    });
    let req = test::TestRequest::put()
        .uri("/credentials")
        .set_json(&payload)
        .insert_header((http::header::AUTHORIZATION, format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // then
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_update_credentials_endpoint_fail_wrong_token() {
    // given
    rusty::adapters::utils::rebuild_db();
    let app = test::init_service(App::new().service(rusty::update_credentials_endpoint)).await;
    let (user_id, _) = create_and_login_user();

    // when
    let payload = json!({
            "user_id": user_id.to_string(),
            "old_password": "password",
            "new_password": "new_password",
    });
    let req = test::TestRequest::put()
        .uri("/credentials")
        .set_json(&payload)
        .insert_header((
            http::header::AUTHORIZATION,
            format!("Bearer {}", "wrong_secret"),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // then
    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}
