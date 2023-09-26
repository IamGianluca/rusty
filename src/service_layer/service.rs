use crate::{adapters::repository::UserRepository, domain::user::NewUser};

// what i originally created as the ChatServer, should become the
// service layer. we can use the different repositories and domain
// to construct all the functionalities we were previously offering
//
// logic sandwich:
// infla / repository
// logic / domain
// infra / repository

fn register_user<'a>(user: NewUser<'a>, repo: &mut UserRepository<'a>) -> &'a str {
    let _ = repo.save_user(&user);
    "ethuat"
}

// fn register_channel<'a>(channel: NewUser<'a>, repo: &mut ChannelRepository<'a>) {}
