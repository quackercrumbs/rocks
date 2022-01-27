use diesel::prelude::*;
use diesel::SqliteConnection;

pub fn establish_connnection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
