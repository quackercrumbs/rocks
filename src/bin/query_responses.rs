extern crate rocks;
use rocks::schema::api_response::dsl;
use rocks::db_util;
use rocks::nasa;

use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel;
use configparser::ini::Ini;
use env_logger;
use env_logger::Env;

#[macro_use]
extern crate log;

use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting up");

    // read configs
    let mut private_config = Ini::new();
    private_config.load("config/private.ini")?;
    let database_url = private_config.get("topsecrets","DATABASE_URL").expect("could not find DATABASE_URL");

    // initialize db connection
    let connection = db_util::establish_connnection(&database_url);
    info!("Connected to database");

    // Read NASA data from db
    read_asteroid_api_responses(&connection).await;
    info!("Finished");
    Ok(())
}


async fn read_asteroid_api_responses(connection: &SqliteConnection) {
    let results = dsl::api_response.load::<rocks::models::ApiResponse>(connection).expect("Could not read all asteroid records.");
    for record in results {
        debug!("response={:?}", record);

        // map raw responses onto 
        let asteroid_response = serde_json::from_str::<nasa::models::NearEarthObjectResponse>(&record.response).expect("Could not serialize response");
        info!("serialized={:?}", asteroid_response);
    }
}


