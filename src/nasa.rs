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
