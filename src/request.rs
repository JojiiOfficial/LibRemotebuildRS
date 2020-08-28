use crate::config::RequestConfig;
use crate::request_error::Error;

use serde::de::DeserializeOwned;
use serde::Serialize;

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
    pub fn new(config: RequestConfig, endpoint: &str, p: T) -> Self {
        Request {
            // WithAuthFromConfig with authorization
            config,
            endpoint: endpoint.to_owned(),
            payload: Some(p),
            ..Request::default()
        }
    }

    pub fn with_auth(&mut self, auth: Authorization) {
        self.auth = Some(auth);
    }

    pub fn with_method(&mut self, method: reqwest::Method) {
        self.method = method;
    }

    pub async fn do_request<U>(self) -> Result<RequestResult<U>, Error>
    where
        U: DeserializeOwned,
    {
        let mut req_builder = reqwest::Client::new().request(
            self.method,
            reqwest::Url::parse(
                String::from(format!("{}{}", self.config.url, self.endpoint)).as_str(),
            )
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

        let r = req_builder.send().await.map_err(Error::Request)?;
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

        if status == STATUS_SUCCESS {
            let response: U = r.json().await.map_err(Error::Decode)?;
            Ok(RequestResult {
                response,
                message: msg.to_string(),
                status_code: status,
            })
        } else {
            Err(Error::Error(msg))
        }
    }
}

#[derive(Debug)]
pub struct RequestResult<T> {
    pub response: T,
    pub message: String,
    pub status_code: u8,
}

#[derive(Default, Serialize)]
pub struct ListJobs {
    #[serde(rename(serialize = "l"))]
    pub limit: i32,
}