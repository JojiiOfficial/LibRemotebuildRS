use reqwest::Error as reqErr;

#[derive(Debug)]
pub enum Error {
    Request(reqErr),
    Decode(reqErr),
    InvalidHeaders,
    Error(String),
    InvalidState,
}
