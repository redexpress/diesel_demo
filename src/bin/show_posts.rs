extern crate diesel;
extern crate diesel_demo;

use diesel::prelude::*;
use diesel_demo::establish_connection;
use diesel_demo::schema::posts::dsl::posts;
use diesel_demo::schema::posts::columns::published;
use diesel_demo::models::Post;

fn main() {
    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
