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
/// Clear the display.
pub fn cls(cpu: &mut CPU) {
    cpu.display_buffer.clear();
}

///00EE - RET
/// Return from a subroutine.
///
/// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
pub fn ret(cpu: &mut CPU) {
    cpu.pc_reg = cpu.stack[cpu.stack_pointer_reg as usize];
    cpu.stack_pointer_reg -= 1;
}

/// 1nnn - JP addr
/// Jump to location nnn.
///
/// The interpreter sets the program counter to nnn.
pub fn jump(cpu: &mut CPU, nnn: u16) {
    cpu.pc_reg = nnn;
}

/// 2nnn - CALL addr
/// Call subroutine at nnn.
///
/// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
pub fn call(cpu: &mut CPU, nnn: u16) {
    cpu.stack_pointer_reg += 1;
    cpu.stack[cpu.stack_pointer_reg as usize] = cpu.pc_reg;
    cpu.pc_reg = nnn;
}

/// 3xkk - SE Vx, byte
/// Skip next instruction if Vx = kk.
///
/// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
pub fn skip_equal(cpu: &mut CPU, vx: u8, kk: u8) {
    if cpu.gp_regs[vx as usize] == kk {
        cpu.pc_reg += 2;
    }
}

/// 4xkk - SNE Vx, byte
/// Skip next instruction if Vx != kk.
///
/// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
pub fn skip_not_equal(cpu: &mut CPU, vx: u8, kk: u8) {
    if cpu.gp_regs[vx as usize] != kk {
        cpu.pc_reg += 2;
    }
}

/// 5xy0 - SE Vx, Vy
/// Skip next instruction if Vx = Vy.
///
/// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
pub fn skip_equal_xy(cpu: &mut CPU, vx: u8, vy: u8) {
    if cpu.gp_regs[vx as usize] == cpu.gp_regs[vy as usize] {
        cpu.pc_reg += 2;
    }
}

/// 6xkk - LD Vx, byte
/// Set Vx = kk.
///
/// The interpreter puts the value kk into register Vx.
pub fn load(cpu: &mut CPU, vx: u8, kk: u8) {
    cpu.gp_regs[vx as usize] = kk;
}

/// 7xkk - ADD Vx, byte
/// Set Vx = Vx + kk.
///
/// Adds the value kk to the value of register Vx, then stores the result in Vx.
pub fn add(cpu: &mut CPU, vx: u8, kk: u8) {
    cpu.gp_regs[vx as usize] += kk;
}

///8xy0 - LD Vx, Vy
/// Set Vx = Vy.
///
/// Stores the value of register Vy in register Vx.
pub fn load_xy(cpu: &mut CPU, vx: u8, vy: u8) {
    cpu.gp_regs[vx as usize] = cpu.gp_regs[vy as usize];
}

///8xy1 - OR Vx, Vy
/// Set Vx = Vx OR Vy.
///
/// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR
/// compares the corresponding bits from two values, and if either bit is 1, then the same bit in the
/// result is also 1. Otherwise, it is 0.
pub fn bitwise_or(cpu: &mut CPU, vx: u8, vy: u8) {
    cpu.gp_regs[vx as usize] = cpu.gp_regs[vy as usize] | cpu.gp_regs[vx as usize];
}

///8xy2 - AND Vx, Vy
/// Set Vx = Vx AND Vy.
///
/// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND
/// compares the corresponding bits from two values, and if both bits are 1, then the same bit in the
/// result is also 1. Otherwise, it is 0.
pub fn bitwise_and(cpu: &mut CPU, vx: u8, vy: u8) {
    cpu.gp_regs[vx as usize] = cpu.gp_regs[vy as usize] & cpu.gp_regs[vx as usize];
}

/// 8xy3 - XOR Vx, Vy
/// Set Vx = Vx XOR Vy.
///
/// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
/// An exclusive OR compares the corresponding bits from two values, and if the bits are not both the
/// same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
pub fn bitwise_xor(cpu: &mut CPU, vx: u8, vy: u8) {
    cpu.gp_regs[vx as usize] = cpu.gp_regs[vy as usize] ^ cpu.gp_regs[vx as usize];
}

/// 8xy4 - ADD Vx, Vy
/// Set Vx = Vx + Vy, set VF = carry.
///
/// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,)
/// VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
pub fn add_xy(cpu: &mut CPU, vx: u8, vy: u8) {
    let mut sum: u16 = cpu.gp_regs[vy as usize] as u16 + cpu.gp_regs[vx as usize] as u16;
    let carry = sum > 255;
    if carry {
        sum -= 255;
        cpu.vf_reg = 1;
    }
    else { cpu.vf_reg = 0; }
    cpu.gp_regs[vx as usize] = sum as u8;
}

///
