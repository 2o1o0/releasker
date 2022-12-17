extern crate serde_json;
use serde_derive::{Deserialize, Serialize};

/// A project.
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "provider")]
    pub provider: String,

    #[serde(rename = "project")]
    pub project: String,

    #[serde(rename = "owner")]
    pub owner: String,

    #[serde(rename = "include_filter")]
    pub include_filter: String,

    #[serde(rename = "allow_prerelease")]
    pub allow_prerelease: bool,
}

// "projects": {
//     "github": [
//         {
//             "owner": "kubernetes",
//             "project": "ingress-nginx",
//             "filter_must": "chart",
//             "allow_prerelease": false
//         },
