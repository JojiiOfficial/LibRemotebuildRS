use crate::{aur, config, endpoints, jobs, request, request_error, responses};

use request::{Request, RequestResult};
use std::collections::HashMap;

pub struct LibRb {
    config: config::RequestConfig,
}

pub fn new(config: config::RequestConfig) -> LibRb {
    LibRb { config }
}

impl LibRb {
    pub fn new_aurbuild<S: AsRef<str>>(&self, pkg_name: S) -> aur::AURBuild {
        let mut hm: HashMap<String, String> = HashMap::new();
        hm.insert(aur::AUR_PACKAGE.to_owned(), pkg_name.as_ref().to_owned());

        aur::AURBuild {
            librb: self,
            args: hm,
            upload_type: jobs::UploadType::NoUploadType,
            disable_ccache: false,
        }
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

    pub async fn cancel_job(&self, job_id: u32) -> Result<(), request_error::Error> {
        let mut request = Request::new(
            self.config.clone(),
            endpoints::JOBCANCEL,
            request::JobRequest { job_id },
        );

        request.with_auth(self.auth_from_conf());
        request.with_method(reqwest::Method::POST);
        request.do_request_void().await?;

        Ok(())
    }

    pub async fn add_job(
        &self,
        job_type: jobs::Type,
        upload_type: jobs::UploadType,
        args: HashMap<String, String>,
        disable_ccache: bool,
    ) -> Result<RequestResult<responses::AddJob>, request_error::Error> {
        let mut request = Request::new(
            self.config.clone(),
            endpoints::JOBADD,
            request::AddJobRequest {
                args,
                upload_type,
                disable_ccache,
                job_type,
            },
        );

        request.with_auth(self.auth_from_conf());
        request.with_method(reqwest::Method::PUT);
        Ok(request.do_request().await?)
    }

    pub async fn set_job_state(
        &self,
        job_id: u32,
        state: jobs::Status,
    ) -> Result<(), request_error::Error> {
        let ep = match state {
            jobs::Status::Paused => endpoints::JOBPAUSE,
            jobs::Status::Running => endpoints::JOBRESUME,
            _ => return Err(request_error::Error::InvalidState),
        };

        let mut request = Request::new(self.config.clone(), ep, request::JobRequest { job_id });

        request.with_auth(self.auth_from_conf());
        request.with_method(reqwest::Method::PUT);
        request.do_request_void().await?;

        Ok(())
    }
}
