use std::collections::HashMap;

use crate::User;

#[derive(Debug)]
pub struct ChatClient {
    namespace: String,
    users: HashMap<String, User>,
}

impl ChatClient {
    fn new(namespace: String) -> ChatClient {
        ChatClient {
            namespace,
            users: HashMap::new(),
        }
    }
    fn register_user(&mut self, user: User) {
        self.users.insert(user.name.to_string(), user);
    }
}

#[cfg(test)]
mod test {

    use crate::chat::ChatClient;
    use crate::user::User;

    #[test]
    fn test_create_chat() {
        let chat1 = ChatClient::new(String::from("company1"));
        assert_eq!(chat1.namespace, "company1");
    }

    #[test]
    fn test_register_user() {
        let mut chat = ChatClient::new(String::from("company1"));
        let user1 = User::new(String::from("user1"));
        chat.register_user(user1);
        assert_eq!(chat.users.len(), 1);
        assert_eq!(chat.users.contains_key("user1"), true);
    }
    #[test]
    fn test_register_multiple_users() {
        let mut chat = ChatClient::new(String::from("company1"));
        let user1 = User::new(String::from("user1"));
        let user2 = User::new(String::from("user2"));
        let user3 = User::new(String::from("user3"));
        chat.register_user(user1);
        chat.register_user(user2);
        chat.register_user(user3);
        assert_eq!(chat.users.len(), 3);
        for user_id in ["user1", "user2", "user3"] {
            assert_eq!(chat.users.contains_key(user_id), true)
        }
    }
}
