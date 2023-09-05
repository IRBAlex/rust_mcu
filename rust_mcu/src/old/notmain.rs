use midir::{MidiOutput, MidiOutputConnection};
use std::thread::sleep;
use std::time::Duration;
use chrono::{Local, DateTime, TimeZone};

const MSG_END: u8 = 0xF7;
const MAX_MSG_LEN: usize = 100;

fn initialize_message() -> Vec<u8> {
    // we need to initialize with
    // 0xf0 0x00 0x00 0x66 0x14 0x12 0x38
    vec!(0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, 0x38)
}

fn string_to_mcu_message(s: &str) -> Vec<u8> {
    // create a new message with required information bytes
    let mut v = initialize_message();

    for b in s.bytes() {
        v.push(b);
    }
    // append message end
    v.push(MSG_END);
    v
}

fn input_to_mcu_message() -> Vec<u8> {
    let mut user_string: String = String::new();
    println!("sm");
    std::io::stdin().read_line(&mut user_string);

    let m = string_to_mcu_message(&user_string);
    m
}

fn validate_message(message: &Vec<u8>) -> bool {
    if message.is_empty() || message.len() > MAX_MSG_LEN {
        return false;
    }
    match message[0] {
        0xF0 => true,
        _ => false
    }
}

fn send_message(msg: &Vec<u8>) {
    if !validate_message(msg) {
        println!("Invalid message");
    }
    let own: Vec<u8> = msg.to_owned();
    println!("{:02X?}", own);
}

fn make_msg_line_two(msg: &mut Vec<u8>) {
    match msg[6] {
        0x00 => msg[6] = 0x38,
        0x38 => (),
        _ => println!("Not a valid line")
    }
}

// I'm assuming sending a blank formatted message will clear the display
fn clear_display() {
    let mut v = initialize_message();
    v.push(MSG_END);
    send_message(&v);
}

fn make_blank_message() -> Vec<u8> {
    let mut v = initialize_message();
    for i in 0..56 {
        v.push(0x20);
    }
    v.push(MSG_END);
    v
}

fn sendmessage(message: &Vec<u8>, conn_out: &mut MidiOutputConnection) {
    if !validate_message(message) {
        println!("invalid message");
    }
    let message_to_send: Vec<u8> = message.to_owned();
    println!("{message_to_send:02X?}");
    conn_out.send(&message_to_send).unwrap();
} 

fn main() {
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

   // Check if the selected index is valid
    if let Some(midi_out_port) = midi_out_ports.get(selected_port_index) { 

        // Connect to the selected output port
        let mut conn_out = midi_out.connect(midi_out_port, "my-midi-output").unwrap();


        let msg = input_to_mcu_message();
        sendmessage(&msg);

        conn_out.close();
    } else {
        println!("Invalid port index selected.");
    }

}
