use crate::adapters::schema::channel_permissions;
use crate::adapters::schema::channels;
use chrono::Utc;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Channel {
    pub id: i32,
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
#[diesel(table_name = channel_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelPermission {
    pub id: i32,
    pub user_id: i32,
    pub channel_id: i32,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = channel_permissions)]
pub struct NewChannelPermission<'a> {
    pub user_id: &'a i32,
    pub channel_id: &'a i32,
}
