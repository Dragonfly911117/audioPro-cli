use audiopro::utils::{decode_hex, format_time};

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
