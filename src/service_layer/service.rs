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
    use crate::adapters::user_repository::DbUserRepository;
    use crate::adapters::user_repository::UserRepository;
    use crate::domain::channel::NewChannel;
    use crate::domain::message::NewMessage;
    use crate::domain::user::NewUser;
    use crate::service_layer::service::send_message;
    use diesel::prelude::*;
    use dotenvy::dotenv;
    use std::env;

    use super::create_user;

    fn rebuild_database() {
        use std::process::Command;
        Command::new("diesel")
            .arg("migration")
            .arg("redo")
            .output()
            // rename error message
            .expect("Something is wrong");
    }

    fn get_database_url() -> String {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set.")
    }

    fn get_database_connection() -> diesel::PgConnection {
        rebuild_database();
        let database_url = get_database_url();
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

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
    fn test_send_message() {
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
