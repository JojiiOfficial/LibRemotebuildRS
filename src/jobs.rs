use chrono::{DateTime, NaiveDateTime};
use serde::de::{Deserializer, Error as DError};
use serde::ser::Serializer;
use serde::Deserialize;

use std::time::Duration;

/// The status for a job
#[derive(Debug)]
pub enum Status {
    Waiting,
    Cancelled,
    Failed,
    Running,
    Done,
    Paused,
}

impl<'de> serde::Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
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
}

impl serde::Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(match self {
            Status::Waiting => 0,
            Status::Cancelled => 1,
            Status::Failed => 2,
            Status::Running => 3,
            Status::Done => 4,
            Status::Paused => 5,
        })
    }
}

/// Type of a job
#[derive(Debug)]
pub enum Type {
    NoBuild,
    JobAUR,
}

impl Default for Type {
    fn default() -> Self {
        Type::NoBuild
    }
}

impl<'de> serde::Deserialize<'de> for Type {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Type, D::Error> {
        Ok(match serde::de::Deserialize::deserialize(deserializer)? {
            0 => Type::NoBuild,
            1 => Type::JobAUR,
            _ => return Err(D::Error::custom("type not found")),
        })
    }
}

impl serde::Serialize for Type {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_u8(match self {
            Type::NoBuild => 0,
            Type::JobAUR => 1,
        })
    }
}

/// Upload type of a job
#[derive(Debug)]
pub enum UploadType {
    NoUploadType,
    DataManager,
}

impl Default for UploadType {
    fn default() -> Self {
        UploadType::NoUploadType
    }
}

impl<'de> serde::Deserialize<'de> for UploadType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match serde::de::Deserialize::deserialize(deserializer)? {
            0 => UploadType::NoUploadType,
            1 => UploadType::DataManager,
            _ => return Err(D::Error::custom("type not found")),
        })
    }
}

impl serde::Serialize for UploadType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(match self {
            UploadType::NoUploadType => 0,
            UploadType::DataManager => 1,
        })
    }
}

/// Infos about a job
#[derive(Debug, Deserialize)]
pub struct Info {
    pub id: u32,
    pub info: String,

    #[serde(rename(deserialize = "pos"))]
    pub position: u32,

    #[serde(rename(deserialize = "jobtype"))]
    pub build_type: Type,

    #[serde(rename(deserialize = "uploadtype"))]
    pub upload_type: UploadType,

    #[serde(rename(deserialize = "state"))]
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
