use midir::{MidiOutput, MidiOutputConnection};
use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use chrono::{Local, DateTime, TimeZone};

// internal dependencies below here

use rust_mcu::base::*;
use rust_mcu::mcu_display_animator::*;
use rust_mcu::messaging::*;

fn main() {

    // midi BP for testin

    // Create a MidiOutput instance
    let midi_out = MidiOutput::new("My Midi Output").unwrap();

    // Get a list of output ports
    let midi_out_ports = midi_out.ports();

    // Display available MIDI devices
    println!("Available MIDI Output Ports:");
    for (i, port) in midi_out_ports.iter().enumerate() {
        println!("{}: {}", i, midi_out.port_name(port).unwrap());
    }

    // Ask the user to select a MIDI device
    println!("Select a MIDI Output Port by entering its index:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let selected_port_index: usize = input.trim().parse().unwrap();




    if let Some(midi_out_port) = midi_out_ports.get(selected_port_index) {    // Check if the selected index is valid

        let mut conn_out: MidiOutputConnection = midi_out.connect(midi_out_port, "my-midi-output").unwrap(); 
    }
}


