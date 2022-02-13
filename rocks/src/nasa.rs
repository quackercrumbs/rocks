pub mod models {
    use std::collections::HashMap;
    use serde::{Serialize,Deserialize};
    #[derive(Serialize,Deserialize,Debug)]
    pub struct Links {
      pub next: Option<String>,
      pub prev: Option<String>,
      #[serde(rename = "self")]
      pub self_link: Option<String>
    }
    #[derive(Serialize,Deserialize,Debug)]
    pub struct EstimatedDiameter {
        pub estimated_diameter_min: f64,
        pub estimated_diameter_max: f64
    }
    #[derive(Serialize,Deserialize,Debug)]
    pub struct RelativeVelocity {
        pub kilometers_per_second: String,
        pub kilometers_per_hour: String,
        pub miles_per_hour: String,
    }
    #[derive(Serialize,Deserialize,Debug)]
    pub struct MissDistance {
        pub astronomical: String,
        pub lunar: String,
        pub kilometers: String,
        pub miles: String,
    }
    #[derive(Serialize,Deserialize,Debug)]
    pub struct CloseApproachEvent {
        pub close_approach_date: String,
        pub close_approach_date_full: String,
        pub epoch_date_close_approach: usize,
        pub relative_velocity: RelativeVelocity,
        pub miss_distance: MissDistance,
        pub orbiting_body: String,
    }
    #[derive(Serialize,Deserialize,Debug)]
    pub struct NearEarthObject {
        pub links: Links,
        pub id: String,
        pub neo_reference_id: String,
        pub nasa_jpl_url: String,
        pub absolute_magnitude_h: f64,
        // change string to some enum?
        pub estimated_diameter: HashMap<String, EstimatedDiameter>,
        pub close_approach_data: Vec<CloseApproachEvent>,
        pub is_sentry_object: bool
    }
    #[derive(Serialize,Deserialize,Debug)]
    pub struct NearEarthObjectResponse {
        pub links: Links,
        pub element_count: usize,
        pub near_earth_objects: HashMap<String, Vec<NearEarthObject>>
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
