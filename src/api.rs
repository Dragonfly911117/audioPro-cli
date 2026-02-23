use reqwest::Client;
use serde::Deserialize;

use crate::config::SpeakerConfig;

#[derive(Deserialize, Debug)]
pub struct PlayerStatus {
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub vol: String,
    #[serde(default)]
    pub mute: String,
    #[serde(default)]
    pub eq: String,
    #[serde(default, rename = "Title")]
    pub title: String,
    #[serde(default, rename = "Artist")]
    pub artist: String,
    #[serde(default)]
    pub curpos: String,
    #[serde(default)]
    pub totlen: String,
}

pub fn build_client() -> Result<Client, String> {
    Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))
}

pub async fn call(client: &Client, config: &SpeakerConfig, command: &str) -> Result<String, String> {
    let url = format!(
        "https://{}:{}/httpapi.asp?command={}",
        config.ip, config.port, command
    );

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))
}

pub async fn get_status(client: &Client, config: &SpeakerConfig) -> Result<PlayerStatus, String> {
    let response = call(client, config, "getPlayerStatus").await?;
    serde_json::from_str(&response).map_err(|e| format!("Failed to parse status: {}", e))
}
