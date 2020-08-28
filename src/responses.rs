use crate::jobs::Info;
use serde::Deserialize;

/// List of jobs from the past
#[derive(Deserialize, Debug, Default)]
pub struct ListJobs {
    pub jobs: Vec<Info>,
}

/// Response for adding a new job
#[derive(Deserialize, Debug)]
pub struct AddJob {
    pub id: u32,

    #[serde(rename(deserialize = "pos"))]
    pub position: i32,
}

/// Login response. Containing the
/// newly created session token
#[derive(Deserialize, Debug)]
pub struct Login {
    pub token: String,
}
