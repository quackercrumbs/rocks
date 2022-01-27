extern crate rocks;
use rocks::schema::api_response::dsl;

use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel;
use hyper::Client;
use hyper::body;
use hyper::client::connect::HttpConnector;
use hyper_tls::HttpsConnector;
use configparser::ini::Ini;
use env_logger;
use env_logger::Env;

#[macro_use]
extern crate log;

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
    let nasa_api_key = private_config.get("topsecrets","NASA_API_KEY").expect("could not find NASA_API_KEY");
    let database_url = private_config.get("topsecrets","DATABASE_URL").expect("could not find DATABASE_URL");

    // initialize db connection
    let connection = db::establish_connnection(&database_url);
    info!("Connected to database");

    // initialize API client
    let https = HttpsConnector::new();
    let client =  Client::builder().build::<_, hyper::Body>(https);

    // Retrieve NASA data
    let start_date = "2015-09-07";
    let end_date = "2015-09-08";
    let asteroid_data = retrieve_asteroid_data(client, start_date, end_date, &nasa_api_key).await?;
    debug!("asteroid_data: {:?}", asteroid_data);

    // Load NASA data onto db
    load_asteroid_api_response(&connection, start_date, end_date, &asteroid_data).await;

    Ok(())
}

async fn retrieve_asteroid_data(client: Client<HttpsConnector<HttpConnector>>, start_date: &str, end_date: &str, api_key: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let asteroid_uri = format!("https://api.nasa.gov/neo/rest/v1/feed?start_date={}&end_date={}&api_key={}", start_date, end_date, api_key).parse()?;
    info!("Making API call to {:?}", asteroid_uri);
    let resp = client.get(asteroid_uri).await?;
    trace!("response: {:?}", resp);
    let body_bytes = body::to_bytes(resp.into_body()).await?;
    let body_string: String = String::from_utf8(body_bytes.to_vec()).expect("Could not convert response body to string");
    debug!("body_string: {:?}", body_string);

    Ok(body_string)
}


async fn load_asteroid_api_response(connection: &SqliteConnection, start_date: &str, end_date: &str, response: &str) {
   info!("Saving asteroid response for start_date={} end_date={}", start_date, end_date); 
    use rocks::schema::api_response;
    let new_api_response = rocks::models::NewApiResponse {
        start_date,
        end_date,
        response
    };
    diesel::insert_into(api_response::table).values(&new_api_response).execute(connection).expect("Unable to insert API response");
}

