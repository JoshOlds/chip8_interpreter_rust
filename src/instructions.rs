use crate::cpu::CPU;

/// Documentation pulled from CowGod's CHIP-8 Reference page
/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
///

/// In these listings, the following variables are used:
//
// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
// kk or byte - An 8-bit value, the lowest 8 bits of the instruction

/// 0nnn - SYS addr
/// Jump to a machine code routine at nnn.
//
// This instruction is only used on the old computers on which Chip-8 was originally implemented.
// It is ignored by modern interpreters.
pub fn sys(_nnn: u16) {
    return;
}

/// 00E0 - CLS
// Clear the display.
pub fn cls(cpu: &mut CPU) {
    cpu.display_reference.clear_screen();
}
