
use std::io::{self, Read, stdout};
use termion::raw::IntoRawMode;


// quit the editor
fn quit_editor(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b001_1111
}

// handle errors
fn handle_errors(e: std::io::Error) {
    panic!("{}", e);
}

fn main(){
    // into_raw_mode() termion privides stdout.
    let _stdout = stdout().into_raw_mode().unwrap();

    // control chars are readable
    for b in io::stdin().bytes(){
        match b {
            Ok(b) => {
                let val = b;
                let c = val as char;

                if c.is_control() {
                    println!("{:?} \r", val);
                } else {
                    println!("{:?} ({}) \r", val, c);
                }

                if val == quit_editor('q') {
                    break;
                }
            },

            Err(e) => handle_errors(e),
        }
    }
}