use crate::domain::{channel::NewChannel, user::NewUser};

pub fn create_test_user<'a>() -> NewUser<'a> {
    NewUser {
        username: "John Doe",
        email: "johndoe@example.com",
    }
}

pub fn create_test_channel<'a>() -> NewChannel<'a> {
    NewChannel {
        name: "Test Channel",
        description: "Test Channel",
    }
}
