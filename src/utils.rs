pub fn decode_hex(hex: &str) -> String {
    if hex.is_empty() {
        return String::new();
    }

    let bytes: Vec<u8> = (0..hex.len())
        .step_by(2)
        .filter_map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
        .collect();

    String::from_utf8_lossy(&bytes).to_string()
}

pub fn format_time(ms_str: &str) -> String {
    let ms: u64 = ms_str.parse().unwrap_or(0);
    let secs = ms / 1000;
    let mins = secs / 60;
    let secs = secs % 60;
    format!("{:02}:{:02}", mins, secs)
}
