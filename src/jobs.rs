use chrono::{DateTime, NaiveDateTime};
use serde::de::{Deserializer, Error};
use serde::Deserialize;

use std::time::Duration;

#[derive(Deserialize, Debug)]
pub enum Status {
    Waiting,
    Cancelled,
    Failed,
    Running,
    Done,
    Paused,
}

#[derive(Debug, Deserialize)]
pub enum Type {
    NoBuild,
    JobAUR,
}

#[derive(Deserialize, Debug)]
pub enum UploadType {
    NoUploadType,
    DataManager,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub id: u32,
    pub info: String,

    #[serde(rename(deserialize = "pos"))]
    pub position: u32,

    #[serde(deserialize_with = "deserialize_type", rename(deserialize = "jobtype"))]
    pub build_type: Type,

    #[serde(
        deserialize_with = "deserialize_upload_type",
        rename(deserialize = "uploadtype")
    )]
    pub upload_type: UploadType,

    #[serde(deserialize_with = "deserialize_status", rename(deserialize = "state"))]
    pub status: Status,

    #[serde(deserialize_with = "deserialize_date", rename(deserialize = "rs"))]
    pub running_since: NaiveDateTime,

    #[serde(deserialize_with = "deserialize_duration", rename(deserialize = "dr"))]
    pub duration: Duration,
}

pub fn deserialize_date<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<NaiveDateTime, D::Error> {
    Ok(
        DateTime::parse_from_rfc3339(serde::de::Deserialize::deserialize(deserializer)?)
            .map_err(D::Error::custom)?
            .naive_local(),
    )
}

pub fn deserialize_duration<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Duration, D::Error> {
    Ok(Duration::from_nanos(serde::de::Deserialize::deserialize(
        deserializer,
    )?))
}

pub fn deserialize_status<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Status, D::Error> {
    Ok(match serde::de::Deserialize::deserialize(deserializer)? {
        0 => Status::Waiting,
        1 => Status::Cancelled,
        2 => Status::Failed,
        3 => Status::Running,
        4 => Status::Done,
        5 => Status::Paused,
        _ => return Err(D::Error::custom("status not found")),
    })
}

pub fn deserialize_type<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Type, D::Error> {
    Ok(match serde::de::Deserialize::deserialize(deserializer)? {
        0 => Type::NoBuild,
        1 => Type::JobAUR,
        _ => return Err(D::Error::custom("type not found")),
    })
}

pub fn deserialize_upload_type<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<UploadType, D::Error> {
    Ok(match serde::de::Deserialize::deserialize(deserializer)? {
        0 => UploadType::NoUploadType,
        1 => UploadType::DataManager,
        _ => return Err(D::Error::custom("type not found")),
    })
}
