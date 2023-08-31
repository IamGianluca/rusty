use crate::{chat::ChatClient, user::User};

mod chat;
mod user;

fn main() {
    println!("I'm a little rusty with Rust!");

    let mut chat = ChatClient::new(String::from("london llc"));
    let luca = User::new(String::from("luca"));

    chat.register_user(luca);
    // println!("Hi, my name is {0}", luca.name);
}
