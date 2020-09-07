use reqwest::Error as reqErr;

#[derive(Debug)]
pub enum Error {
    Request(reqErr),
    Decode(reqErr),
    InvalidHeaders,
    Error(String),
    InvalidState,
    HTTPNotOk(u16),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
