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

pub struct AURBuild {
    pub librb: librb::LibRb,
    pub args: HashMap<String, String>,
    pub upload_type: UploadType,
    pub disable_ccache: bool,
}

impl AURBuild {
    pub fn without_ccache(mut self) -> Self {
        self.disable_ccache = true;
        self
    }

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

        if namespace.len() > 0 {
            self.args.insert(DM_NAMESPACE.to_owned(), namespace);
        }

        self
    }

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
