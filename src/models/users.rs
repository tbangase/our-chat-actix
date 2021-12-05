use crate::utils::hash_string;
use std::str::FromStr;

use crate::utils::establish_connection;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::users;

use std::error::Error;

#[derive(Debug, Clone, Queryable, QueryableByName, Identifiable)]
#[table_name= "users"]
pub struct User {
    pub id: Vec<u8>,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name= "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn get_all() -> Result<Vec<User>, Box<dyn Error>> {
        let connection = establish_connection();
        Ok(users::dsl::users
            .limit(30)
            .load::<User>(&connection)?)
    }

    pub fn auth(id: String, password: String) -> Result<User, Box<dyn Error>> {
        let connection = establish_connection();

        let uuid = Uuid::from_str(&id)?.as_bytes().to_vec();
        let password = hash_string(password);

        Ok(users::dsl::users
            .filter(users::id.eq(uuid))
            .filter(users::password.eq(password))
            .first::<User>(&connection)?)
    }

    pub fn create(items: Vec<NewUser>) -> Result<Vec<User>, Box<dyn Error>> {
        use self::users::id;
        let connection = establish_connection();

        let mut result: Vec<User> = vec![];
        for mut item in items {
            item.password = hash_string(item.password);

            diesel::insert_into(users::table)
                .values(&item)
                .execute(&connection)?;
            result.push(
                users::dsl::users
                    .order(id.desc())
                    .first::<User>(&connection)?,
            )
        }
        Ok(result)
    }

}

