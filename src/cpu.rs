use crate::keyboard::Keyboard;

pub struct CPU {
    pub mem: [u8; 4096],
    pub stack: [u16; 16],
    // Registers available to programs
    pub gp_regs: [u8; 16],
    pub i_reg: u16,
    pub delay_reg: u8,
    pub sound_reg: u8,
    // Private psuedo-registers
    pub pc_reg: u16,
    pub stack_reg: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            mem: [0; 4096],
            stack: [0; 16],
            gp_regs: [0; 16],
            i_reg: 0,
            delay_reg: 0,
            sound_reg: 0,
            pc_reg: 0,
            stack_reg: 0,
        }
    }

    pub fn clear_mem(&mut self) {
        for elem in self.mem.iter_mut() {
            *elem = 0;
        }
    }

    pub fn mem_string(&self) -> String {
        // Format as hex
        format!("{:x?}", self.mem)
    }

    pub fn load_ascii_fonts(&mut self) {
        // Ascii 0
        self.mem[0x00] = 0xF0;
        self.mem[0x01] = 0x90;
        self.mem[0x02] = 0x90;
        self.mem[0x03] = 0x90;
        self.mem[0x04] = 0xF0;
        // Ascii 1
        self.mem[0x05] = 0x20;
        self.mem[0x06] = 0x60;
        self.mem[0x07] = 0x20;
        self.mem[0x08] = 0x20;
        self.mem[0x09] = 0x70;
        // Ascii 2
        self.mem[0x0A] = 0xF0;
        self.mem[0x0B] = 0x10;
        self.mem[0x0C] = 0xF0;
        self.mem[0x0D] = 0x80;
        self.mem[0x0E] = 0xF0;
        // Ascii 3
        self.mem[0x0F] = 0xF0;
        self.mem[0x10] = 0x10;
        self.mem[0x11] = 0xF0;
        self.mem[0x12] = 0x10;
        self.mem[0x13] = 0xF0;
        // Ascii 4
        self.mem[0x14] = 0x90;
        self.mem[0x15] = 0x90;
        self.mem[0x16] = 0xF0;
        self.mem[0x17] = 0x10;
        self.mem[0x18] = 0x10;
        // Ascii 5
        self.mem[0x19] = 0xF0;
        self.mem[0x1A] = 0x80;
        self.mem[0x1B] = 0xF0;
        self.mem[0x1C] = 0x10;
        self.mem[0x1D] = 0xF0;
        // Ascii 6
        self.mem[0x1E] = 0xF0;
        self.mem[0x1F] = 0x80;
        self.mem[0x20] = 0xF0;
        self.mem[0x21] = 0x90;
        self.mem[0x22] = 0xF0;
        // Ascii 7
        self.mem[0x23] = 0xF0;
        self.mem[0x24] = 0x10;
        self.mem[0x25] = 0x20;
        self.mem[0x26] = 0x40;
        self.mem[0x27] = 0x40;
        // Ascii 8
        self.mem[0x28] = 0xF0;
        self.mem[0x29] = 0x90;
        self.mem[0x2A] = 0xF0;
        self.mem[0x2B] = 0x90;
        self.mem[0x2C] = 0xF0;
        // Ascii 9
        self.mem[0x2D] = 0xF0;
        self.mem[0x2E] = 0x90;
        self.mem[0x2F] = 0xF0;
        self.mem[0x30] = 0x10;
        self.mem[0x31] = 0xF0;
        // Ascii A
        self.mem[0x32] = 0xF0;
        self.mem[0x33] = 0x90;
        self.mem[0x34] = 0xF0;
        self.mem[0x35] = 0x90;
        self.mem[0x36] = 0x90;
        // Ascii B
        self.mem[0x37] = 0xE0;
        self.mem[0x38] = 0x90;
        self.mem[0x39] = 0xE0;
        self.mem[0x3A] = 0x90;
        self.mem[0x3B] = 0xE0;
        // Ascii C
        self.mem[0x3C] = 0xF0;
        self.mem[0x3D] = 0x80;
        self.mem[0x3E] = 0x80;
        self.mem[0x3F] = 0x80;
        self.mem[0x40] = 0xF0;
        // Ascii D
        self.mem[0x41] = 0xE0;
        self.mem[0x42] = 0x90;
        self.mem[0x43] = 0x90;
        self.mem[0x44] = 0x90;
        self.mem[0x45] = 0xE0;
        // Ascii E
        self.mem[0x46] = 0xF0;
        self.mem[0x47] = 0x80;
        self.mem[0x48] = 0xF0;
        self.mem[0x49] = 0x80;
        self.mem[0x4A] = 0xF0;
        // Ascii F
        self.mem[0x4B] = 0xF0;
        self.mem[0x4C] = 0x80;
        self.mem[0x4D] = 0xF0;
        self.mem[0x4E] = 0x80;
        self.mem[0x4F] = 0x80;
    }

    pub fn get_ascii_slice(&self, val: u8) -> Result<&[u8], String> {
        if val > 0xF {
            return Err(String::from("Ascii address may only go up to 0xF"));
        }
        let start = 5 * val as usize;
        let end = start + 5;
        let slice = &self.mem[start..end];
        Ok(slice)
    }
}
