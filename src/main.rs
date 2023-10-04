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
struct Info {
    username: String,
    email: String,
}

#[post("/user")]
async fn user(info: web::Json<Info>) -> impl Responder {
    let conn = &mut crate::adapters::utils::get_database_connection();
    let repo = &mut crate::adapters::user_repository::DbUserRepository { connection: conn };

    let user = crate::domain::user::NewUser {
        username: &info.username,
        email: &info.email,
    };
    service_layer::service::create_user(user, repo);
    HttpResponse::Ok()
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(user)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
