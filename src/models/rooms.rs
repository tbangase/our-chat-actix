use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::{schema::rooms, utils::establish_connection};

use std::error::Error;

#[derive(Debug, Clone, Queryable, QueryableByName, Identifiable)]
#[table_name="rooms"]
pub struct Room {
    pub id: Vec<u8>,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name="rooms"]
pub struct NewRoom {
    pub name: String
}

impl Room {
    pub fn get_all() -> Result<Vec<Room>, Box<dyn Error>>{
        let connection = establish_connection();
        Ok(rooms::dsl::rooms.limit(30)
           .load::<Room>(&connection)?)
    }

    pub fn create(room: NewRoom) -> Result<Room, Box<dyn Error>> {
        let connection = establish_connection();

        diesel::insert_into(rooms::table)
            .values(&room)
            .execute(&connection)?;

        Ok(rooms::dsl::rooms
            .order(rooms::created_at.desc())
            .first::<Room>(&connection)?)
    }
}

