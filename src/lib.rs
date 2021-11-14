#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::pg::PgConnection;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
use self::models::post::{NewPost, Post};

pub fn establish_connection() -> DbConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("can't create Postgresql pool");
    let conn = pool.get().expect("connect error");
    conn
}

// pub fn establish_connection_without_pool() -> PgConnection {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }


pub fn create_post(conn: &PgConnection, title: &str, body: &str) -> Post {
    use schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

