use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

pub mod adapters;
pub mod client;
pub mod domain;
pub mod service_layer;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct UserPayload {
    username: String,
    email: String,
}

#[post("/user")]
async fn user(info: web::Json<UserPayload>) -> impl Responder {
    let user = crate::domain::user::NewUser {
        username: &info.username,
        email: &info.email,
    };

    let conn = &mut crate::adapters::utils::get_database_connection();
    let repo = &mut crate::adapters::user_repository::DbUserRepository { conn };
    service_layer::service::create_user(user, repo);

    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct MessagePayload {
    user_id: String,
    channel_id: String,
    content: String,
}
#[post("/message")]
async fn message(info: web::Json<MessagePayload>) -> impl Responder {
    let message = crate::domain::message::NewMessage {
        channel_id: &info.channel_id.parse::<i32>().unwrap(),
        user_id: &info.user_id.parse::<i32>().unwrap(),
        content: &info.content,
    };

    let conn = &mut crate::adapters::utils::get_database_connection();
    let repo = &mut crate::adapters::message_repository::DbMessageRepository { conn };
    service_layer::service::send_message(message, repo);

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(user).service(message))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
