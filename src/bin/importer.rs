use hyper::Client;
use hyper::body;
use hyper_tls::HttpsConnector;
use configparser::ini::Ini;
use env_logger;
use env_logger::Env;
use std::collections::HashMap;

#[macro_use]
extern crate log;

use serde_json;
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
struct NearEarthObjectResponse {
    links: Links,
    element_count: usize,
    near_earth_objects: HashMap<String, Vec<NearEarthObject>>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting up");

    // read configs
    let mut private_config = Ini::new();
    private_config.load("config/private.ini")?;
    let nasa_api_key = private_config.get("topsecrets","NASA_API_KEY").expect("could not find NASA_API_KEY");

    let https = HttpsConnector::new();
    let client =  Client::builder().build::<_, hyper::Body>(https);
    let asteroid_uri = format!("https://api.nasa.gov/neo/rest/v1/feed?start_date=2015-09-07&end_date=2015-09-08&api_key={}", nasa_api_key).parse()?;

    info!("Making API call to {:?}", asteroid_uri);
    let mut resp = client.get(asteroid_uri).await?;
    debug!("response: {:?}", resp);
    let body = resp.body_mut();
    let body_bytes = body::to_bytes(body).await?;
    trace!("response body: {:?}", body_bytes);

    let asteroid_data : NearEarthObjectResponse = serde_json::from_slice(&body_bytes).unwrap();
    info!("asteroid_data: {:?}", asteroid_data);

    Ok(())
}
