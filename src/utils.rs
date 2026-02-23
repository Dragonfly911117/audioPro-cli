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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_hex_empty() {
        assert_eq!(decode_hex(""), "");
    }

    #[test]
    fn decode_hex_ascii() {
        assert_eq!(decode_hex("48656C6C6F"), "Hello");
    }

    #[test]
    fn decode_hex_utf8() {
        // "雪の種" in hex
        assert_eq!(decode_hex("e99baae381aee7a8ae"), "雪の種");
    }

    #[test]
    fn format_time_zero() {
        assert_eq!(format_time("0"), "00:00");
    }

    #[test]
    fn format_time_seconds() {
        assert_eq!(format_time("45000"), "00:45");
    }

    #[test]
    fn format_time_minutes() {
        assert_eq!(format_time("125000"), "02:05");
    }

    #[test]
    fn format_time_invalid() {
        assert_eq!(format_time("invalid"), "00:00");
    }
}
