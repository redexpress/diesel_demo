extern crate diesel;
extern crate diesel_demo;

use std::env::args;
use diesel::prelude::*;
use diesel_demo::establish_connection;
use diesel_demo::schema::posts::dsl::posts;
use diesel_demo::schema::posts::columns::published;
use diesel_demo::models::post::Post;

fn main() {
    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}