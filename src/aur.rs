use crate::{
    jobs::{Type, UploadType},
    librb, request, request_error, responses,
};

use std::collections::HashMap;

pub const DM_TOKEN: &str = "DM_Token"; // DMToken session token for DManager
pub const DM_USER: &str = "DM_USER"; // DMUser username for DManger
pub const DM_HOST: &str = "DM_HOST"; // DMHost Dmanager host
pub const DM_NAMESPACE: &str = "DM_NAMESPACE"; // DMHost Dmanager host
pub const AUR_PACKAGE: &str = "REPO"; // AUR package to build

/// Stores data relevant for AUR build jobs
pub struct AURBuild<'a> {
    pub librb: &'a librb::LibRb,
    pub args: HashMap<String, String>,
    pub upload_type: UploadType,
    pub disable_ccache: bool,
}

impl<'a> AURBuild<'a> {
    /// Turn of ccache usage
    pub fn without_ccache(mut self) -> Self {
        self.disable_ccache = true;
        self
    }

    /// Add dmanager upload data to the job
    pub fn with_dmanager(
        mut self,
        username: String,
        token: String,
        host: String,
        namespace: String,
    ) -> Self {
        self.upload_type = UploadType::DataManager;
        self.args.insert(DM_TOKEN.to_owned(), token);
        self.args.insert(DM_USER.to_owned(), username);
        self.args.insert(DM_HOST.to_owned(), host);

        if !namespace.is_empty() {
            self.args.insert(DM_NAMESPACE.to_owned(), namespace);
        }

        self
    }

    /// Create a new AUR build job
    pub async fn create_job(
        self,
    ) -> Result<request::RequestResult<responses::AddJob>, request_error::Error> {
        self.librb
            .add_job(
                Type::JobAUR,
                self.upload_type,
                self.args,
                self.disable_ccache,
            )
            .await
    }
}
