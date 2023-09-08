

pub mod consts {
    pub const MSG_END: u8 = 0xF7;
    pub const MAX_MSG_LEN: usize = 54;
    pub const SPACE: u8 = 0x20;
}




pub mod base {
    use super::consts::*;

    pub fn initialize_message(line: u8) -> Vec<u8> {
        // we need to initialize with
        // 0xf0 0x00 0x00 0x66 0x14 0x12 0x00/0x38 depending on line num

        match line {
            1 => vec!(0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, 0x00),
            2 => vec!(0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, 0x38),
            _ => {
                println!("Unable to initialize message on line {}", line);
                vec!()
            }  
        }
    }
    
    pub fn string_to_mcu_message(s: &str, line: u8) -> Vec<u8> {
        // create a new message with required information bytes
        let mut v = initialize_message(line);
    
        for b in s.bytes() {
            v.push(b);
        }
        v
    }
    
    
    pub fn input_to_mcu_message(line: u8) -> Vec<u8> {
        let mut user_string: String = String::new();
        println!("enter text to send:\n");
        std::io::stdin().read_line(&mut user_string);
    
        let m = string_to_mcu_message(&user_string, line);
        m
    }
    
    pub fn validate_message(message: &Vec<u8>) -> bool {
        if message.is_empty() || message.len() > MAX_MSG_LEN {
            return false;
        }
        match message[0] {
            0xF0 => true,
            _ => false
        }
    }
    
    fn send_message_test(msg: &Vec<u8>) {
        if !validate_message(msg) {
            println!("Invalid message");
        }
        let own: Vec<u8> = msg.to_owned();
        println!("{:02X?}", own);
    }
}

pub mod messaging {
    use std::mem::MaybeUninit;

    use crate::base::initialize_message;
    use super::base::validate_message;
    use super::consts::*;
    use midir::{MidiOutput, MidiOutputConnection};

    pub enum McuMessageType {
        MainDisplayT,
        MainDisplayB
    }

    impl McuMessageType {

        pub fn to_msg_code(self) -> Vec<u8> {
            match self {
                Self::MainDisplayT => vec!(0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, 0x00),
                Self::MainDisplayB => vec!(0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, 0x36)
            }
        }

    }

    pub fn send_message(msg: &Vec<u8>, conn_out: &mut MidiOutputConnection) {
        if !validate_message(msg) {
            println!("invalid message");
        }
        let mut message_to_send: Vec<u8> = msg.to_owned();
        message_to_send.push(MSG_END);
        conn_out.send(&message_to_send).unwrap();
    } 

    pub fn clear_display(conn_out: &mut MidiOutputConnection) {
        let mut one = initialize_message(1);
        let mut two = initialize_message(2);

        for i in 0..MAX_MSG_LEN-1 {
            one.push(SPACE);
            two.push(SPACE);
        }
        one.push(MSG_END);
        two.push(MSG_END);

        send_message(&one, conn_out);
        send_message(&two, conn_out);

    }


}


pub mod mcu_display_animator {
    use super::base::{
        initialize_message, 
        string_to_mcu_message, 
        validate_message
    };
    use super::consts;
    use super::messaging;

    struct Animator {
        buffer: Vec<u8>,
        line: u8
        // the line number coresponds to the byte stored at buffer[6]
        // 0x00 for line 1, 0x38 for line 2
    }

    impl Animator {
        // allow for creation of blank Animators with Animator::new()
        pub fn new() -> Animator {
            Animator {
                buffer: Vec::<u8>::new(),
                line: 1
            }
        }
        // runs initialize_message(line) on Animator.buffer 
        pub fn init_as_mcu_text_msg(line: u8) -> Animator {
            Animator {
                buffer: initialize_message(line),
                line: line
            }
        }
        // add a single character to an existing Animator's buffer list
        pub fn add_char(&mut self, c: u8) {
            self.buffer.push(c);
        }

        // add buffer (as u8) from a vec to an existing Animator's buffer list
        pub fn add_buffer(&mut self, buffer: Vec<u8>) {
            if buffer.is_empty() {()}

            for b in buffer {
                self.buffer.push(b);
            }
        }

        
        pub fn change_anim_line(&mut self) {
            if self.buffer.is_empty() || self.buffer.len() < 7 {
                println!("Cannot change line of incomplete message");
            }
            match self.buffer[6] {
                0x00 => self.buffer[6] = 0x38,
                0x38 => self.buffer[6] = 0x00,
                _ => println!("Not a valid line number for MCU text message"),
            }
        }
    }
}
