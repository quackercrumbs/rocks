#[macro_use]
extern crate diesel;

use diesel::prelude::*;
pub mod schema;
pub mod models;

pub fn helloworld() {
    println!("helloworld")
}
