use crate::config::RequestConfig;
use crate::jobs;
use crate::request_error::Error;

use serde::de::DeserializeOwned;
use serde::Serialize;

use std::collections::HashMap;

const HEADER_RESPONSE_STATUS: &str = "x-response-status";
const HEADER_RESPONSE_MESSAGE: &str = "x-response-message";
const STATUS_SUCCESS: u8 = 1;
const STATUS_ERROR: u8 = 0;

#[derive(Default)]
pub struct Request<T>
where
    T: Serialize,
{
    config: RequestConfig,
    endpoint: String,
    method: reqwest::Method,
    auth: Option<Authorization>,
    payload: Option<T>,
}

#[derive(Debug)]
pub enum AuthorizationType {
    Bearer,
    Basic,
}

impl std::fmt::Display for AuthorizationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Authorization {
    auth_type: AuthorizationType,
    payload: String,
}

impl Authorization {
    pub fn new(auth_type: AuthorizationType, payload: String) -> Self {
        Authorization { auth_type, payload }
    }
}

impl<T> Request<T>
where
    T: Serialize + Default,
{
    /// Create a new request by using a config, the desired endpoint and a Serializeable value T which will
    /// be formatted to json
    pub fn new(config: RequestConfig, endpoint: &str, p: T) -> Self {
        Request {
            // WithAuthFromConfig with authorization
            config,
            endpoint: endpoint.to_owned(),
            payload: Some(p),
            ..Request::default()
        }
    }

    /// Use the given authorization for the request
    pub fn with_auth(&mut self, auth: Authorization) {
        self.auth = Some(auth);
    }

    /// Use the given HTTP Method for the request
    pub fn with_method(&mut self, method: reqwest::Method) {
        self.method = method;
    }

    /// Prepare and send a request. Don't do any parsing of the results body.
    async fn prepare_response(self) -> Result<(reqwest::Response, String, u8), Error> {
        let mut req_builder = reqwest::Client::new().request(
            self.method,
            reqwest::Url::parse(&self.config.url)
                .unwrap()
                .join(&self.endpoint)
                .unwrap(),
        );

        // Add payload
        if let Some(p) = &self.payload {
            req_builder = req_builder.json(p);
        }

        // Add Auth
        if let Some(auth) = self.auth {
            req_builder = req_builder.header(
                "Authorization",
                format!("{} {}", auth.auth_type, auth.payload),
            );
        }

        // Send the request
        let r = req_builder.send().await.map_err(Error::Request)?;
        let status = r.status().as_u16();
        if status != 200 {
            return Err(Error::HTTPNotOk(status));
        }

        let headers = r.headers();
        if !headers.contains_key(HEADER_RESPONSE_STATUS)
            || !headers.contains_key(HEADER_RESPONSE_MESSAGE)
        {
            return Err(Error::InvalidHeaders);
        }

        let msg = headers
            .get(HEADER_RESPONSE_MESSAGE)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let status: u8 = headers
            .get(HEADER_RESPONSE_STATUS)
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .map_err(|_| Error::InvalidHeaders)?;

        Ok((r, msg, status))
    }

    /// Do a request but don't parse the body
    pub async fn do_request_void(self) -> Result<(), Error> {
        let (_, msg, status) = self.prepare_response().await?;

        if status == STATUS_SUCCESS {
            Ok(())
        } else {
            Err(Error::Error(msg))
        }
    }

    /// Do a request but try to parse the body into a RequestResult
    pub async fn do_request<U>(self) -> Result<RequestResult<U>, Error>
    where
        U: DeserializeOwned,
    {
        let (r, msg, status) = self.prepare_response().await?;

        if status == STATUS_SUCCESS {
            let response: U = r.json().await.map_err(Error::Decode)?;

            Ok(RequestResult {
                response: Some(response),
                message: msg.to_string(),
                status_code: status,
            })
        } else {
            Err(Error::Error(msg))
        }
    }
}

/// Result for a request. T represents
/// The destination type to parse the
/// result in.
#[derive(Debug)]
pub struct RequestResult<T>
where
    T: DeserializeOwned,
{
    pub response: Option<T>,
    pub message: String,
    pub status_code: u8,
}

/// Lists all jobs with a given limit
#[derive(Default, Serialize)]
pub struct ListJobs {
    #[serde(rename(serialize = "l"))]
    pub limit: i32,
}

/// Payload for requests with need
/// a reference to a job by its id
#[derive(Default, Serialize)]
pub struct JobRequest {
    #[serde(rename(serialize = "id"))]
    pub job_id: u32,
}

/// Payload for creating a new Job
/// args - additional arguments
#[derive(Default, Serialize)]
pub struct AddJobRequest {
    #[serde(rename(serialize = "buildtype"))]
    pub job_type: jobs::Type,

    pub args: HashMap<String, String>,

    #[serde(rename(serialize = "uploadtype"))]
    pub upload_type: jobs::UploadType,

    #[serde(rename(serialize = "disableccache"))]
    pub disable_ccache: bool,
}

/// Payload for requests which need
/// user credentials
#[derive(Serialize, Debug, Default)]
pub struct Credential {
    #[serde(rename(serialize = "mid"))]
    pub machine_id: String,

    pub username: String,

    #[serde(rename(serialize = "pass"))]
    pub password: String,
}
