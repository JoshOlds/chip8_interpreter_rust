// Modules
mod cpu;
mod display;
mod input;
mod instructions;
mod memory;

// Core libraries
use crate::cpu::CPU;
use crate::display::{Display, DisplayBuffer, DisplayMode};
use crate::input::{ChipKeys, Input};
use crate::memory::Memory;

// Concrete Displays
use crate::display::crossterm_display::CrossTermDisplay;

// Concrete Inputs
use crate::input::crossterm_input::CrosstermInput;
use crate::input::ChipKeys::Key5;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    keyboard_test()
}

fn keyboard_test() {
    let mut display = CrossTermDisplay::new(DisplayMode::H64V32MONOCHROME);
    let mut system_input = CrosstermInput::new(0);

    let mut cpu = CPU::new(Box::new(display), Box::new(system_input));
    cpu.mem.load_ascii_fonts();

    let vres = cpu.display_reference.get_display_mode().get_v_res();
    let hres = cpu.display_reference.get_display_mode().get_h_res();
    let mut x = hres / 2;
    let mut y = vres / 2;

    loop {
        cpu.input_reference.update();
        let keys = cpu.input_reference.get_active_inputs();

        if keys.contains(&ChipKeys::Key5) {
            y = ((y - 1) % vres);
        }
        if keys.contains(&ChipKeys::Key7) {
            x = ((x - 1) % hres);
        }
        if keys.contains(&ChipKeys::Key8) {
            y = ((y + 1) % vres);
        }
        if keys.contains(&ChipKeys::Key9) {
            x = ((x + 1) % hres);
        }
        if keys.contains(&ChipKeys::ESC) {
            return;
        }

        {
            let display_buffer = cpu.display_reference.get_display_buffer();
            display_buffer.clear();
            display_buffer.write_sprite(x, y, cpu.mem.get_ascii_slice(0xF).unwrap());
        }
        cpu.display_reference.clear_screen();
        cpu.display_reference.draw();
        sleep(Duration::from_millis(16));
    }
}

// fn scroll_test() -> Result<()> {
//     let mut cpu = CPU::new();
//     cpu.mem.load_ascii_fonts();
//
//     let mut display = Display::new(DisplayMode::H64V32MONOCHROME);
//     display.setup_terminal()?;
//
//     for x in 0..16 {
//         display.write_sprite(0, 0, cpu.mem.get_ascii_slice(x).unwrap());
//         display.draw()?;
//         std::thread::sleep(std::time::Duration::from_millis(100));
//         display.clear();
//     }
//
//     let mut x = 0;
//     let mut y = 0;
//
//     loop {
//         display.write_sprite(x, y, cpu.mem.get_ascii_slice(0x0).unwrap());
//         display.write_sprite(x + 6, y + 5, cpu.mem.get_ascii_slice(0x1).unwrap());
//         display.write_sprite(x + 12, y + 10, cpu.mem.get_ascii_slice(0x2).unwrap());
//         display.write_sprite(x + 18, y + 15, cpu.mem.get_ascii_slice(0x3).unwrap());
//         display.write_sprite(x + 24, y + 20, cpu.mem.get_ascii_slice(0x4).unwrap());
//         display.write_sprite(x + 32, y + 25, cpu.mem.get_ascii_slice(0x5).unwrap());
//         display.draw()?;
//         std::thread::sleep(std::time::Duration::from_millis(100));
//         display.clear();
//         x = (x + 1) % 64;
//         y = (y + 1) % 32;
//     }
//
//     Ok(())
// }
