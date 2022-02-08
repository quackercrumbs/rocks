pub mod models {
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

pub mod client {
    use hyper::{Client, client::HttpConnector, body};
    use hyper_tls::HttpsConnector;

    use super::models::NearEarthObjectResponse;

    pub struct NearEarthObjectClient {
        api_key: String,
        client: Client<HttpsConnector<HttpConnector>>
    }

    impl NearEarthObjectClient {
        pub fn new(api_key: &str) -> Self {
            let https = HttpsConnector::new();
            let client =  Client::builder().build::<_, hyper::Body>(https);
            Self {
                api_key: String::from(api_key),
                client
            }
        }

        pub async fn get_near_earth_objects(&self, start_date: &str, end_date: &str) -> Result<NearEarthObjectResponse, Box<dyn std::error::Error + Send + Sync>> {
            let asteroid_uri = format!("https://api.nasa.gov/neo/rest/v1/feed?start_date={}&end_date={}&api_key={}", start_date, end_date, self.api_key).parse()?;
            let mut resp = self.client.get(asteroid_uri).await?;
            let body = resp.body_mut();
            let body_bytes = body::to_bytes(body).await?;
            let asteroid_data: NearEarthObjectResponse = serde_json::from_slice(&body_bytes).unwrap();
            Ok(asteroid_data)
        }
    }
}