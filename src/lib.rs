use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

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
    let conn = &mut crate::adapters::utils::get_database_connection();
    let repo = &mut crate::adapters::user_repository::DbUserRepository { conn };
    service_layer::service::create_user(&info.username, &info.email, &info.password, repo);
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct ChannelPayload {
    name: String,
    description: String,
}
#[post("/channel")]
async fn create_channel_endpoint(info: web::Json<ChannelPayload>) -> impl Responder {
    let conn = &mut crate::adapters::utils::get_database_connection();
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
    let conn = &mut crate::adapters::utils::get_database_connection();
    let repo = &mut crate::adapters::channel_repository::DbChannelRepository { conn };

    service_layer::service::create_message(
        &info.user_id.parse::<i32>().unwrap(),
        &info.channel_id.parse::<i32>().unwrap(),
        &info.content,
        repo,
    );
    HttpResponse::Ok()
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(create_user_endpoint)
            .service(create_channel_endpoint)
            .service(create_message_endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
