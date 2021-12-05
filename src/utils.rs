use data_encoding::HEXUPPER;
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn establish_connection() -> MysqlConnection {
    dotenv::from_filename("/.envs/.local/.rust").ok();
    let config = crate::config::Config::from_env().unwrap();

    let database_url = format!("mysql://{0}:{1}@{2}:{3}/{4}",
                               config.mysql_user.clone(),
                               config.mysql_password.clone(),
                               config.mysql_host.clone(),
                               config.mysql_port.clone(),
                               config.mysql_database.clone());

    // println!("Database URL: {}", database_url);

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn hash_string(item: String) -> String {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;

    let n_iter = NonZeroU32::new(100_000).unwrap();
    // let mut salt = [0u8; CREDENTIAL_LEN];
    // let rng = rand::SystemRandom::new();
    // rng.fill(&mut salt);
    let salt = "mitsUibau maTeial".as_bytes();

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        n_iter,
        &salt,
        item.as_bytes(),
        &mut pbkdf2_hash,
    );

    HEXUPPER.encode(&pbkdf2_hash).clone()
}
