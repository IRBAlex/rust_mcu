use midir::{MidiOutput, MidiOutputConnection};
use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use chrono::{Local, DateTime, TimeZone};

use rust_mcu::base::*;
use rust_mcu::mcu_display_animator::*;


fn main() {

}

fn send_message(msg: &Vec<u8>, conn_out: &mut MidiOutputConnection) {
    if !validate_message(msg) {
        println!("invalid message");
    }
    let message_to_send: Vec<u8> = msg.to_owned();
    conn_out.send(&message_to_send).unwrap();
} 

