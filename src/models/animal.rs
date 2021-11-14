use diesel::prelude::*;

use crate::schema::animals::dsl::*;
use diesel::prelude::*;
use crate::pool_connection;

#[derive(Debug, Queryable, PartialEq)]
pub struct Animal {
    species: String,
    name: Option<String>,
    legs: i32,
}

impl Animal {
    fn new<S: Into<String>>(special: S, named: Option<&str>, leg: i32) -> Self {
        Animal {
            species: special.into(),
            name: named.map(Into::into),
            legs: leg,
        }
    }
}

