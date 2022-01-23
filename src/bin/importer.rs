use hyper::Client;
use configparser::ini::Ini;
use env_logger;
use env_logger::Env;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting up");

    // read configs
    let mut private_config = Ini::new();
    private_config.load("config/private.ini")?;
    let nasa_private_key = private_config.get("topsecrets","NASA_API_KEY").expect("could not find NASA_API_KEY");

    let client =  Client::new();
    let uri = "http://google.com".parse()?;

    let mut resp = client.get(uri).await?;

    Ok(())
}
