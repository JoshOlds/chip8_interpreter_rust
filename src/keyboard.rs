use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::time::Duration;

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
    pub key_esc: bool,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            ..Default::default()
        }
    }

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
        self.key_esc = false;
    }

    pub fn update(&mut self, timeout: u64) -> Result<bool> {
        self.clear();
        if poll(Duration::from_millis(timeout))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => {
                        self.key_esc = true;
                        return Ok(true);
                    }
                    KeyCode::Char('1') => {
                        self.key_1 = true;
                        return Ok(true);
                    }
                    KeyCode::Char('2') => {
                        self.key_2 = true;
                        return Ok(true);
                    }
                    KeyCode::Char('3') => {
                        self.key_3 = true;
                        return Ok(true);
                    }
                    KeyCode::Char('4') => {
                        self.key_c = true;
                        return Ok(true);
                    }
                    KeyCode::Char('q') => {
                        self.key_4 = true;
                        return Ok(true);
                    }
                    KeyCode::Char('w') => {
                        self.key_5 = true;
                        return Ok(true);
                    }
                    KeyCode::Char('e') => {
                        self.key_6 = true;
                        return Ok(true);
                    }
                    KeyCode::Char('r') => {
                        self.key_d = true;
                        return Ok(true);
                    }
                    KeyCode::Char('a') => {
                        self.key_7 = true;
                        return Ok(true);
                    }
                    KeyCode::Char('s') => {
                        self.key_8 = true;
                        return Ok(true);
                    }
                    KeyCode::Char('d') => {
                        self.key_9 = true;
                        return Ok(true);
                    }
                    KeyCode::Char('f') => {
                        self.key_e = true;
                        return Ok(true);
                    }
                    KeyCode::Char('z') => {
                        self.key_a = true;
                        return Ok(true);
                    }
                    KeyCode::Char('x') => {
                        self.key_0 = true;
                        return Ok(true);
                    }
                    KeyCode::Char('c') => {
                        self.key_b = true;
                        return Ok(true);
                    }
                    KeyCode::Char('v') => {
                        self.key_f = true;
                        return Ok(true);
                    }
                    _ => return Ok(false),
                },
                _ => return Ok(false),
            }
        }
        Ok(false)
    }
}
