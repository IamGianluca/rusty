use crate::domain::user::{NewUser, User};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use std::error::Error;

use super::schema::users;

struct UserRepository {
    connection: PgConnection,
}

impl UserRepository {
    fn new(url: &str) -> Self {
        let conn = diesel::Connection::establish(url).unwrap();
        Self { connection: conn }
    }

    fn get_user(&mut self, id: i32) -> Result<User, Box<dyn Error + 'static>> {
        let user: User = users::table.find(id).first(&mut self.connection)?;
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
            .execute(&mut self.connection)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::repository::UserRepository;
    use crate::domain::user::NewUser;
    use dotenvy::dotenv;
    use std::env;

    #[test]
    fn test_create_user_via_repository() {
        dotenv().ok();

        let binding = env::var("DATABASE_URL").unwrap();
        let connection = binding.as_str();
        let mut repo = UserRepository::new(connection);

        // save a user
        let user = NewUser {
            username: &"John Doe".to_string(),
            email: &"johndoe@example.com".to_string(),
        };
        repo.save_user(&user).unwrap();

        // get a user by id
        let user = repo.get_user(1).unwrap();
        println!("{}", user.username);
    }
}
