use crate::{adapters::repository::UserRepository, domain::user::NewUser};

fn create_user<'a>(user: NewUser<'a>, repo: &mut UserRepository<'a>) {
    let _ = repo.save(&user);
}

#[cfg(test)]
mod test {
    use crate::adapters::repository::UserRepository;
    use crate::domain::user::NewUser;
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
        let mut repo = UserRepository::new(conn);

        let user = NewUser {
            username: &"John Doe".to_string(),
            email: &"johndoe@example.com".to_string(),
        };

        // then: no user in the db
        let result = repo.find_all();
        assert_eq!(result.unwrap().len(), 0);

        // when
        create_user(user, &mut repo);

        // then
        let result = repo.find_all();
        assert!(result.is_ok());
        let retrieved_users = result.unwrap();
        assert_eq!(retrieved_users.len(), 1);
        let first = &retrieved_users[0];
        assert_eq!(first.username, "John Doe");
        assert_eq!(first.email, "johndoe@example.com")
    }
}
