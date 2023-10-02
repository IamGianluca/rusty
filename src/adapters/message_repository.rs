use diesel::prelude::*;

use crate::domain::channel::NewChannel;
use crate::domain::message::{Message, NewMessage};
use crate::domain::user::NewUser;
use crate::domain::{channel::Channel, user::User};

use super::schema::channels;
use super::schema::messages;
use super::schema::users;

pub trait MessageRepository {
    fn save_user(&mut self, user: &NewUser);
    fn get_user_by_id(&mut self, id: i32) -> Option<User>;
    fn save_channel(&mut self, channel: &NewChannel);
    fn get_channel_by_id(&mut self, id: i32) -> Option<Channel>;
    fn save_message(&mut self, message: &NewMessage);
    fn get_message_by_id(&mut self, id: i32) -> Option<Message>;
}

pub struct DbMessageRepository<'a> {
    pub connection: &'a mut PgConnection,
}

impl MessageRepository for DbMessageRepository<'_> {
    // user
    fn save_user(&mut self, user: &NewUser) {
        let _rows_inserted = diesel::insert_into(users::table)
            .values(user)
            .execute(&mut *self.connection);
    }
    fn get_user_by_id(&mut self, id: i32) -> Option<User> {
        let user: User = users::table.find(id).first(&mut *self.connection).ok()?;
        Some(user)
    }

    // channel
    fn save_channel(&mut self, channel: &NewChannel) {
        let _rows_inserted = diesel::insert_into(channels::table)
            .values(channel)
            .execute(&mut *self.connection);
    }

    fn get_channel_by_id(&mut self, id: i32) -> Option<Channel> {
        let channel: Channel = channels::table.find(id).first(&mut *self.connection).ok()?;
        Some(channel)
    }

    // message
    fn save_message(&mut self, message: &NewMessage) {
        let _rows_inserted = diesel::insert_into(messages::table)
            .values(message)
            .execute(&mut *self.connection);
    }

    fn get_message_by_id(&mut self, id: i32) -> Option<Message> {
        let msg = messages::table.find(id).first(&mut *self.connection).ok()?;
        Some(msg)
    }
}
