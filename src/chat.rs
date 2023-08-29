use crate::user::User;

pub struct Chat {
    namespace: String,
    users: Vec<User>,
}
impl Chat {
    fn new(namespace: String) -> Chat {
        Chat {
            namespace,
            users: Vec::new(),
        }
    }
    fn register_user(&mut self, user: User) {
        self.users.push(user)
    }
}

#[cfg(test)]
mod test {

    use crate::{chat::Chat, user::create_user};

    #[test]
    fn test_create_chat() {
        let chat1 = Chat::new(String::from("company1"));
        assert_eq!(chat1.namespace, "company1");
    }

    #[test]
    fn test_register_user() {
        let mut chat1 = Chat::new(String::from("company1"));
        let user1 = create_user(String::from("user1"));
        chat1.register_user(user1);
        assert_eq!(chat1.users.len(), 1);
        assert_eq!(chat1.users[0].name, "user1");
    }
}
