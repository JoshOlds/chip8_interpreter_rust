use crate::cpu::CPU;

use rand::prelude::*;
use std::borrow::Borrow;

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

/// Helper function to combine three nibbles into a 12-bit value
pub fn to_nnn(first: &u8, second: &u8, third: &u8) -> u16 {
    let nnn: u16 = ((*first as u16) << 8) + ((*second as u16) << 4) + *third as u16;
    nnn
}

pub fn to_kk(first: &u8, second: &u8) -> u8 {
    let kk: u8 = (*first << 4) + *second;
    kk
}

pub fn execute(cpu: &mut CPU) -> bool {
    // Fetch instruction
    // Split the 2-byte instruction into four nibbles
    let first = (cpu.mem.mem[cpu.pc_reg as usize] & 0xF0) >> 4;
    let second = cpu.mem.mem[cpu.pc_reg as usize] & 0x0F;
    let third = (cpu.mem.mem[(cpu.pc_reg + 1) as usize] & 0xF0) >> 4;
    let fourth = cpu.mem.mem[(cpu.pc_reg + 1) as usize] & 0x0F;

    // Increment the program counter
    cpu.pc_reg += 2;

    // Decode instruction based on first nibble
    match first {
        0x0 => {
            if third == 0xE {
                if fourth == 0x0 {
                    cls(cpu);
                } else if fourth == 0xE {
                    ret(cpu);
                }
            } else {
                sys(to_nnn(&second, &third, &fourth));
            }
        }
        0x1 => jump(cpu, to_nnn(&second, &third, &fourth)),
        0x2 => call(cpu, to_nnn(&second, &third, &fourth)),
        0x3 => skip_equal(cpu, second, to_kk(&third, &fourth)),
        0x4 => skip_not_equal(cpu, second, to_kk(&third, &fourth)),
        0x5 => skip_equal_xy(cpu, second, third),
        0x6 => load(cpu, second, to_kk(&third, &fourth)),
        0x7 => add(cpu, second, to_kk(&third, &fourth)),
        0x8 => match fourth {
            0x0 => load_xy(cpu, second, third),
            0x1 => bitwise_or(cpu, second, third),
            0x2 => bitwise_and(cpu, second, third),
            0x3 => bitwise_xor(cpu, second, third),
            0x4 => add_xy(cpu, second, third),
            0x5 => sub_xy(cpu, second, third),
            0x6 => shift_right(cpu, second), // TODO: Variant of this opcode
            0x7 => sub_yx(cpu, second, third),
            0xE => shift_left(cpu, second), // TODO: Variant of this opcode
            _ => {
                panic!("Unsupported instruction!");
            }
        },
        0x9 => skip_not_equal_xy(cpu, second, third),
        0xA => load_i(cpu, to_nnn(&second, &third, &fourth)),
        0xB => jump_v0(cpu, to_nnn(&second, &third, &fourth)),
        0xC => rand(cpu, second, to_kk(&third, &fourth)),
        0xD => {
            draw(cpu, second, third, fourth);
            return true;
        }
        0xE => match to_kk(&third, &fourth) {
            0x9E => skip_if_key(cpu, second),
            0xA1 => skip_not_key(cpu, second),
            _ => panic!("Unsupported Instruction!"),
        },
        0xF => match to_kk(&third, &fourth) {
            0x07 => load_delay_timer(cpu, second),
            0x0A => wait_for_key(cpu, second),
            0x15 => load_delay_to_vx(cpu, second),
            0x18 => load_sound_timer(cpu, second),
            0x1E => add_i_vx(cpu, second),
            0x29 => load_ascii_address(cpu, second),
            0x33 => load_bcd(cpu, second),
            0x55 => store_regs(cpu, second),
            0x65 => load_regs(cpu, second),
            _ => panic!("Unsupported Instruction!"),
        },
        _ => {
            panic!("Unsupported instruction!");
        }
    }
    false
}

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
/// The interpreter sets the program counter to the address at the top of the stack, then subtracts
/// 1 from the stack pointer.
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
/// The interpreter increments the stack pointer, then puts the current PC on the top of the stack.
/// The PC is then set to nnn.
pub fn call(cpu: &mut CPU, nnn: u16) {
    cpu.stack_pointer_reg += 1;
    cpu.stack[cpu.stack_pointer_reg as usize] = cpu.pc_reg;
    cpu.pc_reg = nnn;
}

/// 3xkk - SE Vx, byte
/// Skip next instruction if Vx = kk.
///
/// The interpreter compares register Vx to kk, and if they are equal, increments the program
/// counter by 2.
pub fn skip_equal(cpu: &mut CPU, vx: u8, kk: u8) {
    if cpu.gp_regs[vx as usize] == kk {
        cpu.pc_reg += 2;
    }
}

/// 4xkk - SNE Vx, byte
/// Skip next instruction if Vx != kk.
///
/// The interpreter compares register Vx to kk, and if they are not equal, increments the program
/// counter by 2.
pub fn skip_not_equal(cpu: &mut CPU, vx: u8, kk: u8) {
    if cpu.gp_regs[vx as usize] != kk {
        cpu.pc_reg += 2;
    }
}

/// 5xy0 - SE Vx, Vy
/// Skip next instruction if Vx = Vy.
///
/// The interpreter compares register Vx to register Vy, and if they are equal, increments the
/// program counter by 2.
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
    let sum = cpu.gp_regs[vx as usize].overflowing_add(kk).0;
    cpu.gp_regs[vx as usize] = sum;
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
        sum -= 256;
        cpu.vf_reg = 1;
    } else {
        cpu.vf_reg = 0;
    }
    cpu.gp_regs[vx as usize] = sum as u8;
}

/// 8xy5 - SUB Vx, Vy
// Set Vx = Vx - Vy, set VF = NOT borrow.
//
// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx,
// and the results stored in Vx.
pub fn sub_xy(cpu: &mut CPU, vx: u8, vy: u8) {
    if cpu.gp_regs[vx as usize] > cpu.gp_regs[vy as usize] {
        cpu.vf_reg = 1;
    } else {
        cpu.vf_reg = 0;
    }
    let mut diff = cpu.gp_regs[vx as usize]
        .overflowing_sub(cpu.gp_regs[vy as usize])
        .0;
    cpu.gp_regs[vx as usize] = diff;
}

/// 8xy6 - SHR Vx {, Vy}
/// Set Vx = Vx SHR 1.
///
/// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
/// Then Vx is divided by 2.
///
pub fn shift_right(cpu: &mut CPU, vx: u8) {
    if cpu.gp_regs[vx as usize] & 0x01 > 0 {
        cpu.vf_reg = 1;
    } else {
        cpu.vf_reg = 0;
    }
    cpu.gp_regs[vx as usize] = cpu.gp_regs[vx as usize] >> 1;
}

/// 8xy7 - SUBN Vx, Vy
/// Set Vx = Vy - Vx, set VF = NOT borrow.
///
/// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results
/// stored in Vx.
pub fn sub_yx(cpu: &mut CPU, vx: u8, vy: u8) {
    if cpu.gp_regs[vy as usize] > cpu.gp_regs[vx as usize] {
        cpu.vf_reg = 1;
    } else {
        cpu.vf_reg = 0;
    }
    let temp_y = cpu.gp_regs[vy as usize];
    let mut diff = temp_y.overflowing_sub(cpu.gp_regs[vy as usize]).0;
    cpu.gp_regs[vx as usize] = diff;
}

/// 8xyE - SHL Vx {, Vy}
/// Set Vx = Vx SHL 1.
///
/// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
/// Then Vx is multiplied by 2.
pub fn shift_left(cpu: &mut CPU, vx: u8) {
    if cpu.gp_regs[vx as usize] & 0x80 > 0 {
        cpu.vf_reg = 1;
    } else {
        cpu.vf_reg = 0;
    }
    cpu.gp_regs[vx as usize] = cpu.gp_regs[vx as usize] << 1;
}

/// 9xy0 - SNE Vx, Vy
/// Skip next instruction if Vx != Vy.
///
/// The values of Vx and Vy are compared, and if they are not equal,
/// the program counter is increased by 2.
pub fn skip_not_equal_xy(cpu: &mut CPU, vx: u8, vy: u8) {
    if cpu.gp_regs[vx as usize] != cpu.gp_regs[vy as usize] {
        cpu.pc_reg += 2;
    }
}

/// Annn - LD I, addr
/// Set I = nnn.
///
/// The value of register I is set to nnn.
pub fn load_i(cpu: &mut CPU, nnn: u16) {
    cpu.i_reg = nnn;
}

/// Bnnn - JP V0, addr
/// Jump to location nnn + V0.
///
/// The program counter is set to nnn plus the value of V0.
pub fn jump_v0(cpu: &mut CPU, nnn: u16) {
    cpu.pc_reg += cpu.gp_regs[0] as u16 + nnn;
}

/// Cxkk - RND Vx, byte
/// Set Vx = random byte AND kk.
///
/// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
/// The results are stored in Vx. See instruction 8xy2 for more information on AND.
pub fn rand(cpu: &mut CPU, vx: u8, kk: u8) {
    cpu.gp_regs[vx as usize] = rand::random::<u8>() & kk;
}

/// Dxyn - DRW Vx, Vy, nibble
/// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
///
/// The interpreter reads n bytes from memory, starting at the address stored in I.
/// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
/// Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
/// VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is
/// outside the coordinates of the display, it wraps around to the opposite side of the screen.
/// See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information
/// on the Chip-8 screen and sprites.
pub fn draw(cpu: &mut CPU, vx: u8, vy: u8, n: u8) {
    let x_coord = cpu.gp_regs[vx as usize] as i32;
    let y_coord = cpu.gp_regs[vy as usize] as i32;
    let sprite_addr = cpu.i_reg as usize;
    let sprite_slice: &[u8] = &cpu.mem.mem[sprite_addr..sprite_addr + n as usize];
    let collision = cpu
        .display_buffer
        .write_sprite(x_coord, y_coord, sprite_slice);
    if collision {
        cpu.vf_reg = 1;
    } else {
        cpu.vf_reg = 0;
    }
}
/// Ex9E - SKP Vx
/// Skip next instruction if key with the value of Vx is pressed.
///
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the
/// down position, PC is increased by 2.
pub fn skip_if_key(cpu: &mut CPU, vx: u8) {
    if cpu.keyboard.is_pressed(cpu.gp_regs[vx as usize]) {
        cpu.pc_reg += 2;
    }
}

/// ExA1 - SKNP Vx
/// Skip next instruction if key with the value of Vx is not pressed.
///
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the
/// up position, PC is increased by 2.
pub fn skip_not_key(cpu: &mut CPU, vx: u8) {
    if cpu.keyboard.is_pressed(cpu.gp_regs[vx as usize]) == false {
        cpu.pc_reg += 2;
    }
}

/// Fx07 - LD Vx, DT
/// Set Vx = delay timer value.
///
/// The value of DT is placed into Vx.
pub fn load_delay_to_vx(cpu: &mut CPU, vx: u8) {
    cpu.gp_regs[vx as usize] = cpu.delay_reg;
}

/// Fx0A - LD Vx, K
/// Wait for a key press, store the value of the key in Vx.
///
/// All execution stops until a key is pressed, then the value of that key is stored in Vx.
pub fn wait_for_key(cpu: &mut CPU, vx: u8) {
    let mut key: u8 = 0x0;
    loop {
        let keys = cpu.keyboard.get_active_inputs();
        if keys.is_empty() == false {
            key = keys[0].to_hex();
            break;
        }
    }
    cpu.gp_regs[vx as usize] = key;
}

/// Fx15 - LD DT, Vx
/// Set delay timer = Vx.
///
/// DT is set equal to the value of Vx.
pub fn load_delay_timer(cpu: &mut CPU, vx: u8) {
    cpu.delay_reg = cpu.gp_regs[vx as usize];
}

/// Fx18 - LD ST, Vx
/// Set sound timer = Vx.
///
/// ST is set equal to the value of Vx.
pub fn load_sound_timer(cpu: &mut CPU, vx: u8) {
    cpu.sound_reg = cpu.gp_regs[vx as usize];
}

/// Fx1E - ADD I, Vx
/// Set I = I + Vx.
///
/// The values of I and Vx are added, and the results are stored in I.
pub fn add_i_vx(cpu: &mut CPU, vx: u8) {
    cpu.i_reg = cpu.i_reg + cpu.gp_regs[vx as usize] as u16;
}

/// Fx29 - LD F, Vx
/// Set I = location of sprite for digit Vx.
///
/// The value of I is set to the location for the hexadecimal sprite corresponding to the value of
/// Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
pub fn load_ascii_address(cpu: &mut CPU, vx: u8) {
    cpu.i_reg = cpu.gp_regs[vx as usize] as u16 * 5;
}

/// Fx33 - LD B, Vx
/// Store BCD representation of Vx in memory locations I, I+1, and I+2.
///
/// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
/// location in I, the tens digit at location I+1, and the ones digit at location I+2.
pub fn load_bcd(cpu: &mut CPU, vx: u8) {
    let val = cpu.gp_regs[vx as usize];
    cpu.mem.mem[cpu.i_reg as usize] = val / 100;
    cpu.mem.mem[(cpu.i_reg + 1) as usize] = (val % 100) / 10;
    cpu.mem.mem[(cpu.i_reg + 2) as usize] = val % 10;
}

/// Fx55 - LD [I], Vx
/// Store registers V0 through Vx in memory starting at location I.
///
/// The interpreter copies the values of registers V0 through Vx into memory,
/// starting at the address in I.
pub fn store_regs(cpu: &mut CPU, vx: u8) {
    let regs = &cpu.gp_regs[0..=vx as usize];
    for (index, val) in regs.iter().enumerate() {
        cpu.mem.mem[cpu.i_reg as usize + index] = *val;
    }
}

/// Fx65 - LD Vx, [I]
/// Read registers V0 through Vx from memory starting at location I.
///
/// The interpreter reads values from memory starting at location I into registers V0 through Vx.
pub fn load_regs(cpu: &mut CPU, vx: u8) {
    for n in 0..=vx {
        cpu.gp_regs[n as usize] = cpu.mem.mem[cpu.i_reg as usize + n as usize];
    }
}
