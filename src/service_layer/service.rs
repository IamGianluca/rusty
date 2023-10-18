use crate::{
    adapters::{message_repository::MessageRepository, user_repository::UserRepository},
    domain::{auth::NewPassword, channel::NewChannel, message::NewMessage, user::NewUser},
};

pub fn authenticate_user(user: &str, repo: &mut dyn UserRepository) -> bool {
    match repo.get_user_by_name(user) {
        Some(_) => true,
        None => false,
    }
}

pub fn create_user(username: &str, email: &str, password: &str, repo: &mut dyn UserRepository) {
    // todo: this operation should be done as one transaction
    let user = NewUser { username, email };
    let user_id = repo.add_user(&user);
    let creds = NewPassword {
        user_id: &user_id,
        password,
    };
    let _ = repo.add_password(&creds);
}

pub fn create_channel(name: &str, description: &str, repo: &mut dyn MessageRepository) {
    let channel = NewChannel { name, description };
    let _ = repo.add_channel(&channel);
}

pub fn create_message(
    user_id: &i32,
    channel_id: &i32,
    content: &str,
    repo: &mut dyn MessageRepository,
) {
    let message = NewMessage {
        channel_id,
        user_id,
        content,
    };
    let _ = repo.add_message(&message);
}

#[cfg(test)]
mod test {
    use crate::adapters::message_repository::{DbMessageRepository, MessageRepository};
    use crate::adapters::user_repository::{DbUserRepository, UserRepository};
    use crate::adapters::utils::get_new_test_database_connection;
    use crate::domain::channel::NewChannel;
    use crate::service_layer::service::{authenticate_user, create_message, create_user};

    use crate::utils::create_test_user;
    #[test]
    fn test_service_create_user() {
        // given
        let conn = &mut get_new_test_database_connection();
        let mut repo = DbUserRepository { conn };

        // then: no user in the db, empty vector
        let result = repo.get_all().unwrap();
        assert_eq!(result.len(), 0);

        // when
        create_user("John Doe", "johndoe@example.com", "password", &mut repo);
        create_user("Jane Doe", "janedoe@example.com", "xxxxxxxx", &mut repo);

        // then
        let result = repo.get_all().unwrap();
        assert_eq!(result.len(), 2);
        let retrieved_user = &result[1];
        assert_eq!(retrieved_user.username, "Jane Doe");
        assert_eq!(retrieved_user.email, "janedoe@example.com");
        let result = repo.get_password_by_user_id(retrieved_user.id).unwrap();
        assert_eq!(result.password, "xxxxxxxx")
    }

    #[test]
    fn test_service_create_message() {
        // given
        let conn = &mut get_new_test_database_connection();
        let mut repo = DbMessageRepository { conn };

        let user = create_test_user();
        repo.add_user(&user);
        let channel = NewChannel {
            name: &"fake channel",
            description: &"a channel",
        };
        repo.add_channel(&channel);

        // when
        create_message(&1, &1, &"something", &mut repo);

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
        repo.add_user(&user);

        // when
        let response = authenticate_user("John Doe", &mut repo);

        // then
        assert!(response);
    }
}
