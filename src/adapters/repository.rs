use crate::domain::user::{NewUser, User};
use chrono::Utc;
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
        Ok(User {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: Utc::now(),
        })
    }

    fn save_user(&mut self, user: &NewUser) -> Result<(), Box<dyn Error + 'static>> {
        use crate::adapters::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(user)
            .execute(&mut *self.connection)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::repository::UserRepository;
    use crate::domain::user::NewUser;
    use diesel::prelude::*;
    use dotenvy::dotenv;
    use std::env;

    fn rebuild_db() {
        use std::process::Command;
        Command::new("diesel")
            .arg("migration")
            .arg("redo")
            .output()
            .expect("Something is wrong");
    }

    #[test]
    fn test_create_user_via_repository() {
        // given
        rebuild_db();
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set.");
        let conn = &mut PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        // when
        let mut repo = UserRepository::new(conn);
        let user = NewUser {
            username: &"John Doe".to_string(),
            email: &"johndoe@example.com".to_string(),
        };
        repo.save_user(&user).unwrap();

        // then
        let user = repo.get_user(1).unwrap();
        assert_eq!(user.username, "John Doe")
    }
}
