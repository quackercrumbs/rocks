use hyper::Client;
use hyper::body;
use hyper_tls::HttpsConnector;
use configparser::ini::Ini;
use env_logger;
use env_logger::Env;

#[macro_use]
extern crate log;

use serde_json;
use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize,Debug)]
struct Links {
  next: String,
  prev: String,
}
#[derive(Serialize,Deserialize,Debug)]
struct AsteroidData {
    links: Links,
    element_count: usize,
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
    info!("response: {:?}", resp);
    let body = resp.body_mut();
    let body_bytes = body::to_bytes(body).await?;
    info!("response body: {:?}", body_bytes);

    let asteroid_data : AsteroidData = serde_json::from_slice(&body_bytes).unwrap();
    info!("asteroid_data: {:?}", asteroid_data);

    Ok(())
}
