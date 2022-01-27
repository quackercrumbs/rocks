#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod db_util;
pub mod nasa;

pub fn helloworld() {
    println!("helloworld")
}
