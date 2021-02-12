use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::time::Duration;

use crate::input::{ChipKeys, Input, Keyboard};

pub struct CrosstermInput {
    pub poll_timeout_millis: u64,
}

impl CrosstermInput {
    pub fn new(poll_timeout_millis: u64) -> CrosstermInput {
        CrosstermInput {
            poll_timeout_millis,
        }
    }
}

impl Input for CrosstermInput {
    /// Uses polling read function with timeout to query active keys
    fn update(&mut self, keyboard: &mut Keyboard) {
        keyboard.clear();

        if poll(Duration::from_millis(self.poll_timeout_millis)).unwrap() {
            return match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => {
                        keyboard.esc = true;
                    }
                    KeyCode::Char('1') => {
                        keyboard.key_1 = true;
                    }
                    KeyCode::Char('2') => {
                        keyboard.key_2 = true;
                    }
                    KeyCode::Char('3') => {
                        keyboard.key_3 = true;
                    }
                    KeyCode::Char('4') => {
                        keyboard.key_c = true;
                    }
                    KeyCode::Char('q') => {
                        keyboard.key_4 = true;
                    }
                    KeyCode::Char('w') => {
                        keyboard.key_5 = true;
                    }
                    KeyCode::Char('e') => {
                        keyboard.key_6 = true;
                    }
                    KeyCode::Char('r') => {
                        keyboard.key_d = true;
                    }
                    KeyCode::Char('a') => {
                        keyboard.key_7 = true;
                    }
                    KeyCode::Char('s') => {
                        keyboard.key_8 = true;
                    }
                    KeyCode::Char('d') => {
                        keyboard.key_9 = true;
                    }
                    KeyCode::Char('f') => {
                        keyboard.key_e = true;
                    }
                    KeyCode::Char('z') => {
                        keyboard.key_a = true;
                    }
                    KeyCode::Char('x') => {
                        keyboard.key_0 = true;
                    }
                    KeyCode::Char('c') => {
                        keyboard.key_b = true;
                    }
                    KeyCode::Char('v') => {
                        keyboard.key_f = true;
                    }
                    _ => {}
                },
                _ => {}
            };
        }
    }
}
