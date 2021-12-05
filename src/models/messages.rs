use chrono::NaiveDateTime;
use diesel::prelude::*;

use uuid::Uuid;
use crate::{
    schema::messages, 
    utils::establish_connection
};

use std::{error::Error, str::FromStr};

#[derive(Debug, Clone, Queryable, QueryableByName, Identifiable)]
#[table_name="messages"]
pub struct Message {
    pub id: Vec<u8>,
    pub user_id: Vec<u8>,
    pub room_id: Vec<u8>,
    pub message: String,
    pub send_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name="messages"]
pub struct NewMessage {
    pub user_id: Vec<u8>,
    pub room_id: Vec<u8>,
    pub message: String,
}

impl Message {
    pub fn get_messages(room_id: String) 
        -> Result<Vec<Message>, Box<dyn Error>> 
    {
        let connection = establish_connection();

        let room_id = Uuid::from_str(&room_id)?.as_bytes().to_vec();

        Ok(messages::dsl::messages
           .filter(messages::dsl::room_id.eq(&room_id))
           .limit(50)
           .load::<Message>(&connection)?)
    }

    pub fn send_message(message: NewMessage) -> Result<Message, Box<dyn Error>> {
        let connection = establish_connection();

        diesel::insert_into(messages::table)
            .values(&message)
            .execute(&connection)?;

        Ok(messages::dsl::messages
           .order(messages::send_at.desc())
           .first::<Message>(&connection)?)
    }
}
