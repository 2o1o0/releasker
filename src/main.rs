mod structures;
use reqwest::header;
use reqwest::header::USER_AGENT;

use crate::structures::Release;
use crate::structures::Releases;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/releases",
        owner = "kubernetes",
        repo = "ingress-nginx"
    );
    println!("{}", request_url);
    // let response = reqwest::get(&request_url).await?;
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "2o1o0-releasker")
        .send()
        .await?;
    // let body = &response.text().await?;
    // println!("Response Body: {}", body);

    let releases: Vec<Release> = response.json().await?;
    // println!("{:#?}", releases);
    for release in releases {
        println!("{:?}", release.tag_name)
    }
    Ok(())
}
