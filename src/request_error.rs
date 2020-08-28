use reqwest::Error as reqErr;

#[derive(Debug)]
pub enum Error {
    Unexpected,
    Request(reqErr),
    Decode(reqErr),
    InvalidHeaders,
    Error(String),
    InvalidState,
}
