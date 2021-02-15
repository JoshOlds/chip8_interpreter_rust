use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::cpu::CPU;

pub fn load_rom_file(cpu: &mut CPU, file_path: &str) -> io::Result<()> {
    let mut in_file = File::open(file_path)?;
    in_file.read(&mut cpu.mem.mem[0x200..])?;
    Ok(())
}
