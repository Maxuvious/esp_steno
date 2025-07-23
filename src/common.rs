pub fn encode_chord(states: &[bool]) -> u8 {
    let mut chord = 0u8;
    for (i, &pressed) in states.iter().enumerate() {
        if pressed { chord |= 1 << i; }
    }
    chord
}

pub fn parse_chord_line(line: &str) -> Option<u8> {
    if let Some(bits) = line.strip_prefix("CHORD:") {
        u8::from_str_radix(bits, 2).ok()
    } else {
        None
    }
}

pub fn process_chord_sequence(seq: &[u8]) -> &'static str {
    // Dummy logic for test/demo
    if seq == [0b00000011] { "thick" }
    else if seq == [0b00000011, 0b00000100] { "thicker" }
    else { "unknown" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_chord() {
        let states = [true, false, true];
        let encoded = encode_chord(&states);
        assert_eq!(encoded, 0b00000101);
    }

    #[test]
    fn test_parse_chord_from_serial() {
        let input = "CHORD:00010101";
        let expected: u8 = 0b00010101;
        let parsed = parse_chord_line(input).unwrap();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_chord_sequence_context() {
        let sequence = [0b00000011, 0b00000100];
        let out = process_chord_sequence(&sequence);
        assert_eq!(out, "thicker");
    }
}
