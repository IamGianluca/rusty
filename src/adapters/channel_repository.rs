use diesel::prelude::*;

use crate::domain::channel::{Channel, ChannelPermission, NewChannel, NewChannelPermission};
use crate::domain::message::{Message, NewMessage};
use crate::domain::user::{NewUser, User};

pub trait ChannelRepository {
    fn add_user(&mut self, user: &NewUser) -> i32;
    fn get_user_by_id(&mut self, user_id: &i32) -> Option<User>;
    fn can_user_send_message(&mut self, user_id: &i32, channel_id: &i32) -> bool;
    fn grant_user_access_to_channel(&mut self, user_id: &i32, channel_id: &i32) -> i32;
    fn add_channel(&mut self, channel: &NewChannel) -> i32;
    fn get_channel_by_id(&mut self, channel_id: &i32) -> Option<Channel>;
    fn add_message(&mut self, message: &NewMessage) -> i32;
    fn get_message_by_id(&mut self, message_id: &i32) -> Option<Message>;
}

pub struct DbChannelRepository<'a> {
    pub conn: &'a mut PgConnection,
}

impl ChannelRepository for DbChannelRepository<'_> {
    // user
    fn add_user(&mut self, user: &NewUser) -> i32 {
        use crate::adapters::schema::users::dsl::*;

        let row_inserted = diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&mut *self.conn)
            .unwrap();
        row_inserted.id
    }

    fn get_user_by_id(&mut self, user_id: &i32) -> Option<User> {
        use crate::adapters::schema::users::dsl::*;

        users.filter(id.eq(user_id)).first(&mut *self.conn).ok()
    }

    fn can_user_send_message(&mut self, _user_id: &i32, _channel_id: &i32) -> bool {
        use crate::adapters::schema::channel_permissions::dsl::*;

        let res = channel_permissions
            .filter(channel_id.eq(_channel_id).and(user_id.eq(_user_id)))
            .first::<ChannelPermission>(&mut *self.conn)
            .ok();
        match res {
            Some(_) => true,
            None => false,
        }
    }

    fn grant_user_access_to_channel(&mut self, _user_id: &i32, _channel_id: &i32) -> i32 {
        use crate::adapters::schema::channel_permissions::dsl::*;

        let permission = NewChannelPermission {
            user_id: _user_id,
            channel_id: _channel_id,
        };
        let row_inserted = diesel::insert_into(channel_permissions)
            .values(permission)
            .get_result::<ChannelPermission>(&mut *self.conn)
            .unwrap();
        row_inserted.id
    }
    // channel
    fn add_channel(&mut self, channel: &NewChannel) -> i32 {
        use crate::adapters::schema::channels::dsl::*;

        let row_inserted = diesel::insert_into(channels)
            .values(channel)
            .get_result::<Channel>(&mut *self.conn)
            .unwrap();
        row_inserted.id
    }

    fn get_channel_by_id(&mut self, channel_id: &i32) -> Option<Channel> {
        use crate::adapters::schema::channels::dsl::*;

        channels
            .filter(id.eq(channel_id))
            .first(&mut *self.conn)
            .ok()
    }

    // message
    fn add_message(&mut self, message: &NewMessage) -> i32 {
        use crate::adapters::schema::messages::dsl::*;

        let row_inserted = diesel::insert_into(messages)
            .values(message)
            .get_result::<Message>(&mut *self.conn)
            .unwrap();
        row_inserted.id
    }

    fn get_message_by_id(&mut self, message_id: &i32) -> Option<Message> {
        use crate::adapters::schema::messages::dsl::*;

        messages
            .filter(id.eq(message_id))
            .first(&mut *self.conn)
            .ok()
    }
}
