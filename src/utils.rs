use crate::{
    adapters::{
        channel_repository::{ChannelRepository, DbChannelRepository},
        user_repository::DbUserRepository,
        utils::get_db_conn,
    },
    domain::{channel::NewChannel, user::NewUser},
    service_layer::service::create_user,
};
pub fn create_test_user<'a>() -> NewUser<'a> {
    NewUser {
        username: "John Doe",
        email: "johndoe@example.com",
    }
}

pub fn create_test_user_in_db() -> i32 {
    let conn = &mut get_db_conn();
    let mut repo = DbUserRepository { conn };
    let user = create_test_user();
    let user_id = create_user(user.username, user.email, "password", &mut repo);
    // let user_id = repo.add_user(&user);
    user_id
}

pub fn create_test_channel<'a>() -> NewChannel<'a> {
    NewChannel {
        name: "Test Channel",
        description: "Test Channel",
    }
}

pub fn create_test_channel_in_db() -> i32 {
    let conn = &mut get_db_conn();
    let mut repo = DbChannelRepository { conn };
    let channel = create_test_channel();
    let channel_id = repo.add_channel(&channel);
    channel_id
}
