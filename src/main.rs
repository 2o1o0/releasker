mod structure_projects;
mod structure_providers;
mod structures;

use std::process::exit;

use reqwest::header::ACCEPT;
use reqwest::header::USER_AGENT;

use crate::structure_projects::Project;
use crate::structure_providers::Provider;
use crate::structures::Release;
// use surrealdb_rs::param::Root;
// use surrealdb_rs::protocol::Ws;
// use surrealdb_rs::{Result, Surreal};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let db_hostname = "database";
    // let db_port = 8000;
    // let db_connection_string = format!(
    //     "{hostname}:{port}",
    //     hostname = db_hostname,
    //     port = String::from(db_port)
    // );
    // let client = Surreal::connect::<Ws>("localhost:8000").await?;
    let client = reqwest::Client::new();

    let providers = [
        Provider {
            name: String::from("github"),
            url: String::from("https://api.github.com"),
        },
        Provider {
            name: String::from("dockerhub"),
            url: String::from("https://api.github.com"),
        },
    ];

    let projects = [
        Project {
            provider: String::from("github"),
            owner: String::from("rancher"),
            project: String::from("rancher"),
            allow_prerelease: false,
            include_filter: String::from(""),
        },
        Project {
            provider: String::from("github"),
            owner: String::from("kubernetes"),
            project: String::from("ingress-nginx"),
            allow_prerelease: false,
            include_filter: String::from("helm-chart"),
        },
    ];

    for project in projects {
        for provider in providers.iter() {
            if project.provider == provider.name {
                let mut request_url: String;
                let mut response: reqwest::Response;

                if provider.name == "github" {
                    request_url = format!(
                        "{url}/repos/{owner}/{repo}/releases",
                        url = provider.url,
                        owner = project.owner,
                        repo = project.project
                    );

                    response = client
                        .get(&request_url)
                        .header(USER_AGENT, "2o1o0-releasker")
                        .header(ACCEPT, "application/vnd.github+json")
                        .send()
                        .await?;
                } else {
                    exit(1)
                };

                println!("{}", request_url);

                let releases: Vec<Release> = response.json().await?;

                for release in releases {
                    if release.tag_name.contains(&project.include_filter)
                        && project.allow_prerelease >= release.prerelease
                    {
                        println!("{:?}", release.tag_name);
                    }
                }
            }
        }
    }

    Ok(())
}
