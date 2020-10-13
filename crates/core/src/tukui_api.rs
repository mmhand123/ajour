use crate::{addon::Addon, config::Flavor, error::ClientError, network::request_async, Result};
use isahc::config::RedirectPolicy;
use isahc::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize)]
/// Struct for applying tukui details to an `Addon`.
pub struct TukuiPackage {
    pub name: String,
    pub version: String,
    pub url: String,
    pub web_url: String,
    pub lastupdate: String,
    pub patch: Option<String>,
    pub author: Option<String>,
    pub small_desc: Option<String>,
}

/// Return the tukui API endpoint.
fn api_endpoint(id: &str, flavor: &Flavor) -> String {
    match flavor {
        Flavor::Retail => match id {
            "-1" => "https://www.tukui.org/api.php?ui=tukui".to_owned(),
            "-2" => "https://www.tukui.org/api.php?ui=elvui".to_owned(),
            _ => format!("https://www.tukui.org/api.php?addon={}", id),
        },
        Flavor::Classic => format!("https://www.tukui.org/api.php?classic-addon={}", id),
    }
}

/// Function to fetch a remote addon package which contains
/// information about the addon on the repository.
pub async fn fetch_remote_package(id: &str, flavor: &Flavor) -> Result<TukuiPackage> {
    let client = HttpClient::builder()
        .redirect_policy(RedirectPolicy::Follow)
        .max_connections_per_host(6)
        .build()
        .unwrap();
    let url = api_endpoint(id, flavor);
    let timeout = Some(30);
    let mut resp = request_async(&client, &url, vec![], timeout).await?;

    if resp.status().is_success() {
        let package = resp.json()?;
        Ok(package)
    } else {
        Err(ClientError::Custom(format!(
            "Couldn't fetch details for addon. Server returned: {}",
            resp.text()?
        )))
    }
}

pub async fn latest_package(tukui_id: u32, flavor: Flavor) -> Result<(u32, Flavor, TukuiPackage)> {
    let tukui_id_string = tukui_id.to_string();

    let package = fetch_remote_package(&tukui_id_string, &flavor).await?;

    Ok((tukui_id, flavor, package))
}
