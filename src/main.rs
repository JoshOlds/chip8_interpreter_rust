mod cpu;
mod keyboard;
mod memory;
mod display;

use cpu::CPU;
use keyboard::Keyboard;
use display::{Display, DisplayMode};

use crossterm::{
    cursor,
    event::{read, Event},
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};

use std::io::{stdout, Write};

fn main() -> Result<()> {
    keyboard_test()
}

fn keyboard_test() -> Result<()> {
    terminal::enable_raw_mode().unwrap();

    let mut cpu = CPU::new();
    cpu.mem.load_ascii_fonts();

    let mut display = Display::new(DisplayMode::H64V32MONOCHROME);
    display.setup_terminal()?;

    let mut keyboard = Keyboard::new();

    let (xres, yres) = display.get_screen_size();
    let mut x: u8 = xres as u8 / 2;
    let mut y: u8 = yres as u8 / 2;

    loop {
        keyboard.update(1000)?;

        if keyboard.key_5 {
            y = ((y as i32 - 1) % 32) as u8;
        }
        if keyboard.key_7 {
            x = ((x as i32 - 1) % 64) as u8;
        }
        if keyboard.key_8 {
            y = ((y as i32 + 1) % 32) as u8;
        }
        if keyboard.key_9 {
            x = ((x as i32 + 1) % 64) as u8;
        }
        if keyboard.key_esc {
            return Ok(());
        }

        display.clear();
        display.write_sprite(x, y, cpu.mem.get_ascii_slice(0xF).unwrap());
        display.draw()?;

        // match read()? {
        //     Event::Key(event) => match event.code {
        //         crossterm::event::KeyCode::Up => y = ((y as i32 - 1) % 32) as u8,
        //         crossterm::event::KeyCode::Down => y = ((y as i32 + 1) % 32) as u8,
        //         crossterm::event::KeyCode::Left => x = ((x as i32 - 1) % 64) as u8,
        //         crossterm::event::KeyCode::Right => x = ((x as i32 + 1) % 64) as u8,
        //         _ => (),
        //     },
        //     _ => (),
        // }
    }
}

fn scroll_test() -> Result<()> {
    let mut cpu = CPU::new();
    cpu.mem.load_ascii_fonts();

    let mut display = Display::new(DisplayMode::H64V32MONOCHROME);
    display.setup_terminal()?;

    for x in 0..16 {
        display.write_sprite(0, 0, cpu.mem.get_ascii_slice(x).unwrap());
        display.draw()?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        display.clear();
    }

    let mut x = 0;
    let mut y = 0;

    loop {
        display.write_sprite(x, y, cpu.mem.get_ascii_slice(0x0).unwrap());
        display.write_sprite(x + 6, y + 5, cpu.mem.get_ascii_slice(0x1).unwrap());
        display.write_sprite(x + 12, y + 10, cpu.mem.get_ascii_slice(0x2).unwrap());
        display.write_sprite(x + 18, y + 15, cpu.mem.get_ascii_slice(0x3).unwrap());
        display.write_sprite(x + 24, y + 20, cpu.mem.get_ascii_slice(0x4).unwrap());
        display.write_sprite(x + 32, y + 25, cpu.mem.get_ascii_slice(0x5).unwrap());
        display.draw()?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        display.clear();
        x = (x + 1) % 64;
        y = (y + 1) % 32;
    }

    Ok(())
}
