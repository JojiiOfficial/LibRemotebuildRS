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
    /// Return a new AURBuild which allows you to create build AUR jobs
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

    /// Return Authorization created from `self`s config
    pub fn auth_from_conf(&self) -> request::Authorization {
        request::Authorization::new(
            request::AuthorizationType::Bearer,
            self.config.token.to_owned(),
        )
    }

    /// List all running and past jobs. `limit` indicates the limit how
    /// much to display
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

    /// Cancel a running job
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

    /// Gets information about a job
    pub async fn job_info(
        &self,
        job_id: u32,
    ) -> Result<RequestResult<jobs::Info>, request_error::Error> {
        let mut request = Request::new(
            self.config.clone(),
            endpoints::JOBINFO,
            request::JobRequest { job_id },
        );

        request.with_auth(self.auth_from_conf());
        request.with_method(reqwest::Method::GET);
        Ok(request.do_request().await?)
    }

    /// Creates and adds a new job
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

    /// Set a jobs state either to `paused` or `running`
    /// This allows to pause/continue a task
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

    /// Login into an existing account. Returns the token on success
    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<RequestResult<responses::Login>, request_error::Error> {
        let mut request = Request::new(
            self.config.clone(),
            endpoints::LOGIN,
            request::Credential {
                machine_id: self.config.machine_id.clone(),
                username,
                password,
            },
        );

        request.with_method(reqwest::Method::POST);
        Ok(request.do_request().await?)
    }
}
