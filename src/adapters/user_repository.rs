use crate::domain::auth::{Credential, NewCredential};
use crate::domain::user::{NewUser, User};
use diesel::prelude::*;
use diesel::{self, PgConnection};

pub trait UserRepository {
    fn add_user(&mut self, user: &NewUser) -> i32;
    fn get_user_by_id(&mut self, id: i32) -> Option<User>;
    fn get_user_by_name(&mut self, name: &str) -> Option<User>;
    fn add_credential(&mut self, credential: &NewCredential) -> i32;
    fn get_credential_by_user_id(&mut self, user_id: i32) -> Option<Credential>;
    fn get_all(&mut self) -> Option<Vec<User>>;
}

pub struct DbUserRepository<'a> {
    pub conn: &'a mut PgConnection,
}

impl UserRepository for DbUserRepository<'_> {
    fn add_user(&mut self, user: &NewUser) -> i32 {
        use crate::adapters::schema::users::dsl::*;

        let inserted_user = diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&mut *self.conn);
        inserted_user.unwrap().id
    }

    fn get_user_by_id(&mut self, user_id: i32) -> Option<User> {
        use crate::adapters::schema::users::dsl::*;

        users.filter(id.eq(user_id)).first(&mut *self.conn).ok()
    }

    fn get_user_by_name(&mut self, name: &str) -> Option<User> {
        use crate::adapters::schema::users::dsl::*;

        users
            .filter(username.eq(name))
            .first::<User>(&mut *self.conn)
            .ok()
    }

    fn add_credential(&mut self, credential: &NewCredential) -> i32 {
        use crate::adapters::schema::credentials::dsl::*;

        let inserted_credential = diesel::insert_into(credentials)
            .values(credential)
            .get_result::<Credential>(&mut *self.conn);
        inserted_credential.unwrap().id
    }

    fn get_credential_by_user_id(&mut self, _user_id: i32) -> Option<Credential> {
        use crate::adapters::schema::credentials::dsl::*;

        credentials
            .filter(user_id.eq(_user_id))
            .first::<Credential>(&mut *self.conn)
            .ok()
    }

    fn get_all(&mut self) -> Option<Vec<User>> {
        use crate::adapters::schema::users::dsl::*;

        users.select(User::as_select()).load(&mut *self.conn).ok()
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::user_repository::{DbUserRepository, UserRepository};
    use crate::adapters::utils::init_db;
    use crate::utils::create_test_user;

    #[test]
    fn test_create_method() {
        // given
        let conn = &mut init_db();

        // when
        let mut repo = DbUserRepository { conn };
        let user = create_test_user();
        let result = repo.add_user(&user);

        // then
        assert_eq!(result, 1)
    }

    #[test]
    fn test_get_user_by_id_when_users_table_is_empty() {
        // given
        let conn = &mut init_db();

        // when
        let mut repo = DbUserRepository { conn };
        let result = repo.get_user_by_id(1);

        // then
        assert_eq!(result.is_some(), false)
    }

    #[test]
    fn test_get_user_by_id() {
        // given
        let conn = &mut init_db();

        let mut repo = DbUserRepository { conn };
        let new_user = create_test_user();
        let id = repo.add_user(&new_user);

        // when
        let retrieved_user = repo.get_user_by_id(id).unwrap();

        // then
        assert_eq!(retrieved_user.username, new_user.username)
    }
    #[test]
    fn test_get_user_by_name() {
        // given
        let conn = &mut init_db();

        let mut repo = DbUserRepository { conn };
        let new_user = create_test_user();
        let _ = repo.add_user(&new_user);

        // when
        let retrieved_user = repo.get_user_by_name(&"John Doe").unwrap();

        // then
        assert_eq!(retrieved_user.username, new_user.username)
    }
}
