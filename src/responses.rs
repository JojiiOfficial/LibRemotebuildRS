use crate::jobs::Info;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct ListJobs {
    pub jobs: Vec<Info>,
}

#[derive(Deserialize, Debug)]
pub struct AddJob {
    pub id: u32,

    #[serde(rename(deserialize = "pos"))]
    pub position: i32,
}

#[derive(Deserialize, Debug)]
pub struct Login {
    pub token: String,
}
