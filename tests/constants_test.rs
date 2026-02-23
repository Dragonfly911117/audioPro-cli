use audiopro::constants::{eq_presets, mode_map, source_to_mode};

#[test]
fn mode_map_spotify() {
    assert_eq!(mode_map().get("31"), Some(&"spotify"));
}

#[test]
fn mode_map_bluetooth() {
    assert_eq!(mode_map().get("41"), Some(&"bluetooth"));
}

#[test]
fn mode_map_count() {
    assert_eq!(mode_map().len(), 11);
}

#[test]
fn eq_presets_count() {
    assert_eq!(eq_presets().len(), 25);
}

#[test]
fn eq_presets_first_last() {
    let presets = eq_presets();
    assert_eq!(presets[0], ("0", "Off"));
    assert_eq!(presets[24], ("24", "Vocal Booster"));
}

#[test]
fn source_to_mode_aliases() {
    let sources = source_to_mode();
    assert_eq!(sources.get("bt"), Some(&"bluetooth"));
    assert_eq!(sources.get("linein"), Some(&"line-in"));
}
