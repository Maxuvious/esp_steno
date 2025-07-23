#[cfg(feature = "device")]
use esp_idf_hal::gpio::*;
#[cfg(feature = "device")]
use std::{thread, time::Duration};

// Always import from flat src/common.rs:
use crate::common::encode_chord;

#[cfg(feature = "device")]
pub fn run() {
    let button_pins = [
        Gpio2::new().unwrap(), // Key 0
        Gpio4::new().unwrap(), // Key 1
        Gpio5::new().unwrap(), // Key 2
        // ...add more as needed
    ];

    let mut buttons: Vec<PinDriver<Input, PullUp>> = button_pins
        .into_iter()
        .map(|pin| PinDriver::input(pin).unwrap())
        .collect();

    loop {
        let button_states: Vec<bool> = buttons.iter().map(|b| b.is_low()).collect();
        let chord = encode_chord(&button_states);

        if chord != 0 {
            println!("CHORD:{:08b}", chord);
            thread::sleep(Duration::from_millis(150));
        }
        thread::sleep(Duration::from_millis(10));
    }
}
