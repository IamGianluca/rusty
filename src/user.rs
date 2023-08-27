pub struct User {
    pub name: String,
}
pub fn create_user(name: String) -> User {
    User { name }
}

#[cfg(test)]
mod tests {

    use crate::user::create_user;

    #[test]
    fn test_create_user() {
        let user1 = create_user(String::from("user1"));
        assert_eq!(user1.name, "user1")
    }
}
