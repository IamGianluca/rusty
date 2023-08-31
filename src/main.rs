use std::io;

use crate::{chat::ChatClient, user::User};

mod chat;
mod user;

fn main() {
    println!("I'm a little rusty with Rust!");

    let mut chat = ChatClient::new(String::from("london llc"));

    println!("Please input your name.");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line.");
    let username = username.trim().to_string();

    let luca = User::new(username);
    chat.register_user(luca);
}
