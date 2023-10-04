use diesel::prelude::*;

use crate::domain::channel::{Channel, NewChannel};
use crate::domain::message::{Message, NewMessage};
use crate::domain::user::{NewUser, User};

use super::schema::channels;
use super::schema::messages;
use super::schema::users;

pub trait MessageRepository {
    fn save_user(&mut self, user: &NewUser) -> i32;
    fn get_user_by_id(&mut self, id: i32) -> Option<User>;
    fn save_channel(&mut self, channel: &NewChannel) -> i32;
    fn get_channel_by_id(&mut self, id: i32) -> Option<Channel>;
    fn save_message(&mut self, message: &NewMessage) -> i32;
    fn get_message_by_id(&mut self, id: i32) -> Option<Message>;
}

pub struct DbMessageRepository<'a> {
    pub conn: &'a mut PgConnection,
}

impl MessageRepository for DbMessageRepository<'_> {
    // user
    fn save_user(&mut self, user: &NewUser) -> i32 {
        let row_inserted = diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(&mut *self.conn)
            .unwrap();
        row_inserted.id
    }
    fn get_user_by_id(&mut self, id: i32) -> Option<User> {
        let user: User = users::table.find(id).first(&mut *self.conn).ok()?;
        Some(user)
    }

    // channel
    fn save_channel(&mut self, channel: &NewChannel) -> i32 {
        let row_inserted = diesel::insert_into(channels::table)
            .values(channel)
            .get_result::<Channel>(&mut *self.conn)
            .unwrap();
        row_inserted.id
    }

    fn get_channel_by_id(&mut self, id: i32) -> Option<Channel> {
        let channel: Channel = channels::table.find(id).first(&mut *self.conn).ok()?;
        Some(channel)
    }

    // message
    fn save_message(&mut self, message: &NewMessage) -> i32 {
        let row_inserted = diesel::insert_into(messages::table)
            .values(message)
            .get_result::<Message>(&mut *self.conn)
            .unwrap();
        row_inserted.id
    }

    fn get_message_by_id(&mut self, id: i32) -> Option<Message> {
        let msg = messages::table.find(id).first(&mut *self.conn).ok()?;
        Some(msg)
    }
}
