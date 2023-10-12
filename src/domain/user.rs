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

    use crate::utils::create_test_user;

    #[test]
    fn test_create_user() {
        let user1 = create_test_user();
        assert_eq!(user1.username, "John Doe")
    }
}
