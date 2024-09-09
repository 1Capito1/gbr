use crate::cpu::CPU;
use crate::instructions::*;
use crate::cpu::CpuState;

/// All opcode information can be found at
/// [this beautiful opcode table](https://meganesu.github.io/generate-gb-opcodes/)

fn decode(opcode: u8, cpu: &mut CPU) {
    match opcode {
        // 0x0N instructions
        0x00 => nop(cpu),

        0x01 => {
            let value = cpu.get_next_two_bytes();
            let register = &mut cpu.registers.get_bc();
            ld16(register, value);
        },

        0x02 => {
            let address = cpu.registers.get_bc() as usize;
            let register = cpu.registers.a;

            ld_to_memory(register, address, cpu);
        },

        0x03 => cpu.registers.inc_bc(),
        0x04 => inc8(&mut cpu.registers.b),
        0x05 => dec8(&mut cpu.registers.b),
        0x06 => {
            cpu.program_counter += 1;
            let d8 = cpu.work_ram[cpu.program_counter];
            ld8(&mut cpu.registers.b, d8);
        },
        0x07 => rlca(cpu),

        0x08 => {
            let address = cpu.get_next_two_bytes() as usize;      
            let lo_sp = cpu.stack_ptr as u8;
            ld_to_memory(lo_sp, address, cpu);
        },
        0x09 => add16(&mut cpu.registers.get_hl(), cpu.registers.get_bc(), cpu),
        0x0A => {
            let reg_bc = cpu.registers.get_bc();
            let reg_a = &mut cpu.registers.a;
            let value = cpu.work_ram[reg_bc as usize] as u8;
            ld8(reg_a, value);
        },
        0x0B => cpu.registers.dec_bc(),
        0x0C => inc8(&mut cpu.registers.c),
        0x0D => dec8(&mut cpu.registers.c),
        
        0x0E => {
            cpu.program_counter += 1;
            let value = cpu.work_ram[cpu.program_counter];
            ld8(&mut cpu.registers.c, value);
        },

        0x0F => rrca(cpu),

        // 0x1N Instructions
        0x10 => cpu.state = CpuState::STOP,
        0x11 => {
            let value = cpu.get_next_two_bytes();
            ld16(&mut cpu.registers.get_de(), value);
        },
        0x12 => ld_to_memory(cpu.registers.a, cpu.work_ram[cpu.registers.get_de() as usize] as usize, cpu),
        0x13 => cpu.registers.inc_de(),
        0x14 => inc8(&mut cpu.registers.d),
        0x15 => dec8(&mut cpu.registers.d),
        0x16 => {
            let value = cpu.get_next_one_byte();
            ld8(&mut cpu.registers.d, value);
        },
        0x17 => rla(cpu),
        0x18 => {
            let offset = cpu.get_next_one_byte() as i8;
            jr(offset, cpu);
        },
        0x19 => add16(&mut cpu.registers.get_hl(), cpu.registers.get_de(), cpu),
        0x1A => {
            let reg_de = cpu.registers.get_de() as usize;
            let reg_a = &mut cpu.registers.a;
            let ram = cpu.work_ram;
            ld_from_memory(reg_a, reg_de, ram);
        },
        0x1B => cpu.registers.dec_de(),
        0x1C => inc8(&mut cpu.registers.e),
        0x1D => dec8(&mut cpu.registers.e),
        0x1E => {
            let immediate = cpu.get_next_one_byte();
            ld8(&mut cpu.registers.e, immediate);
        },
        0x1F => rra(cpu),

        // 0x2N instructions
        0x20 => {
            if !cpu.registers.flags.z {
                let s8 = cpu.get_next_one_byte() as i8;
                jr(s8, cpu);
            } else {
                cpu.program_counter += 1;
            }
        },
        0x21 => {
            let d16 = cpu.get_next_two_bytes();
            ld16(&mut cpu.registers.get_hl(), d16);
        },
        0x22 => {
            let hl = cpu.registers.get_hl();
            ld_to_memory(cpu.registers.a, hl as usize, cpu);
            cpu.registers.inc_hl();
        }
        0x23 => cpu.registers.inc_hl(),
        0x24 => inc8(&mut cpu.registers.h),
        0x25 => dec8(&mut cpu.registers.h),
        0x26 => {
            let d8 = cpu.get_next_one_byte();
            ld8(&mut cpu.registers.h, d8);
        },
        0x27 => daa(cpu),
        0x28 => {
            if cpu.registers.flags.z {
                let s8 = cpu.get_next_one_byte() as i8;
                jr(s8, cpu);
            } else {
                cpu.program_counter += 1;
            }
        },
        0x29 => {
            let mut hl = cpu.registers.get_hl();
            hl = hl.wrapping_add(hl);
            cpu.registers.set_hl(hl);
        },
        0x2A => {
            let hl = cpu.registers.get_hl();
            let a = &mut cpu.registers.a;
            ld_from_memory(a, hl as usize, cpu.work_ram);
            cpu.registers.set_hl(hl.wrapping_add(1));
        },
        0x2B => cpu.registers.dec_hl(),
        0x2C => inc8(&mut cpu.registers.l),
        0x2D => dec8(&mut cpu.registers.l),
        0x2E => {
            let d8 = cpu.get_next_one_byte();
            ld8(&mut cpu.registers.l, d8);
        },
        0x2F => cpl(cpu),

        // 0x3N instructions
        0x30 => {
            if !cpu.registers.flags.c {
                let s8 = cpu.get_next_one_byte() as i8;
                jr(s8, cpu);
            } else {
                cpu.program_counter += 1;
            }
        },
        0x31 => {
            let d16 = cpu.get_next_two_bytes();
            cpu.stack_ptr = d16 as usize;
        },
        0x32 => {
            ld_to_memory(cpu.registers.a, cpu.registers.get_hl() as usize, cpu);
            cpu.registers.dec_hl();
        },
        0x33 => {
            cpu.stack_ptr += 1;
        },
        0x34 => {
            cpu.work_ram[cpu.registers.get_hl() as usize] += 1;
        },
        0x35 => {
            cpu.work_ram[cpu.registers.get_hl() as usize] -= 1;
        },
        0x36 => {
            let d8 = cpu.get_next_one_byte();
            ld_to_memory(d8, cpu.registers.get_hl() as usize, cpu);
        },
        0x37 => {
            cpu.registers.flags.c = true;
        },
        0x38 => {
            if cpu.registers.flags.c {
                let s8 = cpu.get_next_one_byte() as i8;
                jr(s8, cpu);
            } else {
                cpu.program_counter += 1;
            }
        },
        0x39 => {
            add16(&mut cpu.registers.get_hl(), cpu.stack_ptr as u16, cpu);
        },
        0x3A => {
            let reg_hl = cpu.registers.get_hl();
            ld8(&mut cpu.registers.a, cpu.work_ram[reg_hl as usize]);
            cpu.registers.dec_hl();
        },
        0x3B => {
            cpu.stack_ptr -= 1;
        },
        0x3C => {
            inc8(&mut cpu.registers.a);
        },
        0x3D => {
            dec8(&mut cpu.registers.a);
        },
        0x3E => {
            let d8 = cpu.get_next_one_byte();
            ld8(&mut cpu.registers.a, d8);
        },
        0x3F => {
            cpu.registers.flags.c = !cpu.registers.flags.c;
        },

        // 0x4N instructions
        0x40 => {
            let reg_b = cpu.registers.b;
            ld8(&mut cpu.registers.b, reg_b);
        }
        0x41 => ld8(&mut cpu.registers.b, cpu.registers.c),
        0x42 => ld8(&mut cpu.registers.b, cpu.registers.d),
        0x43 => ld8(&mut cpu.registers.b, cpu.registers.e),
        0x44 => ld8(&mut cpu.registers.b, cpu.registers.h),
        0x45 => ld8(&mut cpu.registers.b, cpu.registers.l),
        0x46 => {
            let reg_hl = cpu.registers.get_hl();
            ld_from_memory(&mut cpu.registers.b, reg_hl as usize, cpu.work_ram);
        }
        0x47 => ld8(&mut cpu.registers.b, cpu.registers.a),
        0x48 => ld8(&mut cpu.registers.c, cpu.registers.b),
        0x49 => {
            let reg_c = cpu.registers.c;
            ld8(&mut cpu.registers.c, reg_c);
        },
        0x4A => ld8(&mut cpu.registers.c, cpu.registers.d),
        0x4B => ld8(&mut cpu.registers.c, cpu.registers.e),
        0x4C => ld8(&mut cpu.registers.c, cpu.registers.h),
        0x4D => ld8(&mut cpu.registers.c, cpu.registers.l),
        0x4E => {
            let reg_hl = cpu.registers.get_hl();
            ld_from_memory(&mut cpu.registers.b, reg_hl as usize, cpu.work_ram);
        },
        0x4F => ld8(&mut cpu.registers.c, cpu.registers.a),

        // 0x5N instructions
        0x50 => ld8(&mut cpu.registers.d, cpu.registers.b),
        0x51 => ld8(&mut cpu.registers.d, cpu.registers.c),
        0x52 => {
            let reg_d = cpu.registers.d;
            ld8(&mut cpu.registers.d, reg_d);
        },
        0x53 => ld8(&mut cpu.registers.d, cpu.registers.e),
        0x54 => ld8(&mut cpu.registers.d, cpu.registers.h),
        0x55 => ld8(&mut cpu.registers.d, cpu.registers.l),
        0x56 => {
            let reg_hl = cpu.registers.get_hl();
            ld_from_memory(&mut cpu.registers.d, reg_hl as usize, cpu.work_ram);
        },
        0x57 => ld8(&mut cpu.registers.d, cpu.registers.a),
        0x58 => ld8(&mut cpu.registers.e, cpu.registers.b),
        0x59 => ld8(&mut cpu.registers.e, cpu.registers.c),
        0x5A => ld8(&mut cpu.registers.e, cpu.registers.d),
        0x5B => {
            let reg_e = cpu.registers.e;
            ld8(&mut cpu.registers.e, reg_e);
        },
        0x5C => ld8(&mut cpu.registers.e, cpu.registers.h),
        0x5D => ld8(&mut cpu.registers.e, cpu.registers.l),
        0x5E => {
            let reg_hl = cpu.registers.get_hl();
            ld_from_memory(&mut cpu.registers.e, reg_hl as usize, cpu.work_ram);
        },
        0x5F => ld8(&mut cpu.registers.e, cpu.registers.a),
        
        // 0x6N instructions
        0x60 => ld8(&mut cpu.registers.h, cpu.registers.b),
        0x61 => ld8(&mut cpu.registers.h, cpu.registers.c),
        0x62 => ld8(&mut cpu.registers.h, cpu.registers.d),
        0x63 => ld8(&mut cpu.registers.h, cpu.registers.e),
        0x64 => {
            let reg_h = cpu.registers.h;
            ld8(&mut cpu.registers.h, reg_h);
        },
        0x65 => ld8(&mut cpu.registers.h, cpu.registers.l),
        0x66 => {
            let reg_hl = cpu.registers.get_hl();
            ld_from_memory(&mut cpu.registers.h, reg_hl as usize, cpu.work_ram);
        },
        0x67 => ld8(&mut cpu.registers.h, cpu.registers.a),
        0x68 => ld8(&mut cpu.registers.l, cpu.registers.b),
        0x69 => ld8(&mut cpu.registers.l, cpu.registers.c),
        0x6A => ld8(&mut cpu.registers.l, cpu.registers.d),
        0x6B => ld8(&mut cpu.registers.l, cpu.registers.e),
        0x6C => ld8(&mut cpu.registers.l, cpu.registers.h),
        0x6D => {
            let reg_l = cpu.registers.l;
            ld8(&mut cpu.registers.l, reg_l);
        },
        0x6E => {
            let reg_hl = cpu.registers.get_hl();
            ld_from_memory(&mut cpu.registers.l, reg_hl as usize, cpu.work_ram);
        },
        0x6F => ld8(&mut cpu.registers.l, cpu.registers.a),
        
        // 0x7N instructions
        0x70 => ld_to_memory(cpu.registers.b, cpu.registers.get_hl() as usize, cpu),
        0x71 => ld_to_memory(cpu.registers.c, cpu.registers.get_hl() as usize, cpu),
        0x72 => ld_to_memory(cpu.registers.d, cpu.registers.get_hl() as usize, cpu),
        0x73 => ld_to_memory(cpu.registers.e, cpu.registers.get_hl() as usize, cpu),
        0x74 => ld_to_memory(cpu.registers.h, cpu.registers.get_hl() as usize, cpu),
        0x75 => ld_to_memory(cpu.registers.l, cpu.registers.get_hl() as usize, cpu),
        0x76 => cpu.state = CpuState::HALT,
        0x77 => ld_to_memory(cpu.registers.a, cpu.registers.get_hl() as usize, cpu),
        0x78 => ld8(&mut cpu.registers.a, cpu.registers.b),
        0x79 => ld8(&mut cpu.registers.a, cpu.registers.c),
        0x7A => ld8(&mut cpu.registers.a, cpu.registers.d),
        0x7B => ld8(&mut cpu.registers.a, cpu.registers.e),
        0x7C => ld8(&mut cpu.registers.a, cpu.registers.h),
        0x7D => ld8(&mut cpu.registers.a, cpu.registers.l),
        0x7E => {
            let reg_hl = cpu.registers.get_hl();
            ld_from_memory(&mut cpu.registers.a, reg_hl as usize, cpu.work_ram);
        },
        0x7F => {
            let reg_a = cpu.registers.a;
            ld8(&mut cpu.registers.a, reg_a);
        },
        // 0x8N instructions
        //
        // 0x9N instructions
        //
        // 0xAN instructions
        //
        // 0xBN instructions
        //
        // 0xCN instructions
        //
        // 0xDN instructions
        //
        // 0xEN instructions
        //
        // 0xFN instructions

        _ => todo!(), 
    }
}
