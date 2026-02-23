use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct SpeakerConfig {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

pub fn load(custom_path: Option<PathBuf>) -> Result<SpeakerConfig, String> {
    let paths = if let Some(p) = custom_path {
        vec![p]
    } else {
        let mut paths = vec![PathBuf::from("speaker.json")];
        if let Some(home) = home_dir() {
            paths.push(home.join(".config/audiopro/speaker.json"));
        }
        paths
    };

    for path in &paths {
        if path.exists() {
            let content = std::fs::read_to_string(path)
                .map_err(|e| format!("Failed to read config: {}", e))?;
            return serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse config: {}", e));
        }
    }

    Err("Config not found. Create speaker.json with: {\"name\": \"Speaker\", \"ip\": \"192.168.1.x\", \"port\": 443}".to_string())
}

fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}
