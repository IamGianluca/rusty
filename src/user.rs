pub struct User {
    pub name: String,
}
impl User {
    pub fn new(name: String) -> User {
        User { name }
    }
}

#[cfg(test)]
mod tests {

    use crate::user::User;

    #[test]
    fn test_create_user() {
        let user1 = User::new(String::from("user1"));
        assert_eq!(user1.name, "user1")
    }
}
