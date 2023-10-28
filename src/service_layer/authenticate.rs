use std::env;

use crate::adapters::user_repository::UserRepository;

pub fn get_secret_key() -> String {
    use dotenvy::dotenv;
    dotenv().ok();
    env::var("SECRET_KEY").expect("SECRET_KEY environment is not set.")
}

pub fn create_token() -> String {
    get_secret_key()
}

pub fn validate_token(token: &str) -> Result<(), String> {
    if token == get_secret_key() {
        Ok(())
    } else {
        Err("Invalid token".to_string())
    }
}

pub fn authenticate_user(username: &str, password: &str, repo: &mut dyn UserRepository) -> bool {
    match repo.get_user_by_name(username) {
        Some(user) => {
            let credentials = repo.get_credential_by_user_id(user.id).unwrap();
            if credentials.password == password {
                return true;
            } else {
                return false;
            }
        }
        None => return false,
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::user_repository::DbUserRepository;
    use crate::adapters::utils::init_db;
    use crate::service_layer::authenticate::authenticate_user;
    use crate::service_layer::service::create_user;

    #[test]
    fn test_service_authenticate_user() {
        // given
        let conn = &mut init_db();
        let mut repo = DbUserRepository { conn };

        // when
        let response = authenticate_user("John Doe", "password", &mut repo);

        // then
        assert!(!response);

        // given
        create_user("John Doe", "johndoe@example.com", "password", &mut repo);

        // when
        let response = authenticate_user("John Doe", "password", &mut repo);

        // then
        assert!(response);
    }
}
