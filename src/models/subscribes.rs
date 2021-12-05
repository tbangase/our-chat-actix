use chrono::NaiveDateTime;

use diesel::prelude::*;
use uuid::Uuid;
use crate::{
    schema::subscribes, 
    utils::establish_connection
};

use std::{error::Error, str::FromStr};

#[derive(Debug, Clone, Queryable, QueryableByName, Identifiable)]
#[table_name="subscribes"]
pub struct Subscribe {
    pub id: Vec<u8>,
    pub user_id: Vec<u8>,
    pub room_id: Vec<u8>,
    pub subscribe_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name="subscribes"]
pub struct NewSubscribe {
    pub user_id: Vec<u8>,
    pub room_id: Vec<u8>,
}

impl Subscribe {
    pub fn get_my_subscribes(user_id: String) -> Result<Vec<Subscribe>, Box<dyn Error>> {
        let connection = establish_connection();

        let user_id = Uuid::from_str(&user_id)?.as_bytes().to_vec();

        Ok(subscribes::dsl::subscribes
           .filter(subscribes::dsl::user_id.eq(&user_id))
           .limit(30)
           .load::<Subscribe>(&connection)?)
    }

    pub fn subscribe_room(subscribe: NewSubscribe) -> Result<Subscribe, Box<dyn Error>> {
        let connection = establish_connection();

        diesel::insert_into(subscribes::table)
            .values(&subscribe)
            .execute(&connection)?;

        Ok(subscribes::dsl::subscribes
           .order(subscribes::subscribe_at.desc())
           .first::<Subscribe>(&connection)?)
    }
}
