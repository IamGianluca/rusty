use crate::{adapters::repository::ChannelRepository, domain::channel::NewChannel};
use crate::{adapters::repository::MessageRepository, domain::channel::NewMessage};
use crate::{adapters::repository::UserRepository, domain::user::NewUser};

// what i originally created as the ChatServer, should become the
// service layer. we can use the different repositories and domain
// to construct all the functionalities we were previously offering
//
// logic sandwich:
// infla / repository
// logic / domain
// infra / repository

fn create_user<'a>(user: NewUser<'a>, repo: &mut UserRepository<'a>) -> &'a str {
    let _ = repo.save_user(&user);
    "ethuat"
}

fn create_channel<'a>(channel: NewChannel<'a>, repo: &mut ChannelRepository<'a>) -> &'a str {
    let _ = repo.save_channel(&channel);
    "usntahe"
}

fn send_message_to_channel<'a>(
    message: NewMessage<'a>,
    repo: &mut MessageRepository<'a>,
) -> &'a str {
    // infra
    let channels = repo.get_channel();

    // logic
    chennel.

    // infra
    let _ = repo.save_message(&message);
    "nuthanet"
}
