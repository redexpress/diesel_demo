use diesel::debug_query;
use diesel::prelude::*;
use diesel_demo::schema::users::dsl::users;
use diesel_demo::schema::users::columns::{name, id};
use diesel_demo::models::user::User;
use diesel_demo::establish_connection;
use diesel_demo::schema::blogs;

fn main() {

}

/// `foo = (SELECT ...)`这样子的查询语句
pub fn single_value_example() {
    let connection = establish_connection();

    let last_blog = blogs::table.order(blogs::id.desc());

    let most_recently_active_user = users
        .select(name)
        .filter(
            id.nullable()
                .eq(last_blog.select(blogs::user_id).single_value()),
        )
        .first::<String>(&connection)
        .unwrap();
    println!("most_recently_active_user:{:?}", most_recently_active_user);

    let query = users.select(name).filter(
        id.nullable()
            .eq(last_blog.select(blogs::user_id).single_value()),
    );
    let data = query.first::<String>(&connection).unwrap();
    println!("data:{:?}", data);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    println!("SQL:{:?}", sql);

    println!("single_value end");
}

pub fn get_result_example() {
    let connection = establish_connection();
    let inserted_row = diesel::insert_into(users)
        .values(name.eq("Ruby"))
        .get_result::<User>(&connection)
        .unwrap();
    println!("inserted_row: {:?}", inserted_row);

    let update_result = diesel::update(users.find(31))
        .set(name.eq("Jim"))
        .get_result::<(i32, String)>(&connection);
    println!("update_result:{:?}", update_result);
    println!("get_result end");
}