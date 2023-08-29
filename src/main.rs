use crate::user::User;

mod chat;
mod user;

fn main() {
    println!("I'm a little rusty with Rust!");

    let luca = User::new(String::from("luca"));
    println!("Hi, my name is {0}", luca.name);
}
