pub mod schema;
pub mod model;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{Connection, SqliteConnection};

use diesel::prelude::*;
use crate::model::NewInformation;
use crate::schema::information;

fn main() {
    let database_url = "dev.db";
    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    create_post(&mut connection, "hello", "zero");
}

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> usize {
    let new_post = NewInformation { title, body };

    diesel::insert_into(information::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}
