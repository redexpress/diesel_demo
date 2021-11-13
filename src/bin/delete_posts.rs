extern crate diesel;
extern crate diesel_demo;

use diesel::prelude::*;
use std::env::args;
use diesel_demo::establish_connection;
use diesel_demo::schema::posts::dsl::posts;
use diesel_demo::schema::posts::columns::title;

fn main() {
    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
