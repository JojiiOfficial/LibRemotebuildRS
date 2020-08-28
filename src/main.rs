#![allow(dead_code)]

mod config;
mod endpoints;
mod jobs;
mod request;
mod request_error;
mod responses;

use request::{Request, RequestResult};

#[tokio::main]
async fn main() {
    let config = config::RequestConfig {
        url: "".to_owned(),
        machine_id: "".to_owned(),
        token: "".to_owned(),
        username: "".to_owned(),
    };

    let librb = LibRb::new(config);

    let res = librb.list_jobs(10).await.unwrap();

    for i in res.response.jobs {
        println!("{:?}", i);
    }
}

struct LibRb {
    config: config::RequestConfig,
}

impl LibRb {
    pub fn new(config: config::RequestConfig) -> Self {
        LibRb { config }
    }

    pub fn auth_from_conf(&self) -> request::Authorization {
        request::Authorization::new(
            request::AuthorizationType::Bearer,
            self.config.token.to_owned(),
        )
    }

    pub async fn list_jobs(
        &self,
        limit: i32,
    ) -> Result<RequestResult<responses::ListJobs>, request_error::Error> {
        let mut request = Request::new(
            self.config.clone(),
            endpoints::JOBS,
            request::ListJobs { limit },
        );

        request.with_auth(self.auth_from_conf());
        Ok(request.do_request().await?)
    }
}
