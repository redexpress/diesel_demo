use diesel::prelude::*;
use diesel_demo::establish_connection;
use diesel_demo::schema::animals::dsl::*;
use diesel_demo::schema::animals::columns::*;
use diesel_demo::models::animal::Animal;

fn main() {

}


fn animal_distinct() {
    println!("' DISTINCT ON '子句查询");

    let connection = establish_connection();
    connection.execute("DELETE FROM animals").unwrap(); //删除animals表数据
    let result = diesel::insert_into(animals)
        .values(&vec![
            (species.eq("dog"), name.eq(Some("招财狗")), legs.eq(4)),
            (species.eq("dog"), name.eq(None), legs.eq(4)),
            (species.eq("spider"), name.eq(None), legs.eq(8)),
        ])
        .execute(&connection)
        .unwrap();

    // let all_animals = animals.select((species, name, legs)).load(&connection);
    let all_animals = animals
        .select((species, name, legs))
        .load::<Animal>(&connection);

    match all_animals {
        Ok(kk) => println!("所有的:{:?}", kk),
        Err(e) => println!("没有数据:{}", e),
    }

    let distinct_animals = animals
        .select((species, name, legs))
        .distinct_on(species)
        .load::<Animal>(&connection);
    // let distinct_animals = animals.select((species, name, legs)).distinct_on(species).load(&connection);
    match distinct_animals {
        Ok(d_animals) => println!("distinct_on结果：{:?}", d_animals),
        Err(e) => println!("没有数据:{}", e),
    }
}

fn animal_filter() {
    let connection = establish_connection();
    diesel::delete(animals).execute(&connection).unwrap(); //删除表数据
    diesel::insert_into(animals)
        .values(&vec![
            (species.eq("cat"), legs.eq(4), name.eq("Sinatra")),
            (species.eq("dog"), legs.eq(3), name.eq("Fido")),
            (species.eq("spider"), legs.eq(8), name.eq("Charlotte")),
        ])
        .execute(&connection)
        .unwrap();

    let good_animals = animals
        .filter(name.eq("Fido"))
        .or_filter(legs.eq(4))
        .select(name)
        .get_results::<Option<String>>(&connection);
    println!("or_filter_example:{:?}", good_animals);

    type DB = diesel::pg::Pg;
    let sql = diesel::debug_query::<DB, _>(&animals.filter(name.eq("Fido")).or_filter(legs.eq(4)).select(name))
        .to_string();
    println!("SQL:{:?}", sql);
    println!("or_filtter_example end");
}
