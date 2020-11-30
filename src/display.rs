use crossterm::{
    cursor,
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};

use std::io::Write;

#[derive(Copy, Clone)]
pub enum DisplayMode {
    H64V32MONOCHROME,
    H128V64MONOCHROME,
}

pub struct Display {
    mode: DisplayMode,
    h_res: usize,
    v_res: usize,
    h_bytes: usize,
    stdout: std::io::Stdout,
    term_char: style::StyledContent<char>,
    pub buffer: [[bool; 64]; 32],
}

impl Display {
    pub fn new(mode: DisplayMode) -> Display {
        match mode {
            DisplayMode::H64V32MONOCHROME => Display {
                mode: mode,
                h_res: 64,
                v_res: 32,
                h_bytes: 64 / 8,
                stdout: std::io::stdout(),
                term_char: style::style('*').with(style::Color::Green),
                buffer: [[false; 64]; 32],
            },
            DisplayMode::H128V64MONOCHROME => Display {
                mode: mode,
                h_res: 128,
                v_res: 64,
                h_bytes: 128 / 8,
                stdout: std::io::stdout(),
                term_char: style::style('*').with(style::Color::Green),
                buffer: [[false; 64]; 32],
            },
        }
    }

    pub fn get_mode(&self) -> DisplayMode {
        self.mode
    }

    pub fn get_screen_size(&self) -> (usize, usize) {
        (self.h_res, self.v_res)
    }

    pub fn setup_terminal(&mut self) -> Result<()> {
        self.stdout
            .queue(terminal::SetSize(self.h_res as u16, self.v_res as u16))?
            .queue(cursor::DisableBlinking)?
            .queue(cursor::Hide)?
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;

        self.stdout.flush()?;
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        // Align cursor to start
        self.stdout.queue(cursor::MoveTo(0, 0))?;
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?;

        for (col, vbuf) in self.buffer.iter().enumerate() {
            for (row, pixel) in vbuf.iter().enumerate() {
                // Write pixel data
                if *pixel {
                    self.stdout
                        .queue(style::PrintStyledContent(self.term_char))?;
                } else {
                    self.stdout.queue(cursor::MoveRight(1))?;
                }
            }
            self.stdout.queue(cursor::MoveToColumn(0))?;
            self.stdout.queue(cursor::MoveDown(1))?;
        }

        self.stdout.flush()?;
        Ok(())
    }

    pub fn clear(&mut self) {
        for vbuf in self.buffer.iter_mut() {
            for pixel in vbuf.iter_mut() {
                *pixel = false;
            }
        }
    }

    fn xor(operand: &mut bool, value: bool) -> bool {
        if !*operand && value {
            *operand = true;
            return false;
        }
        if *operand && value {
            *operand = false;
            return true;
        }
        false
    }

    pub fn write_sprite(&mut self, x: u8, y: u8, slice: &[u8]) -> bool {
        let x = x as usize;
        let y = y as usize;
        let mut collision = false;
        for (i, byte) in slice.iter().enumerate() {
            if byte & 0x80 != 0 {
                if Display::xor(&mut self.buffer[(y + i) % self.v_res][x % self.h_res], true) {
                    collision = true;
                };
            }
            if byte & 0x40 != 0 {
                if Display::xor(
                    &mut self.buffer[(y + i) % self.v_res][(x + 1) % self.h_res],
                    true,
                ) {
                    collision = true;
                };
            }
            if byte & 0x20 != 0 {
                if Display::xor(
                    &mut self.buffer[(y + i) % self.v_res][(x + 2) % self.h_res],
                    true,
                ) {
                    collision = true;
                };
            }
            if byte & 0x10 != 0 {
                if Display::xor(
                    &mut self.buffer[(y + i) % self.v_res][(x + 3) % self.h_res],
                    true,
                ) {
                    collision = true;
                };
            }
            if byte & 0x08 != 0 {
                if Display::xor(
                    &mut self.buffer[(y + i) % self.v_res][(x + 4) % self.h_res],
                    true,
                ) {
                    collision = true;
                };
            }
            if byte & 0x04 != 0 {
                if Display::xor(
                    &mut self.buffer[(y + i) % self.v_res][(x + 5) % self.h_res],
                    true,
                ) {
                    collision = true;
                };
            }
            if byte & 0x02 != 0 {
                if Display::xor(
                    &mut self.buffer[(y + i) % self.v_res][(x + 6) % self.h_res],
                    true,
                ) {
                    collision = true;
                };
            }
            if byte & 0x01 != 0 {
                if Display::xor(
                    &mut self.buffer[(y + i) % self.v_res][(x + 7) % self.h_res],
                    true,
                ) {
                    collision = true;
                };
            }
        }
        collision
    }
}
