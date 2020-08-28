#![allow(dead_code)]

mod aur;
mod config;
mod endpoints;
mod jobs;
mod librb;
mod request;
mod request_error;
mod responses;

#[tokio::main]
async fn main() {
    let _librb = librb::new(config);
}
