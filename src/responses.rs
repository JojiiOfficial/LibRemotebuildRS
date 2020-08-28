use crate::jobs::Info;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct ListJobs {
    pub jobs: Vec<Info>,
}
