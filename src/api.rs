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
    #[serde(default, rename = "Loop")]
    pub loop_mode: String,
}

#[derive(Deserialize, Debug)]
pub struct DeviceInfo {
    #[serde(default, rename = "DeviceName")]
    pub device_name: String,
    #[serde(default, rename = "Firmware")]
    pub firmware: String,
    #[serde(default, rename = "Hardware")]
    pub hardware: String,
    #[serde(default, rename = "Uuid")]
    pub uuid: String,
    #[serde(default, rename = "Apcli0")]
    pub wifi_ip: String,
    #[serde(default, rename = "Eth2")]
    pub eth_ip: String,
    #[serde(default, rename = "MAC")]
    pub mac: String,
    #[serde(default, rename = "Netstat")]
    pub netstat: String,
    #[serde(default, rename = "mcu_ver")]
    pub mcu_ver: String,
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

pub async fn get_device_info(client: &Client, config: &SpeakerConfig) -> Result<DeviceInfo, String> {
    let response = call(client, config, "getStatus").await?;
    serde_json::from_str(&response).map_err(|e| format!("Failed to parse device info: {}", e))
}
