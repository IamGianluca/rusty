use crate::domain::channel::Channel;
use std::collections::HashMap;

use super::user::NewUser;
// what are the competenses of the server class?
// perhaps it is to check if someone is authorized to access a channel? or
// what else????? do we even need this class?
//
//
//
// pub struct ChatServer<'a> {
//     workspace: String,
//     channels: HashMap<String, Channel>,
//     users: HashMap<String, NewUser<'a>>,
// }
//
// impl<'a> ChatServer<'a> {
//     pub fn new(workspace: String) -> ChatServer<'a> {
//         println!("Creating new chat.");
//         let mut chat = ChatServer {
//             workspace,
//             channels: HashMap::new(),
//             users: HashMap::new(),
//         };
//         match chat.create_channel("general".to_string()) {
//             Ok(_) => chat,
//             Err(err) => {
//                 println!("{}", err);
//                 chat
//             }
//         }
//     }

// // todo: we should pass a NewChannel to this method, not a string
// // the service layer will create the NewChannel obj for us
// pub fn create_channel(&mut self, name: String) -> Result<&str, &str> {
//     if self.channels.contains_key(&name) {
//         Err("Channel {channel} already exists.")
//     } else {
//         let channel = Channel::new(name);
//         self.channels.insert(channel.name.to_string(), channel);
//         Ok("Created channel {channel}")
//     }
// }
//
// pub fn create_user(&mut self, user: NewUser<'a>) {
//     self.users.insert(user.username.to_string(), user);
// }

// pub fn send_message(&mut self, user: String, channel: String, message: String) {
//     let ch = self.channels.get_mut(&channel).unwrap();
//     ch.add_message(user, message);
// }
// }

// pub fn register_new_user_to_chat_server<'a>(server: &mut ChatServer<'a>, user: NewUser<'a>) {
//     server.create_user(user)
// }

#[cfg(test)]
mod test {
    use crate::domain::{server::ChatServer, user::NewUser};

    fn create_user_from_username(name: &str) -> NewUser {
        NewUser {
            username: name,
            email: "{name}@example.com",
        }
    }
    #[test]
    fn test_create_chat() {
        let chat = ChatServer::new(String::from("company1"));
        assert_eq!(chat.workspace, "company1");
    }

    #[test]
    fn test_create_user() {
        let mut chat = ChatServer::new(String::from("company1"));
        chat.create_user(create_user_from_username("user1"));
        assert_eq!(chat.users.len(), 1);
        assert!(chat.users.contains_key("user1"));
    }

    #[test]
    fn test_create_multiple_users() {
        let mut chat = ChatServer::new(String::from("company1"));
        chat.create_user(create_user_from_username("user1"));
        chat.create_user(create_user_from_username("user2"));
        chat.create_user(create_user_from_username("user3"));
        assert_eq!(chat.users.len(), 3);
        for user_name in ["user1", "user2", "user3"] {
            assert!(chat.users.contains_key(user_name))
        }
    }

    #[test]
    fn test_chat_starts_with_default_channel() {
        let chat = ChatServer::new(String::from("company1"));
        assert_eq!(chat.channels.len(), 1);
        assert!(chat.channels.contains_key("general"))
    }

    #[test]
    fn test_create_channel() {
        let mut chat = ChatServer::new(String::from("company1"));
        let _ = chat.create_channel("channel1".to_string());
        assert_eq!(chat.channels.len(), 2);
        for channel_name in ["general", "channel1"] {
            assert!(chat.channels.contains_key(channel_name))
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

    #[test]
    fn test_create_channel_with_existing_name() {
        let mut chat = ChatServer::new("company1".to_string());
        let _ = chat.create_channel("new".to_string());
        let result = chat.create_channel("new".to_string());
        assert!(result.is_err())
    }
}
