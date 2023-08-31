use std::io;

use crate::{chat::BackendServer, user::User};

mod chat;
mod user;

fn main() {
    println!("I'm a little rusty with Rust!");

    println!("Please input workspace name.");
    let mut workspace = String::new();
    io::stdin()
        .read_line(&mut workspace)
        .expect("Failed to read line.");
    let workspace = workspace.trim().to_string();
    let mut chat = BackendServer::new(String::from(workspace));

    println!("Please input your name.");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line.");
    let username = username.trim().to_string();
    chat.create_user(username);
}
