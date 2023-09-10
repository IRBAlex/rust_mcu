pub mod consts {
    pub const MSG_END: u8 = 0xF7;
    pub const MAX_MSG_LEN: usize = 54;
    pub const WSPACE: u8 = 0x20;
}

pub enum McuMsgType {
    MainDisplayT,
    MainDisplayB
}

impl McuMsgType {

    fn to_msg_code(self) -> Vec<u8> {
        match self {
            Self::MainDisplayT => vec!(0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, 0x00),
            Self::MainDisplayB => vec!(0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, 0x36)
        }
    }
    pub fn debug_print_msg_type_as_bytes(self) {
        let debug_print: Vec<u8> = self.to_msg_code();
        println!("{:02X?}", debug_print);
        drop(debug_print);
    }
}

pub mod base {
    use super::consts::*;
    use super::McuMsgType;

    pub fn initialize_msg(msg_type: McuMsgType) -> Vec<u8> {
        msg_type.to_msg_code()
    }

    pub fn string_to_mcu_msg(s: &str, msg_type: McuMsgType) -> Vec<u8> {
        // create a new msg with required information bytes
        let mut v = initialize_msg(msg_type);
    
        for b in s.bytes() {
            v.push(b);
        }
        v
    }

    pub fn input_to_mcu_msg(msg_type: McuMsgType) -> Vec<u8> {
        let mut user_string: String = String::new();
        println!("enter text to send:\n");
        std::io::stdin().read_line(&mut user_string);
    
        let m = string_to_mcu_msg(&user_string, msg_type);
        m
    }
}


pub mod messaging {

    use super::McuMsgType;
    use super::base::initialize_msg;
    use super::consts::*;
    use midir::{MidiOutput, MidiOutputConnection};


    // ALL functions that rely on send_msg() will also need a MidiOutputConnection
    // these functions should begin with "send" 
    // to ensure the need for MidiOutputConnection is clear
    pub fn send_msg(msg: &Vec<u8>, conn_out: &mut MidiOutputConnection) {

        let mut msg_to_send: Vec<u8> = msg.to_owned();
        msg_to_send.push(MSG_END);
        conn_out.send(&msg_to_send).unwrap();
    } 

    pub fn clear_main_display(conn_out: &mut MidiOutputConnection) {
        let mut one = initialize_msg(McuMsgType::MainDisplayT);
        let mut two = initialize_msg(McuMsgType::MainDisplayB);

        for i in 0..MAX_MSG_LEN-1 {
            one.push(WSPACE);
            two.push(WSPACE);
        }
        one.push(MSG_END);
        two.push(MSG_END);

        send_msg(&one, conn_out);
        send_msg(&two, conn_out);

    }
}


pub mod mcu_display_animator {
use std::time::Duration;
use std::thread::sleep;

    use midir::MidiOutputConnection;

    use super::McuMsgType;
    use super::base::{
        initialize_msg, 
        string_to_mcu_msg, 
    };
    use super::consts;
    use super::messaging::{
        send_msg,
        clear_main_display,
    };

    struct Animator {
        char_buffer: Vec<u8>,
        frame_buffer: Vec<Vec<u8>>
    }

    impl Animator {
        // allow for creation of blank Animators with Animator::new()
        pub fn new() -> Animator {
            Animator {
                char_buffer: Vec::new(), // Vec<u8>
                frame_buffer: Vec::new() // Vec<Vec<u8>>
                
            }
        }

        pub fn add_to_char_buffer(&mut self, char_buffer_addon: Vec<u8>) {
            if char_buffer_addon.is_empty() {()}

            for b in char_buffer_addon {
                self.char_buffer.push(b);
            }
        }


        pub fn init_frame_buffer(&mut self, msg_type: McuMsgType) {

            if self.char_buffer.is_empty() {()}

            for c in self.char_buffer {
                let mut new_frame: Vec<u8> = initialize_msg(msg_type);
                new_frame.push(c);
                self.frame_buffer.push(new_frame);
        }

        pub fn send_anim_cycle(
            self, 
            location: McuMsgType,
            conn_out: MidiOutputConnection) {
            
            let timeout_sec: Duration = Duration::from_secs(u64::MAX);
            }



        
    }

}
