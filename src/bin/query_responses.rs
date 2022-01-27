extern crate rocks;
use rocks::schema::api_response::dsl;

use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel;
use configparser::ini::Ini;
use env_logger;
use env_logger::Env;

#[macro_use]
extern crate log;

use serde_json;

mod nasa_models {
    use std::collections::HashMap;
    use serde::{Serialize,Deserialize};
    #[derive(Serialize,Deserialize,Debug)]
    struct Links {
      next: Option<String>,
      prev: Option<String>,
      #[serde(rename = "self")]
      self_link: Option<String>
    }
    #[derive(Serialize,Deserialize,Debug)]
    struct EstimatedDiameter {
        estimated_diameter_min: f64,
        estimated_diameter_max: f64
    }
    #[derive(Serialize,Deserialize,Debug)]
    struct RelativeVelocity {
        kilometers_per_second: String,
        kilometers_per_hour: String,
        miles_per_hour: String,
    }
    #[derive(Serialize,Deserialize,Debug)]
    struct MissDistance {
        astronomical: String,
        lunar: String,
        kilometers: String,
        miles: String,
    }
    #[derive(Serialize,Deserialize,Debug)]
    struct CloseApproachEvent {
        close_approach_date: String,
        close_approach_date_full: String,
        epoch_date_close_approach: usize,
        relative_velocity: RelativeVelocity,
        miss_distance: MissDistance,
        orbiting_body: String,
    }
    #[derive(Serialize,Deserialize,Debug)]
    struct NearEarthObject {
        links: Links,
        id: String,
        neo_reference_id: String,
        nasa_jpl_url: String,
        absolute_magnitude_h: f64,
        // change string to some enum?
        estimated_diameter: HashMap<String, EstimatedDiameter>,
        close_approach_data: Vec<CloseApproachEvent>,
        is_sentry_object: bool
    }
    #[derive(Serialize,Deserialize,Debug)]
    pub struct NearEarthObjectResponse {
        links: Links,
        element_count: usize,
        near_earth_objects: HashMap<String, Vec<NearEarthObject>>
    }
}

mod db {
    use diesel::prelude::*;
    pub fn establish_connnection(database_url: &str) -> SqliteConnection {
        SqliteConnection::establish(database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting up");

    // read configs
    let mut private_config = Ini::new();
    private_config.load("config/private.ini")?;
    let database_url = private_config.get("topsecrets","DATABASE_URL").expect("could not find DATABASE_URL");

    // initialize db connection
    let connection = db::establish_connnection(&database_url);
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
        let asteroid_response = serde_json::from_str::<nasa_models::NearEarthObjectResponse>(&record.response).expect("Could not serialize response");
        info!("serialized={:?}", asteroid_response);
    }
}


