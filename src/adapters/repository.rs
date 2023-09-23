use crate::domain::user::{NewUser, User};
use diesel::prelude::*;
use diesel::{self, PgConnection};
use std::error::Error;

use super::schema::users;

struct UserRepository<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> UserRepository<'a> {
    fn new(connection: &'a mut PgConnection) -> Self {
        UserRepository { connection }
    }

    fn get_user(&mut self, id: i32) -> Result<User, Box<dyn Error + 'static>> {
        let user: User = users::table.find(id).first(&mut *self.connection)?;
        Ok(user)
    }

    fn save_user(&mut self, user: &NewUser) -> Result<User, Box<dyn Error + 'static>> {
        use crate::adapters::schema::users::dsl::*;

        let inserted_user = diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&mut *self.connection)?;
        Ok(inserted_user)
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::repository::UserRepository;
    use crate::domain::user::NewUser;
    use diesel::prelude::*;
    use dotenvy::dotenv;
    use std::env;

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
    fn test_create_method() {
        // given
        let conn = &mut get_database_connection();

        // when
        let mut repo = UserRepository::new(conn);
        let user = NewUser {
            username: &"John Doe".to_string(),
            email: &"johndoe@example.com".to_string(),
        };
        let result = repo.save_user(&user);

        // then
        assert!(result.is_ok())
    }

    #[test]
    fn test_get_method_with_empty_user_table() {
        // given
        let conn = &mut get_database_connection();

        // when
        let mut repo = UserRepository::new(conn);
        let result = repo.get_user(999);

        // then
        assert_eq!(result.is_ok(), false)
    }

    #[test]
    fn test_get_user_that_was_just_added() {
        // given
        let conn = &mut get_database_connection();

        let mut repo = UserRepository::new(conn);
        let user = NewUser {
            username: &"John Doe".to_string(),
            email: &"johndoe@example.com".to_string(),
        };
        let inserted_user = repo.save_user(&user).unwrap();

        // when
        let retrieved_user = repo.get_user(inserted_user.id).unwrap();

        // then
        assert_eq!(retrieved_user.username, inserted_user.username)
    }
}
