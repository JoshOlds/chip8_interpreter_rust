use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::time::Duration;

use crate::input::{ChipKeys, Input, Keyboard};

pub struct CrosstermInput {
    pub keyboard: Keyboard,
    pub poll_timeout_millis: u64,
}

impl CrosstermInput {
    pub fn new(poll_timeout_millis: u64) -> CrosstermInput {
        CrosstermInput {
            keyboard: Keyboard::new(),
            poll_timeout_millis,
        }
    }
}

impl Input for CrosstermInput {
    fn get_active_inputs(&self) -> Vec<ChipKeys> {
        self.keyboard.get_active_inputs()
    }

    /// Uses polling read function with timeout to query active keys
    fn update(&mut self) {
        self.keyboard.clear();

        if poll(Duration::from_millis(self.poll_timeout_millis)).unwrap() {
            return match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => {
                        self.keyboard.esc = true;
                    }
                    KeyCode::Char('1') => {
                        self.keyboard.key_1 = true;
                    }
                    KeyCode::Char('2') => {
                        self.keyboard.key_2 = true;
                    }
                    KeyCode::Char('3') => {
                        self.keyboard.key_3 = true;
                    }
                    KeyCode::Char('4') => {
                        self.keyboard.key_c = true;
                    }
                    KeyCode::Char('q') => {
                        self.keyboard.key_4 = true;
                    }
                    KeyCode::Char('w') => {
                        self.keyboard.key_5 = true;
                    }
                    KeyCode::Char('e') => {
                        self.keyboard.key_6 = true;
                    }
                    KeyCode::Char('r') => {
                        self.keyboard.key_d = true;
                    }
                    KeyCode::Char('a') => {
                        self.keyboard.key_7 = true;
                    }
                    KeyCode::Char('s') => {
                        self.keyboard.key_8 = true;
                    }
                    KeyCode::Char('d') => {
                        self.keyboard.key_9 = true;
                    }
                    KeyCode::Char('f') => {
                        self.keyboard.key_e = true;
                    }
                    KeyCode::Char('z') => {
                        self.keyboard.key_a = true;
                    }
                    KeyCode::Char('x') => {
                        self.keyboard.key_0 = true;
                    }
                    KeyCode::Char('c') => {
                        self.keyboard.key_b = true;
                    }
                    KeyCode::Char('v') => {
                        self.keyboard.key_f = true;
                    }
                    _ => {}
                },
                _ => {}
            };
        }
    }
}
