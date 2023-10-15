use actix_web::{test, App};

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
