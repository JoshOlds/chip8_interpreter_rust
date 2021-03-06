pub mod crossterm_input;

/// Trait to abstract CHIP-8 System input
pub trait Input {
    /// Updates (reads) all key values and updates the given keyboard object
    fn update(&mut self, keyboard: &mut Keyboard);
}

const NUMBER_OF_KEYS: usize = 17;

/// Enumeration that maps all CHIP-8 Keyboard input, plus interpreter system keys
#[derive(PartialEq)]
pub enum ChipKeys {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    ESC,
}

impl ChipKeys {
    pub fn to_hex(&self) -> u8 {
        return match self {
            ChipKeys::Key0 => 0x0,
            ChipKeys::Key1 => 0x1,
            ChipKeys::Key2 => 0x2,
            ChipKeys::Key3 => 0x3,
            ChipKeys::Key4 => 0x4,
            ChipKeys::Key5 => 0x5,
            ChipKeys::Key6 => 0x6,
            ChipKeys::Key7 => 0x7,
            ChipKeys::Key8 => 0x8,
            ChipKeys::Key9 => 0x9,
            ChipKeys::KeyA => 0xA,
            ChipKeys::KeyB => 0xB,
            ChipKeys::KeyC => 0xC,
            ChipKeys::KeyD => 0xD,
            ChipKeys::KeyE => 0xE,
            ChipKeys::KeyF => 0xF,
            ChipKeys::ESC => 0x0,
        };
    }
}

/// CHIP-8 Keyboard + Extra system keys
#[derive(Default)]
pub struct Keyboard {
    pub key_0: bool,
    pub key_1: bool,
    pub key_2: bool,
    pub key_3: bool,
    pub key_4: bool,
    pub key_5: bool,
    pub key_6: bool,
    pub key_7: bool,
    pub key_8: bool,
    pub key_9: bool,
    pub key_a: bool,
    pub key_b: bool,
    pub key_c: bool,
    pub key_d: bool,
    pub key_e: bool,
    pub key_f: bool,
    pub esc: bool,
}

impl Keyboard {
    /// Constructs a new keyboard, all keys default to not pressed
    pub fn new() -> Keyboard {
        Keyboard {
            ..Default::default()
        }
    }

    /// Clears all keys (resets state to not pressed)
    pub fn clear(&mut self) {
        self.key_0 = false;
        self.key_1 = false;
        self.key_2 = false;
        self.key_3 = false;
        self.key_4 = false;
        self.key_5 = false;
        self.key_6 = false;
        self.key_7 = false;
        self.key_8 = false;
        self.key_9 = false;
        self.key_a = false;
        self.key_b = false;
        self.key_c = false;
        self.key_d = false;
        self.key_e = false;
        self.key_f = false;
        self.esc = false;
    }

    /// Returns a Vector of ChipKeys that contains all active (pressed) keys
    pub fn get_active_inputs(&self) -> Vec<ChipKeys> {
        let mut inputs: Vec<ChipKeys> = Vec::with_capacity(NUMBER_OF_KEYS);
        if self.key_0 {
            inputs.push(ChipKeys::Key0);
        }
        if self.key_1 {
            inputs.push(ChipKeys::Key1);
        }
        if self.key_2 {
            inputs.push(ChipKeys::Key2);
        }
        if self.key_3 {
            inputs.push(ChipKeys::Key3);
        }
        if self.key_4 {
            inputs.push(ChipKeys::Key4);
        }
        if self.key_5 {
            inputs.push(ChipKeys::Key5);
        }
        if self.key_6 {
            inputs.push(ChipKeys::Key6);
        }
        if self.key_7 {
            inputs.push(ChipKeys::Key7);
        }
        if self.key_8 {
            inputs.push(ChipKeys::Key8);
        }
        if self.key_9 {
            inputs.push(ChipKeys::Key9);
        }
        if self.key_a {
            inputs.push(ChipKeys::KeyA);
        }
        if self.key_b {
            inputs.push(ChipKeys::KeyB);
        }
        if self.key_c {
            inputs.push(ChipKeys::KeyC);
        }
        if self.key_d {
            inputs.push(ChipKeys::KeyD);
        }
        if self.key_e {
            inputs.push(ChipKeys::KeyE);
        }
        if self.key_f {
            inputs.push(ChipKeys::KeyF);
        }
        if self.esc {
            inputs.push(ChipKeys::ESC);
        }
        inputs
    }

    pub fn is_pressed(&self, hex_key: u8) -> bool {
        return match hex_key {
            0x0 => self.key_0,
            0x1 => self.key_1,
            0x2 => self.key_2,
            0x3 => self.key_3,
            0x4 => self.key_4,
            0x5 => self.key_5,
            0x6 => self.key_6,
            0x7 => self.key_7,
            0x8 => self.key_8,
            0x9 => self.key_9,
            0xA => self.key_a,
            0xB => self.key_b,
            0xC => self.key_c,
            0xD => self.key_d,
            0xE => self.key_e,
            0xF => self.key_f,
            _ => false,
        };
    }
}
