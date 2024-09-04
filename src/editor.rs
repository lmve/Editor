use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {

}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop{
            if let Err(error) = self.process_keypress() {
                something_wrong();
            }

        }
    }

    pub fn defuault() -> Self {
        Self{}
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => return Ok(()),
            _ => (),
        }
        Ok(())
    }

}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next(){
            return key;
        }
    }
}

fn something_wrong() -> ! {
    panic!("Something went wrong!");
}
