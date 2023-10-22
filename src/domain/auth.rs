use crate::adapters::schema::credentials;
use chrono::Utc;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = credentials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Credential {
    pub id: i32,
    pub user_id: i32,
    pub password: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = credentials)]
pub struct NewCredential<'a> {
    pub user_id: &'a i32,
    pub password: &'a str,
}

// pub fn authenticate_user() -> bool {
//     true
// }
