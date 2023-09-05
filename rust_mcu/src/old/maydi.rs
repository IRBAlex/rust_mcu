fn main() {

}

#[derive(Debug)]
struct Animator {
    chars: Vec<u8>,
    line_num: u8 //only 2 valid line numbers exist, 0x00 and 0x38
}

impl Animator {
    fn new() -> Animator {
        Animator {chars: Vec::<u8>::new(), line_num: 0x00}
    }
    
    fn init_as_mcu_text_msg() -> Animator {
        let v = initialize_message();
        
    }
}