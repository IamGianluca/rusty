pub struct User {
    pub name: String,
}

#[cfg(test)]
mod tests {

    use crate::user::User;

    #[test]
    fn test_create_user() {
        let user1 = User {
            name: String::from("user1"),
        };
        assert_eq!(user1.name, "user1")
    }
}
