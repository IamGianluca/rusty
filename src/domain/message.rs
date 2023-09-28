use crate::adapters::schema::messages;
use chrono::Utc;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i32,
    pub channel_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub channel_id: &'a i32,
    pub user_id: &'a i32,
    pub content: &'a str,
}
