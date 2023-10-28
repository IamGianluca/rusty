use actix_web::Result;
use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use diesel::migration::MigrationMetadata;
use serde::Deserialize;
use service_layer::authenticate::get_secret_key;

pub mod adapters;
pub mod domain;
pub mod service_layer;
pub mod utils;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct UserPayload {
    username: String,
    password: String,
    email: String,
}

#[post("/user")]
async fn create_user_endpoint(info: web::Json<UserPayload>) -> impl Responder {
    let conn = &mut crate::adapters::utils::get_db_conn();
    let repo = &mut crate::adapters::user_repository::DbUserRepository { conn };
    service_layer::service::create_user(&info.username, &info.email, &info.password, repo);
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[post("/authenticate")]
async fn authenticate_user_endpoint(data: web::Json<LoginPayload>) -> HttpResponse {
    let conn = &mut crate::adapters::utils::get_db_conn();
    let repo = &mut crate::adapters::user_repository::DbUserRepository { conn };
    service_layer::authenticate::authenticate_user(&data.username, &data.password, repo);
    let token = service_layer::authenticate::create_token();
    HttpResponse::Ok().body(token)
}

#[derive(Deserialize)]
struct CredentialsPayload {
    user_id: String,
    old_password: String,
    new_password: String,
}

#[put("/credentials")]
async fn update_credentials_endpoint(
    info: web::Json<CredentialsPayload>,
    creds: BearerAuth,
) -> Result<HttpResponse> {
    let conn = &mut crate::adapters::utils::get_db_conn();
    let repo = &mut crate::adapters::user_repository::DbUserRepository { conn };
    let r = service_layer::authenticate::validate_token(&creds.token());
    if r.is_err() {
        return Ok(HttpResponse::Forbidden().finish());
    }
    service_layer::service::update_credentials(
        &info.user_id,
        &info.old_password,
        &info.new_password,
        repo,
    );
    Ok(HttpResponse::Ok().finish())
}

// #[post("/authenticate")]
// async fn authenticate_user_endpoint(
//     data: web::Json<LoginPayload>,
//     // creds: BearerAuth,
// ) -> Result<HttpResponse> {
//     let conn = &mut crate::adapters::utils::get_db_conn();
//     let repo = &mut crate::adapters::user_repository::DbUserRepository { conn };
//     // let r = service_layer::authentication::validate_token(&creds.token());
//     // if r.is_err() {
//     //     return Ok(HttpResponse::Forbidden().finish());
//     // }
//     service_layer::authentication::authenticate_user(&data.username, &data.password, repo);
//     service_layer::authentication::create_token();
//     Ok(HttpResponse::Ok().finish())
// }

#[derive(Deserialize)]
struct ChannelPayload {
    name: String,
    description: String,
}

#[post("/channel")]
async fn create_channel_endpoint(info: web::Json<ChannelPayload>) -> impl Responder {
    let conn = &mut crate::adapters::utils::get_db_conn();
    let repo = &mut crate::adapters::channel_repository::DbChannelRepository { conn };
    service_layer::service::create_channel(&info.name, &info.description, repo);
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct MessagePayload {
    user_id: String,
    channel_id: String,
    content: String,
}
#[post("/message")]
async fn create_message_endpoint(info: web::Json<MessagePayload>) -> impl Responder {
    let conn = &mut crate::adapters::utils::get_db_conn();
    let repo = &mut crate::adapters::channel_repository::DbChannelRepository { conn };

    service_layer::service::create_message(
        &info.user_id.parse::<i32>().unwrap(),
        &info.channel_id.parse::<i32>().unwrap(),
        &info.content,
        repo,
    );
    HttpResponse::Ok()
}

pub struct AppState {
    pub secret_key: String,
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(AppState {
                secret_key: get_secret_key(),
            })
            .service(hello)
            .service(create_user_endpoint)
            .service(authenticate_user_endpoint)
            .service(update_credentials_endpoint)
            .service(create_channel_endpoint)
            .service(create_message_endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
