

pub mod consts {
    pub const MSG_END: u8 = 0xF7;
    pub const MAX_MSG_LEN: usize = 100;
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
        // append message end
        v.push(MSG_END);
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

pub mod midi_message_sending {
    use super::base::validate_message;
    use midir::{MidiOutput, MidiOutputConnection};

    fn send_message(msg: &Vec<u8>, conn_out: &mut MidiOutputConnection) {
        if !validate_message(msg) {
            println!("invalid message");
        }
        let message_to_send: Vec<u8> = msg.to_owned();
        conn_out.send(&message_to_send).unwrap();
    } 
}


pub mod mcu_display_animator {
    use super::base::*;
    use super::consts;
    struct Animator {
        buffer: Vec<u8>,
    }

    impl Animator {
        // allow for creation of blank Animators with Animator::new()
        pub fn new() -> Animator {
            Animator {
                buffer: Vec::<u8>::new(),
            }
        }
        // runs initialize_message(line) on Animator.buffer 
        pub fn init_as_mcu_text_msg(line: u8) -> Animator {
            Animator {
                buffer: initialize_message(line),
            }
        }
        // add a single character to an existing Animator's buffer list
        pub fn add_char(&mut self, c: u8) {
            self.buffer.push(c);
        }

        // add buffer (as u8) from a vec to an existing Animator's buffer list
        pub fn add_buffer(&mut self, buffer: Vec<u8>) {
            if buffer.is_empty() {()}

            for c in buffer {
                self.buffer.push(c);
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

        // cycle through the characters in the Animator's buffer
        pub fn cycle_char(){()}
    }
}
