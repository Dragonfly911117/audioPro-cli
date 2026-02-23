use reqwest::Client;

use crate::api::{self, get_status};
use crate::config::SpeakerConfig;
use crate::constants::{eq_presets, mode_map, source_to_mode};
use crate::utils::{decode_hex, format_time};

pub async fn status(client: &Client, config: &SpeakerConfig) -> Result<(), String> {
    let status = get_status(client, config).await?;
    let modes = mode_map();
    let presets = eq_presets();

    let source = modes
        .get(status.mode.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| status.mode.clone());
    let eq_name = presets
        .iter()
        .find(|(k, _)| *k == status.eq)
        .map(|(_, v)| *v)
        .unwrap_or("Unknown");

    let muted = status.mute == "1";
    let title = decode_hex(&status.title);
    let artist = decode_hex(&status.artist);

    println!("{}", config.name);
    println!("  Source:   {}", source.to_uppercase());
    println!("  Status:   {}", status.status);
    println!(
        "  Volume:   {}{}",
        status.vol,
        if muted { " (muted)" } else { "" }
    );
    println!("  EQ:       {}", eq_name);

    if !title.is_empty() || !artist.is_empty() {
        println!();
        if !title.is_empty() {
            println!("  Track:    {}", title);
        }
        if !artist.is_empty() {
            println!("  Artist:   {}", artist);
        }
        if !status.totlen.is_empty() && status.totlen != "0" {
            println!(
                "  Time:     {} / {}",
                format_time(&status.curpos),
                format_time(&status.totlen)
            );
        }
    }

    Ok(())
}

pub async fn volume(client: &Client, config: &SpeakerConfig, level: &str) -> Result<(), String> {
    let new_vol = if level.starts_with('+') || level.starts_with('-') {
        let status = get_status(client, config).await?;
        let current: i32 = status.vol.parse().unwrap_or(50);
        let delta: i32 = level.parse().map_err(|_| "Invalid volume adjustment")?;
        (current + delta).clamp(0, 100)
    } else {
        level
            .parse::<i32>()
            .map_err(|_| "Invalid volume level")?
            .clamp(0, 100)
    };

    api::call(client, config, &format!("setPlayerCmd:vol:{}", new_vol)).await?;
    println!("Volume: {}", new_vol);
    Ok(())
}

pub async fn mute(client: &Client, config: &SpeakerConfig) -> Result<(), String> {
    let status = get_status(client, config).await?;
    let new_mute = if status.mute == "1" { 0 } else { 1 };
    api::call(client, config, &format!("setPlayerCmd:mute:{}", new_mute)).await?;
    println!("{}", if new_mute == 1 { "Muted" } else { "Unmuted" });
    Ok(())
}

pub async fn playback(client: &Client, config: &SpeakerConfig, cmd: &str) -> Result<(), String> {
    api::call(client, config, &format!("setPlayerCmd:{}", cmd)).await?;
    println!(
        "{}",
        match cmd {
            "play" => "Playing",
            "pause" => "Paused",
            "stop" => "Stopped",
            "next" => "Next track",
            "prev" => "Previous track",
            _ => cmd,
        }
    );
    Ok(())
}

pub async fn source(client: &Client, config: &SpeakerConfig, name: Option<&str>) -> Result<(), String> {
    let Some(name) = name else {
        println!("Available sources:");
        println!("  wifi");
        println!("  bluetooth (bt)");
        println!("  spotify");
        println!("  line-in (linein)");
        println!("  optical");
        println!("  airplay");
        println!("  dlna");
        println!("  usb");
        return Ok(());
    };

    let sources = source_to_mode();
    let mode = sources
        .get(name.to_lowercase().as_str())
        .ok_or_else(|| {
            format!(
                "Unknown source '{}'. Use 'audiopro source' to list available sources.",
                name
            )
        })?;

    api::call(client, config, &format!("setPlayerCmd:switchmode:{}", mode)).await?;
    println!("Source: {}", mode.to_uppercase());
    Ok(())
}

pub async fn eq(client: &Client, config: &SpeakerConfig, preset: Option<&str>) -> Result<(), String> {
    let presets = eq_presets();

    let Some(preset) = preset else {
        println!("EQ Presets:");
        for (code, name) in &presets {
            println!("  {:>2}  {}", code, name);
        }
        return Ok(());
    };

    let (num, name) = if let Ok(n) = preset.parse::<u8>() {
        if n > 24 {
            return Err("EQ preset must be 0-24".to_string());
        }
        let name = presets
            .iter()
            .find(|(k, _)| k.parse::<u8>().unwrap_or(255) == n)
            .map(|(_, v)| *v)
            .unwrap_or("Unknown");
        (n, name)
    } else {
        let lower = preset.to_lowercase();
        presets
            .iter()
            .find(|(_, v)| v.to_lowercase() == lower)
            .map(|(k, v)| (k.parse::<u8>().unwrap_or(0), *v))
            .ok_or_else(|| format!("Unknown EQ preset '{}'. Use number 0-24 or preset name.", preset))?
    };

    api::call(client, config, &format!("setPlayerCmd:equalizer:{}", num)).await?;
    println!("EQ: {}", name);
    Ok(())
}

pub async fn preset(client: &Client, config: &SpeakerConfig, number: u8) -> Result<(), String> {
    if !(1..=10).contains(&number) {
        return Err("Preset must be 1-10".to_string());
    }
    api::call(client, config, &format!("MCUKeyShortClick:{}", number)).await?;
    println!("Preset {} triggered", number);
    Ok(())
}

pub async fn reboot(client: &Client, config: &SpeakerConfig) -> Result<(), String> {
    api::call(client, config, "reboot").await?;
    println!("Rebooting speaker...");
    Ok(())
}
