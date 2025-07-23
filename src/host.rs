#[cfg(feature = "host")]
use serialport::SerialPort;
#[cfg(feature = "host")]
use std::{io::BufRead, time::Duration};

use crate::common::{parse_chord_line, process_chord_sequence};

#[cfg(feature = "host")]
pub fn run() {
    let port_name = "COM5"; // or "/dev/ttyUSB0"
    let baud_rate = 115_200;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(2000))
        .open();

    let mut port = match port {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Could not open serial port {}: {}", port_name, e);
            return;
        }
    };

    let reader = std::io::BufReader::new(port);

    println!("Listening for steno chords on {}...", port_name);

    let mut chord_buffer = Vec::new();

    for line in reader.lines() {
        if let Ok(text) = line {
            if let Some(chord) = parse_chord_line(&text) {
                println!("Received chord: {:08b}", chord);
                chord_buffer.push(chord);

                // For demo: process every 2 chords as a "sequence"
                if chord_buffer.len() == 2 {
                    let result = process_chord_sequence(&chord_buffer);
                    println!("Sequence output: {}", result);
                    chord_buffer.clear();
                }
            } else {
                println!("(debug) {}", text);
            }
        }
    }
}
