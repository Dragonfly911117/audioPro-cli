use std::collections::HashMap;

pub fn mode_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("0", "idle"),
        ("1", "airplay"),
        ("2", "dlna"),
        ("10", "wifi"),
        ("11", "usb"),
        ("20", "http"),
        ("31", "spotify"),
        ("40", "line-in"),
        ("41", "bluetooth"),
        ("43", "optical"),
        ("99", "slave"),
    ])
}

pub fn eq_presets() -> Vec<(&'static str, &'static str)> {
    vec![
        ("0", "Off"),
        ("1", "Flat"),
        ("2", "Acoustic"),
        ("3", "Bass Booster"),
        ("4", "Bass Reducer"),
        ("5", "Classical"),
        ("6", "Dance"),
        ("7", "Deep"),
        ("8", "Electronic"),
        ("9", "Game"),
        ("10", "Hip-Hop"),
        ("11", "Jazz"),
        ("12", "Latin"),
        ("13", "Loudness"),
        ("14", "Lounge"),
        ("15", "Movie"),
        ("16", "Piano"),
        ("17", "Pop"),
        ("18", "R&B"),
        ("19", "Rock"),
        ("20", "Small Speakers"),
        ("21", "Spoken Word"),
        ("22", "Treble Booster"),
        ("23", "Treble Reducer"),
        ("24", "Vocal Booster"),
    ]
}

pub fn loop_mode_map() -> Vec<(&'static str, &'static str)> {
    vec![
        ("0", "Sequential"),
        ("1", "Repeat All"),
        ("2", "Shuffle"),
        ("3", "Shuffle + Repeat"),
        ("-1", "Repeat One"),
    ]
}

pub fn source_to_mode() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("wifi", "wifi"),
        ("bluetooth", "bluetooth"),
        ("bt", "bluetooth"),
        ("spotify", "spotify"),
        ("line-in", "line-in"),
        ("linein", "line-in"),
        ("optical", "optical"),
        ("airplay", "airplay"),
        ("dlna", "dlna"),
        ("usb", "udisk"),
    ])
}
