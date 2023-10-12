use crate::domain::user::NewUser;

pub fn create_test_user<'a>() -> NewUser<'a> {
    NewUser {
        username: "John Doe",
        email: "johndoe@example.com",
    }
}
