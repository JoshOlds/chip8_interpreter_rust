use crate::display::{Display, DisplayBuffer, DisplayMode};
use crossterm::{
    cursor,
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::io::Write;

/// Chip-8 Display interface object that uses CrossTerm as its concrete implementation.
pub struct CrossTermDisplay {
    /// Handle to standard output for writing to terminal
    stdout: std::io::Stdout,
    /// Content style object for formatting terminal output
    term_char: style::StyledContent<char>,
}

impl Display for CrossTermDisplay {
    fn draw(&mut self, display_buffer: &DisplayBuffer) {
        // Align cursor to start
        self.stdout.queue(cursor::MoveTo(0, 0)).unwrap();
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        // Iterate over the display buffer and draw
        for (col, vbuf) in display_buffer.buff.iter().enumerate() {
            for (row, pixel) in vbuf.iter().enumerate() {
                // Write pixel data
                if *pixel {
                    self.stdout
                        .queue(style::PrintStyledContent(self.term_char))
                        .unwrap();
                } else {
                    self.stdout.queue(cursor::MoveRight(1)).unwrap();
                }
            }
            self.stdout.queue(cursor::MoveToColumn(0)).unwrap();
            self.stdout.queue(cursor::MoveDown(1)).unwrap();
        }

        self.stdout.flush().unwrap();
    }

    fn clear_screen(&mut self) {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        self.stdout.flush().unwrap();
    }

    fn hide(&mut self) {
        unimplemented!()
    }
}

impl CrossTermDisplay {
    /// Constructs a new CrossTermDisplay. RAII, formats the terminal display upon construction.
    pub fn new(mode: &DisplayMode) -> CrossTermDisplay {
        let mut new = CrossTermDisplay {
            stdout: std::io::stdout(),
            term_char: style::style('*').with(style::Color::Green),
        };
        // TODO: Actually handle failure to setup terminal
        new.setup_terminal(&mode).unwrap();
        return new;
    }

    /// Sets the character and color for the terminal
    pub fn set_term_character(&mut self, character: &char, color: &style::Color) {
        self.term_char = style::style(*character).with(*color);
    }

    /// Configures the display. Resizes terminal, disables blinking, sets cursor, etc.
    fn setup_terminal(&mut self, display_mode: &DisplayMode) -> Result<()> {
        terminal::enable_raw_mode().unwrap();
        self.stdout
            .queue(terminal::SetSize(
                display_mode.get_h_res() as u16,
                display_mode.get_v_res() as u16,
            ))?
            .queue(cursor::DisableBlinking)?
            .queue(cursor::Hide)?
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;

        self.stdout.flush();
        Ok(())
    }
}
