mod chat;
mod user;

fn main() {
    println!("I'm a little rusty with Rust!");

    let luca = user::create_user(String::from("luca"));
    println!("Hi, my name is {0}", luca.name);
}
