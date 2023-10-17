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
async fn add_user_endpoint(info: web::Json<UserPayload>) -> impl Responder {
    let conn = &mut crate::adapters::utils::get_database_connection();
    let repo = &mut crate::adapters::user_repository::DbUserRepository { conn };

    let user = crate::domain::user::NewUser {
        username: &info.username,
        email: &info.email,
    };
    let password = &info.password;
    service_layer::service::create_user(user, password, repo);

    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct ChannelPayload {
    name: String,
    description: String,
}
#[post("/channel")]
async fn add_channel_endpoint(info: web::Json<ChannelPayload>) -> impl Responder {
    let conn = &mut crate::adapters::utils::get_database_connection();
    let repo = &mut crate::adapters::message_repository::DbMessageRepository { conn };

    let channel = crate::domain::channel::NewChannel {
        name: &info.name,
        description: &info.description,
    };
    service_layer::service::create_channel(channel, repo);

    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct MessagePayload {
    user_id: String,
    channel_id: String,
    content: String,
}
#[post("/message")]
async fn add_message_endpoint(info: web::Json<MessagePayload>) -> impl Responder {
    let conn = &mut crate::adapters::utils::get_database_connection();
    let repo = &mut crate::adapters::message_repository::DbMessageRepository { conn };

    let message = crate::domain::message::NewMessage {
        channel_id: &info.channel_id.parse::<i32>().unwrap(),
        user_id: &info.user_id.parse::<i32>().unwrap(),
        content: &info.content,
    };
    service_layer::service::send_message(message, repo);

    HttpResponse::Ok()
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(add_user_endpoint)
            .service(add_channel_endpoint)
            .service(add_message_endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
