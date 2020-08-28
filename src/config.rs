#[derive(Debug, Default, Clone)]
pub struct RequestConfig {
    /// URL to the remote build server
    pub url: String,
    /// Local machine_id from /etc/machine-id
    pub machine_id: String,
    /// Remotebuild username
    pub username: String,
    /// SessionToken for authorizing
    pub token: String,
}
