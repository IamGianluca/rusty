use crate::{
    adapters::{message_repository::MessageRepository, user_repository::UserRepository},
    domain::{message::NewMessage, user::NewUser},
};

pub fn authenticate_user(user: &str, repo: &mut dyn UserRepository) -> bool {
    match repo.get_user_by_name(user) {
        Some(_) => true,
        None => false,
    }
}

pub fn create_user(user: NewUser, repo: &mut dyn UserRepository) {
    let _ = repo.save_user(&user);
}

pub fn send_message(message: NewMessage, repo: &mut dyn MessageRepository) {
    let _ = repo.save_message(&message);
}

#[cfg(test)]
mod test {
    use crate::adapters::message_repository::{DbMessageRepository, MessageRepository};
    use crate::adapters::user_repository::{DbUserRepository, UserRepository};
    use crate::adapters::utils::get_new_test_database_connection;
    use crate::domain::channel::NewChannel;
    use crate::domain::message::NewMessage;
    use crate::service_layer::service::{authenticate_user, create_user, send_message};

    use crate::utils::create_test_user;
    #[test]
    fn test_service_create_user() {
        // given
        let conn = &mut get_new_test_database_connection();
        let mut repo = DbUserRepository { conn };

        let user = create_test_user();

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
        let conn = &mut get_new_test_database_connection();
        let mut repo = DbMessageRepository { conn };

        let user = create_test_user();
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

    #[test]
    fn test_service_authenticate_user() {
        // given
        let conn = &mut get_new_test_database_connection();
        let mut repo = DbUserRepository { conn };

        // when
        let response = authenticate_user("John Doe", &mut repo);

        // then
        assert!(!response);

        // given
        let user = create_test_user();
        repo.save_user(&user);

        // when
        let response = authenticate_user("John Doe", &mut repo);

        // then
        assert!(response);
    }
}
