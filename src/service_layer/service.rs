use crate::{
    adapters::{channel_repository::ChannelRepository, user_repository::UserRepository},
    domain::{auth::NewCredential, channel::NewChannel, message::NewMessage, user::NewUser},
};

pub fn grant_user_access_to_channel(
    user_id: &i32,
    channel_id: &i32,
    repo: &mut dyn ChannelRepository,
) {
    repo.grant_user_access_to_channel(user_id, channel_id);
}

pub fn create_user(
    username: &str,
    email: &str,
    password: &str,
    repo: &mut dyn UserRepository,
) -> i32 {
    // todo: this operation should be done as one transaction
    let user = NewUser { username, email };
    let user_id = repo.add_user(&user);
    let creds = NewCredential {
        user_id: &user_id,
        password,
    };
    let _ = repo.add_credential(&creds);
    user_id
}

pub fn update_credentials(
    user_id: &str,
    old_password: &str,
    new_password: &str,
    repo: &mut dyn UserRepository,
) -> bool {
    let user_id = user_id.parse::<i32>().unwrap();
    match repo.get_credential_by_user_id(user_id) {
        Some(creds) => {
            if creds.password == old_password {
                let new_credential = NewCredential {
                    user_id: &user_id,
                    password: new_password,
                };
                repo.update_credentials(&new_credential);
                true
            } else {
                return false;
            }
        }
        None => return false,
    }
}

pub fn create_channel(name: &str, description: &str, repo: &mut dyn ChannelRepository) {
    let channel = NewChannel { name, description };
    let _ = repo.add_channel(&channel);
}

pub fn create_message(
    user_id: &i32,
    channel_id: &i32,
    content: &str,
    repo: &mut dyn ChannelRepository,
) {
    if repo.can_user_send_message(user_id, channel_id) {
        let message = NewMessage {
            channel_id,
            user_id,
            content,
        };
        let _ = repo.add_message(&message);
    }
    // todo: handle more gracefully the case when the user is not authorized
}

#[cfg(test)]
mod test {
    use crate::adapters::channel_repository::{ChannelRepository, DbChannelRepository};
    use crate::adapters::user_repository::{DbUserRepository, UserRepository};
    use crate::adapters::utils::init_db;
    use crate::service_layer::service::{
        create_message, create_user, grant_user_access_to_channel,
    };

    use crate::utils::{create_test_channel_in_db, create_test_user_in_db};
    #[test]
    fn test_service_create_user() {
        // given
        let conn = &mut init_db();
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
        let result = repo.get_credential_by_user_id(retrieved_user.id).unwrap();
        assert_eq!(result.password, "xxxxxxxx")
    }

    #[test]
    fn test_user_joins_a_channel() {
        // given
        let conn = &mut init_db();
        let mut repo = DbChannelRepository { conn };

        let user_id = create_test_user_in_db();
        let channel_id = create_test_channel_in_db();

        // then: user does not have permission yet
        assert!(!repo.can_user_send_message(&user_id, &channel_id));

        // when
        grant_user_access_to_channel(&user_id, &channel_id, &mut repo);

        // then
        assert!(repo.can_user_send_message(&user_id, &channel_id))
    }

    #[test]
    fn test_service_create_message() {
        // given
        let conn = &mut init_db();
        let mut repo = DbChannelRepository { conn };

        let user_id = create_test_user_in_db();
        let channel_id = create_test_channel_in_db();
        grant_user_access_to_channel(&user_id, &channel_id, &mut repo);

        // when
        create_message(&user_id, &channel_id, &"something", &mut repo);

        // then
        let result = repo.get_message_by_id(&1);
        assert!(result.is_some());
        assert_eq!(result.unwrap().content, "something")
    }

    #[test]
    fn test_service_create_message_fail_due_to_permission() {
        // given
        let conn = &mut init_db();
        let mut repo = DbChannelRepository { conn };

        // note: we are not creating an entry in the db for the user to be permitted to post on
        // this channel
        let user_id = create_test_user_in_db();
        let channel_id = create_test_channel_in_db();

        // when
        create_message(&user_id, &channel_id, &"something", &mut repo);

        // then
        // todo: create_message should not silently fail, and instead notify that the message was not created
        let result = repo.get_message_by_id(&1);
        assert!(result.is_none());
    }
}
