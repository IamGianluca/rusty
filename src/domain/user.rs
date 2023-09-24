use crate::adapters::schema::users;
use chrono::Utc;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
}

#[cfg(test)]
mod test {

    use crate::domain::user::NewUser;

    #[test]
    fn test_create_user() {
        let user1 = NewUser {
            username: &"user1".to_string(),
            email: &"fake@email.com".to_string(),
        };
        assert_eq!(user1.username, "user1")
    }
}
