use crate::adapters::schema::channels;
use crate::adapters::schema::messages;
use chrono::Utc;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Channel {
    pub id: i32,
    // should create a workspace concept and have workspace_id in here
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = channels)]
pub struct NewChannel<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i32,
    // perhaps we should set this to NOT NULL in the sql query
    // https://github.com/diesel-rs/diesel/issues/330#issuecomment-219279105
    pub channel_id: Option<i32>,
    pub user_id: Option<i32>,
    pub content: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub content: &'a str,
}
