use std::collections::HashMap;

use crate::User;

pub struct BackendServer {
    workspace: String,
    users: HashMap<String, User>,
}

impl BackendServer {
    pub fn new(workspace: String) -> BackendServer {
        println!("Creating new chat.");
        BackendServer {
            workspace,
            users: HashMap::new(),
        }
    }
    pub fn create_user(&mut self, name: String) {
        let user = User::new(name);
        println!("Creating new user.");
        self.users.insert(user.name.to_string(), user);
    }
}

#[cfg(test)]
mod test {

    use crate::chat::BackendServer;

    #[test]
    fn test_create_chat() {
        let chat1 = BackendServer::new(String::from("company1"));
        assert_eq!(chat1.workspace, "company1");
    }

    #[test]
    fn test_create_user() {
        let mut chat = BackendServer::new(String::from("company1"));
        chat.create_user("user1".to_string());
        assert_eq!(chat.users.len(), 1);
        assert_eq!(chat.users.contains_key("user1"), true);
    }

    #[test]
    fn test_create_multiple_users() {
        let mut chat = BackendServer::new(String::from("company1"));
        chat.create_user("user1".to_string());
        chat.create_user("user2".to_string());
        chat.create_user("user3".to_string());
        assert_eq!(chat.users.len(), 3);
        for user_id in ["user1", "user2", "user3"] {
            assert_eq!(chat.users.contains_key(user_id), true)
        }
    }
}
