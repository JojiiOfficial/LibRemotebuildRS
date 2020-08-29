use lib_remotebuild_rs::*;

#[tokio::main]
async fn main() {
    let librb = librb::new(config::RequestConfig {
        machine_id: "".to_owned(),
        token: "".to_owned(),
        url: "".to_owned(),
        username: "".to_owned(),
    });

    let job_info = librb.job_info(179).await.unwrap();
    println!("{:#?}", job_info);
}
