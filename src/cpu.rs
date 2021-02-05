use crate::memory::Memory;

/// Chip-8 CPU. Contains all Registers and Memory included in the CHIP-8 System.
pub struct CPU {
    /// Memory the CPU is operating on
    pub mem: Memory,
    /// Function Stack - Array of 16x16-bit values (16 words)
    pub stack: [u16; 16],
    /// General purpose registers - 16x8-bit registers stored as u8 array.
    pub gp_regs: [u8; 16],
    /// Index Register - Single 16-bit register commonly used to store addresses
    pub i_reg: u16,
    /// Delay Register - When non-zero, counts down at 60hz
    pub delay_reg: u8,
    /// Sound Register - When non-zero, counts down at 60hz. Tone is played while value is non-zero.
    pub sound_reg: u8,
    /// Program Counter - Single 16-bit register that points to the current memory instruction
    pub pc_reg: u16,
    /// Stack Register - Single 8-bit register that points to the address of the top of the stack.
    pub stack_reg: u8,
    /// Video Register - Single 8-bit register. Set to 1 by interpreter if a pixel collision occurs
    pub vf_reg: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            mem: Memory::new(),
            stack: [0; 16],
            gp_regs: [0; 16],
            i_reg: 0,
            delay_reg: 0,
            sound_reg: 0,
            pc_reg: 0,
            stack_reg: 0,
            vf_reg: 0,
        }
    }
}
