#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod db_util;

pub fn helloworld() {
    println!("helloworld")
}
