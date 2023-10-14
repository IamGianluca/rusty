use diesel::prelude::*;

use crate::domain::channel::{Channel, NewChannel};
use crate::domain::message::{Message, NewMessage};
use crate::domain::user::{NewUser, User};

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
        use crate::adapters::schema::users::dsl::*;

        let row_inserted = diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&mut *self.conn)
            .unwrap();
        row_inserted.id
    }
    fn get_user_by_id(&mut self, user_id: i32) -> Option<User> {
        use crate::adapters::schema::users::dsl::*;

        let user: User = users.filter(id.eq(user_id)).first(&mut *self.conn).ok()?;
        Some(user)
    }

    // channel
    fn save_channel(&mut self, channel: &NewChannel) -> i32 {
        use crate::adapters::schema::channels::dsl::*;

        let row_inserted = diesel::insert_into(channels)
            .values(channel)
            .get_result::<Channel>(&mut *self.conn)
            .unwrap();
        row_inserted.id
    }

    fn get_channel_by_id(&mut self, channel_id: i32) -> Option<Channel> {
        use crate::adapters::schema::channels::dsl::*;

        let channel: Channel = channels
            .filter(id.eq(channel_id))
            .first(&mut *self.conn)
            .ok()?;
        Some(channel)
    }

    // message
    fn save_message(&mut self, message: &NewMessage) -> i32 {
        use crate::adapters::schema::messages::dsl::*;

        let row_inserted = diesel::insert_into(messages)
            .values(message)
            .get_result::<Message>(&mut *self.conn)
            .unwrap();
        row_inserted.id
    }

    fn get_message_by_id(&mut self, message_id: i32) -> Option<Message> {
        use crate::adapters::schema::messages::dsl::*;

        let msg = messages
            .filter(id.eq(message_id))
            .first(&mut *self.conn)
            .ok()?;
        Some(msg)
    }
}
