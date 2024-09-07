use crate::cpu::CPU;
use crate::instructions::*;

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

        0x03 => inc16(&mut cpu.registers.get_bc()),
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
        0x0B => dec16(&mut cpu.registers.get_bc()),
        0x0C => inc8(&mut cpu.registers.c),
        0x0D => dec8(&mut cpu.registers.c),
        
        0x0E => {
            cpu.program_counter += 1;
            let value = cpu.work_ram[cpu.program_counter];
            ld8(&mut cpu.registers.c, value);
        },

        0x0F => rrca(cpu),

        // 0x1N Instructions
        0x10 => cpu.do_stop = true,
        0x11 => {
            let value = cpu.get_next_two_bytes();
            ld16(&mut cpu.registers.get_de(), value);
        },
        0x12 => ld_to_memory(cpu.registers.a, cpu.work_ram[cpu.registers.get_de() as usize] as usize, cpu),
        0x13 => inc16(&mut cpu.registers.get_de()),
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
        0x1B => dec16(&mut cpu.registers.get_de()),
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
            }
        },
        0x21 => {
            let d16 = cpu.get_next_two_bytes();
            ld16(&mut cpu.registers.get_hl(), d16);
        },
        0x22 => {
            let hl = cpu.registers.get_hl();
            ld_to_memory(cpu.registers.a, hl as usize, cpu);
            cpu.registers.set_hl(hl.wrapping_add(1));
        },
        0x23 => {
            inc16(&mut cpu.registers.get_hl());
        },
        0x24 => inc8(&mut cpu.registers.h),
        0x25 => dec8(&mut cpu.registers.h),
        0x26 => {
            let d8 = cpu.get_next_one_byte();
            ld8(&mut cpu.registers.h, d8);
        },
        0x27 => {

        }
        _ => todo!(), 
    }
}
