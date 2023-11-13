use minifb::Key;
use minifb::Window;
use minifb::KeyRepeat;

pub struct Keyboard {
    key_pressed: Option<u8>,
}

impl<'a> Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            key_pressed: None,
        }
    }

    // Todo implement proper key handling
    pub fn is_key_pressed(&self, key_code:u8) -> bool {
        return true;
        match self.key_pressed {
            Some(key) => key == key_code,
            _=> false
        }
    }

    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.key_pressed = key;
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    }
}