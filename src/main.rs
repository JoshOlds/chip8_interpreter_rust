// Modules
mod cpu;
mod display;
mod keyboard;
mod memory;

// Core libraries
use crate::cpu::CPU;
use crate::display::{Display, DisplayBuffer, DisplayMode};
use crate::keyboard::Keyboard;
use crate::memory::Memory;

// Displays
use crate::display::crossterm_display::CrossTermDisplay;

fn main() {
    keyboard_test()
}

fn keyboard_test() {
    let mut cpu = CPU::new();
    cpu.mem.load_ascii_fonts();

    let mut display = CrossTermDisplay::new(DisplayMode::H64V32MONOCHROME);

    let mut keyboard = Keyboard::new();

    let vres = display.get_display_mode().get_v_res();
    let hres = display.get_display_mode().get_h_res();
    let mut x = hres / 2;
    let mut y = vres / 2;

    loop {
        keyboard.update(1000);

        if keyboard.key_5 {
            y = ((y - 1) % vres);
        }
        if keyboard.key_7 {
            x = ((x - 1) % hres);
        }
        if keyboard.key_8 {
            y = ((y + 1) % vres);
        }
        if keyboard.key_9 {
            x = ((x + 1) % hres);
        }
        if keyboard.key_esc {
            return;
        }

        {
            let display_buffer = display.get_display_buffer();
            display_buffer.clear();
            display_buffer.write_sprite(x, y, cpu.mem.get_ascii_slice(0xF).unwrap());
        }
        display.clear_screen();
        display.draw();
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
