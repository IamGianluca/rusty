use crate::domain::user::{NewUser, User};
use diesel::prelude::*;
use diesel::{self, PgConnection};

use super::schema::users;

pub trait UserRepository {
    fn save_user(&mut self, user: &NewUser) -> i32;
    fn get_user_by_id(&mut self, id: i32) -> Option<User>;
    fn get_all(&mut self) -> Option<Vec<User>>;
}

pub struct DbUserRepository<'a> {
    pub connection: &'a mut PgConnection,
}

impl UserRepository for DbUserRepository<'_> {
    fn get_user_by_id(&mut self, id: i32) -> Option<User> {
        let user: User = users::table.find(id).first(&mut *self.connection).ok()?;
        Some(user)
    }

    fn get_all(&mut self) -> Option<Vec<User>> {
        let result: Vec<User> = users::table
            .select(User::as_select())
            .load(&mut *self.connection)
            .ok()?;
        Some(result)
    }

    fn save_user(&mut self, user: &NewUser) -> i32 {
        use crate::adapters::schema::users::dsl::*;

        let inserted_user = diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&mut *self.connection);
        inserted_user.unwrap().id
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::user_repository::{DbUserRepository, UserRepository};
    use crate::adapters::utils::get_new_database_connection;
    use crate::domain::user::NewUser;

    #[test]
    fn test_create_method() {
        // given
        let conn = &mut get_new_database_connection();

        // when
        let mut repo = DbUserRepository { connection: conn };
        let user = NewUser {
            username: &"John Doe".to_string(),
            email: &"johndoe@example.com".to_string(),
        };
        let result = repo.save_user(&user);

        // then
        assert_eq!(result, 1)
    }

    #[test]
    fn test_get_method_with_empty_user_table() {
        // given
        let conn = &mut get_new_database_connection();

        // when
        let mut repo = DbUserRepository { connection: conn };
        let result = repo.get_user_by_id(1);

        // then
        assert_eq!(result.is_some(), false)
    }

    #[test]
    fn test_get_user_that_was_just_added() {
        // given
        let conn = &mut get_new_database_connection();

        let mut repo = DbUserRepository { connection: conn };
        let inserted_user = NewUser {
            username: &"John Doe".to_string(),
            email: &"johndoe@example.com".to_string(),
        };
        let id = repo.save_user(&inserted_user);

        // when
        let retrieved_user = repo.get_user_by_id(id).unwrap();

        // then
        assert_eq!(retrieved_user.username, inserted_user.username)
    }
}
