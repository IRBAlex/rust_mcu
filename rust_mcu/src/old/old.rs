use midir::{MidiOutput, MidiOutputConnection};
use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use chrono::{Local, DateTime, TimeZone};
 
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

 
 
 
    if let Some(midi_out_port) = midi_out_ports.get(selected_port_index) {    // Check if the selected index is valid
 
    let mut conn_out = midi_out.connect(midi_out_port, "my-midi-output").unwrap();    // Connect to the selected output port
        
        // sendtext(1, &mut conn_out,"qwerty");                                                                                                                            //send text to display
 
    cleardisplay(1, &mut conn_out);
    cleardisplay(2, &mut conn_out);
 
    let mut file = File::open("src/test.txt").expect("File not found");                                                                 //open text file
    let mut data = String::new();                                                                                                                                     //turn file into string
    file.read_to_string(&mut data).expect("Error while reading file");                                                                        //error handle
    let num_to_truncate = 54;
 
    for i in 1..30 {
        let mut file = File::open("src/test.txt").expect("File not found");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Error while reading file");
 
        if i > 1 {
            
            let previous_string = &data;
            let previous_string = truncate_characters_from_front(&previous_string, (num_to_truncate * (i -1)));
            let previous_string = truncate_string_by_length(&previous_string, (num_to_truncate));
 
            let data = truncate_characters_from_front(& data, (num_to_truncate * i));
            let data = truncate_string_by_length(&data,num_to_truncate);
                
            println!("{data}");
 
            sendtext(1, &mut conn_out, &previous_string);
            sendtext(2, &mut conn_out, &data);
 
        } else {
            let data = truncate_string_by_length(&data,num_to_truncate);  
            sendtext(1, &mut conn_out, &data);
        }
            sleep(Duration::from_millis(200));
 
    }
    sleep(Duration::from_millis(100));
    cleardisplay(1,&mut conn_out);
    cleardisplay(2, &mut conn_out);
 
 
    let mut t = 0;
    while t <5000 {
        let current_time: DateTime<Local> = Local::now();
        let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("Current time: {}", formatted_time);
        sendtext(1, &mut conn_out, &formatted_time);
        sleep(Duration::from_millis(250));
        t = t +1;
    }
 
 
 
    conn_out.close();       // Don't forget to close the connection when you're done
 
 
 
} else {
        println!("Invalid port index selected.");
    }
}
 
fn truncate_characters_from_front(input: &str, num_to_truncate: usize) -> String {
    if num_to_truncate >= input.len() {
        return String::new();
    }
 
    let truncated: String = input.chars().skip(num_to_truncate).collect();
    truncated
}
 
fn truncate_string_by_length(input: &str,num_to_truncate: usize) -> String {
    let mut truncated = String::new();
    let mut current_length = 0;
    let max_length = 54;
 
    for c in input.chars() {
        truncated.push(c);
        current_length += 1;
 
        if current_length >= max_length {
            break;
        }
    }
 
    truncated
}
 
fn cleardisplay(line:u8, conn_out: &mut MidiOutputConnection){
    let messagetype = 0xF0 as u8;
    let mut hex_values: Vec<u8> = vec![0x20; 64];
    initializemessage(messagetype, &mut hex_values);
    match line{
        1 => hex_values[6] = 0x00,
        2 => hex_values[6] = 0x38,
        _ => println!("invalid line"),
    }
    let message = &hex_values;
    conn_out.send(message).unwrap();
    println!("display clear attempt");
    println!("{:?}", hex_values);
 
}
 
fn sendtext(line: u8, conn_out: &mut MidiOutputConnection,input: &str){
    let messagetype = 0xF0 as u8;                                           // Set message type as text to send
    let mut hex_values: Vec<u8> = vec![0x20; 64];                   // Open memory for the message written and sent
    initializemessage(messagetype, &mut hex_values);            // initialize the message with the necassary status bits to be sent
    match line{
        1 => hex_values[6] = 0x00,
        2 => hex_values[6] = 0x38,
        _ => println!("invalid line"),
    }
 
    //println!("sending : {}", user_input);
 
    //println!("{:?}", user_input.as_bytes());
 
    let textbytes = input.as_bytes().to_vec();
 
    for i in 0..textbytes.len() {
        hex_values[i+8] = textbytes[i];
    }
 
 
    //println!("{:?}", hex_values);
 
 
    let message = &hex_values;
    conn_out.send(message).unwrap();
 
}
 
fn userprompttext(line: u8, conn_out: &mut MidiOutputConnection,){
    let messagetype = 0xF0 as u8;                                       // Set message type as text to send
    let mut hex_values: Vec<u8> = vec![0x20; 64];                   // Open memory for the message written and sent
    initializemessage(messagetype, &mut hex_values);            // initialize the message with the necassary status bits to be sent
    match line{
        1 => hex_values[6] = 0x00,
        2 => hex_values[6] = 0x38,
        _ => println!("invalid line"),
    }
 
    println!("Please enter a string:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Test");
 
    //println!("sending : {}", user_input);
 
    //println!("{:?}", user_input.as_bytes());
 
    let textbytes = user_input.as_bytes().to_vec();
 
    for i in 0..(textbytes.len() -2) {
        hex_values[i+8] = textbytes[i] - 0;
    }
 
 
    println!("{:?}", hex_values);
 
 
    let message = &hex_values;
    conn_out.send(message).unwrap();
}
 
fn initializemessage(messagetype: u8, hex_values: &mut Vec<u8>) {
    if messagetype == 0xF0 {
        todo!(
            for 
        );
        hex_values[0] = 0xF0;
        hex_values[1] = 0x00;
        hex_values[2] = 0x00;
        hex_values[3] = 0x66;
        hex_values[4] = 0x14;
        hex_values[5] = 0x12;
        hex_values[6] = 0x38;
        // hex_values[63] = 0xF7;
        // we'll append MSG_END to our messages later
    else
}
 
fn sendmessage(hex_values: &mut Vec<u8>, conn_out: &mut MidiOutputConnection){
    let message = &hex_values;
    conn_out.send(message).unwrap();
}