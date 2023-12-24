use chrono::{DateTime, Utc};
use little_walk_auth::core::entities::Profile;
use little_walk_dog::core::entities::Dog;

pub struct Invitation {
    pub id: String,
    pub dogs: Vec<Dog>,
    pub initiator: Profile,
    pub latitude: f64,
    pub longitude: f64,
    pub start_at: DateTime<Utc>,
    pub canceled_at: Option<DateTime<Utc>>,
}
