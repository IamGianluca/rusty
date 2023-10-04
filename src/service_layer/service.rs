use crate::{
    adapters::{message_repository::MessageRepository, user_repository::UserRepository},
    domain::{message::NewMessage, user::NewUser},
};

fn create_user(user: NewUser, repo: &mut dyn UserRepository) {
    let _ = repo.save_user(&user);
}

fn send_message(message: NewMessage, repo: &mut dyn MessageRepository) {
    let _ = repo.save_message(&message);
}

#[cfg(test)]
mod test {
    use crate::adapters::message_repository::{DbMessageRepository, MessageRepository};
    use crate::adapters::user_repository::{DbUserRepository, UserRepository};
    use crate::adapters::utils::get_database_connection;
    use crate::domain::channel::NewChannel;
    use crate::domain::message::NewMessage;
    use crate::domain::user::NewUser;
    use crate::service_layer::service::{create_user, send_message};

    #[test]
    fn test_service_create_user() {
        // given
        let conn = &mut get_database_connection();
        let mut repo = DbUserRepository { connection: conn };

        let user = NewUser {
            username: &"John Doe".to_string(),
            email: &"johndoe@example.com".to_string(),
        };

        // then: no user in the db, empty vector
        let result = repo.get_all();
        assert_eq!(result.unwrap().len(), 0);

        // when
        create_user(user, &mut repo);

        // then
        let result = repo.get_all();
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.len(), 1);
        let first = &result[0];
        assert_eq!(first.username, "John Doe");
        assert_eq!(first.email, "johndoe@example.com")
    }

    #[test]
    fn test_service_send_message() {
        // given
        let conn = &mut get_database_connection();
        let mut repo = DbMessageRepository { connection: conn };

        let user = NewUser {
            username: &"John Doe".to_string(),
            email: &"johndoe@example.com".to_string(),
        };
        repo.save_user(&user);
        let channel = NewChannel {
            name: &"fake channel",
            description: &"a channel",
        };
        repo.save_channel(&channel);

        // when
        let message = NewMessage {
            channel_id: &1,
            user_id: &1,
            content: &"something",
        };
        send_message(message, &mut repo);

        // then
        let result = repo.get_message_by_id(1);
        assert!(result.is_some());
        assert_eq!(result.unwrap().content, "something")
    }
}
