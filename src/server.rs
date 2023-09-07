use std::collections::HashMap;

use crate::user::User;

pub struct ChatServer {
    workspace: String,
    channels: HashMap<String, Channel>,
    users: HashMap<String, User>,
}

impl ChatServer {
    pub fn new(workspace: String) -> ChatServer {
        println!("Creating new chat.");
        let mut chat = ChatServer {
            workspace,
            channels: HashMap::new(),
            users: HashMap::new(),
        };
        chat.create_channel("general".to_string());
        chat
    }
    pub fn create_channel(&mut self, name: String) {
        let channel = Channel::new(name);
        self.channels.insert(channel.name.to_string(), channel);
    }
    pub fn create_user(&mut self, name: String) {
        let user = User::new(name);
        println!("Creating new user.");
        self.users.insert(user.name.to_string(), user);
    }

    pub fn send_message(&mut self, user: String, channel: String, message: String) {
        let ch = self.channels.get_mut(&channel).unwrap();
        ch.add_message(user, message);
    }
}
pub struct Channel {
    name: String,
    messages: Vec<Message>,
}

impl Channel {
    pub fn new(name: String) -> Channel {
        Channel {
            name,
            messages: Vec::new(),
        }
    }
    pub fn add_message(&mut self, sender: String, text: String) {
        let msg = Message::new(sender, text);
        self.messages.push(msg);
    }
}

pub struct Message {
    sender: String,
    text: String,
}

impl Message {
    pub fn new(sender: String, text: String) -> Message {
        Message { sender, text }
    }
}

#[cfg(test)]
mod test {

    use crate::server::ChatServer;

    #[test]
    fn test_create_chat() {
        let chat1 = ChatServer::new(String::from("company1"));
        assert_eq!(chat1.workspace, "company1");
    }

    #[test]
    fn test_create_user() {
        let mut chat = ChatServer::new(String::from("company1"));
        chat.create_user("user1".to_string());
        assert_eq!(chat.users.len(), 1);
        assert_eq!(chat.users.contains_key("user1"), true);
    }

    #[test]
    fn test_create_multiple_users() {
        let mut chat = ChatServer::new(String::from("company1"));
        chat.create_user("user1".to_string());
        chat.create_user("user2".to_string());
        chat.create_user("user3".to_string());
        assert_eq!(chat.users.len(), 3);
        for user_name in ["user1", "user2", "user3"] {
            assert_eq!(chat.users.contains_key(user_name), true)
        }
    }

    #[test]
    fn test_chat_starts_with_default_channel() {
        let chat = ChatServer::new(String::from("company1"));
        assert_eq!(chat.channels.len(), 1);
        assert_eq!(chat.channels.contains_key("general"), true)
    }

    #[test]
    fn test_create_channel() {
        let mut chat = ChatServer::new(String::from("company1"));
        chat.create_channel("channel1".to_string());
        assert_eq!(chat.channels.len(), 2);
        for channel_name in ["general", "channel1"] {
            assert_eq!(chat.channels.contains_key(channel_name), true)
        }
    }

    #[test]
    fn test_send_message() {
        let mut chat = ChatServer::new("company1".to_string());
        chat.send_message(
            "user1".to_string(),
            "general".to_string(),
            "hello world!".to_string(),
        );
        let msg = &chat.channels.get("general").unwrap().messages[0];
        assert_eq!(&msg.text, "hello world!");
        assert_eq!(&msg.sender, "user1");
    }

    #[test]
    fn test_send_multiple_messages_preserve_order() {
        let mut chat = ChatServer::new("company1".to_string());
        chat.send_message(
            "user1".to_string(),
            "general".to_string(),
            "hello world!".to_string(),
        );
        chat.send_message(
            "user2".to_string(),
            "general".to_string(),
            "life is great".to_string(),
        );
        let msg = &chat.channels.get("general").unwrap().messages[0];
        assert_eq!(&msg.text, "hello world!");
        assert_eq!(&msg.sender, "user1");
        let msg = &chat.channels.get("general").unwrap().messages[1];
        assert_eq!(&msg.text, "life is great");
        assert_eq!(&msg.sender, "user2");
    }
}
