use actix_web::{test, App};
use serde_json::json;

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
    let app = test::init_service(App::new().service(rusty::add_user_endpoint)).await;

    // when
    let payload = json!({
            "username": "Doe John",
            "password": "password",
            "email": "johnxxxx@example.com"
    });
    let req = test::TestRequest::post()
        .uri("/user")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    // then
    assert!(resp.status().is_success());
}
