use actix_web::{test, App};
use rusty::{
    adapters::{channel_repository::ChannelRepository, user_repository::UserRepository},
    utils::create_test_user_in_db,
};
use serde_json::json;

fn create_test_user() -> i32 {
    let conn = &mut rusty::adapters::utils::get_db_conn();
    let repo = &mut rusty::adapters::user_repository::DbUserRepository { conn };

    rusty::service_layer::service::create_user("John Doe", "johndoe@example.com", "password", repo);
    let users = repo.get_all().unwrap();
    users[0].id
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

    // when
    let payload = json!({
            "name": "test channel",
            "description": "test channel"
    });
    let req = test::TestRequest::post()
        .uri("/channel")
        .set_json(&payload)
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
    let user_id = create_test_user();
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
        .to_request();
    let resp = test::call_service(&app, req).await;

    // then
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_authenticate_endpoint() {
    // given
    rusty::adapters::utils::rebuild_db();
    use rusty::AppState;
    let app = test::init_service(
        App::new()
            .app_data(AppState {
                // todo: this is not really being used - fix it!
                secret_key: "your-secret-key".to_string(),
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
    use actix_web::http;
    let req = test::TestRequest::get()
        .uri("/login")
        .set_json(&payload)
        .insert_header((
            http::header::AUTHORIZATION,
            format!("Bearer {}", "your-secret-key"),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // then
    assert!(resp.status().is_success());
}
#[actix_web::test]
async fn test_authenticate_endpoint_fail_wrong_token() {
    // given
    rusty::adapters::utils::rebuild_db();
    use rusty::AppState;
    let app = test::init_service(
        App::new()
            // todo: this is not really being used - fix it!
            .app_data(AppState {
                secret_key: "secret".to_string(),
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
    use actix_web::http;
    let req = test::TestRequest::get()
        .uri("/login")
        .set_json(&payload)
        .insert_header((http::header::AUTHORIZATION, format!("Bearer {}", "secret")))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // then
    println!("{}", resp.status());
    assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn test_update_credentials_endpoint() {
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
    assert!(resp.status().is_success());
}
