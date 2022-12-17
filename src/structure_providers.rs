extern crate serde_json;
use serde_derive::{Deserialize, Serialize};

/// A provider.
#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "url")]
    pub url: String,
}
