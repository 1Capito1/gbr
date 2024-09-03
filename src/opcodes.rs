use crate::cpu::CPU;
use crate::instructions::*;

fn decode(opcode: u8, cpu: &mut CPU) {
    match opcode {
       0x00 => nop(cpu),

       0x01 => {
           cpu.program_counter += 1;
           let lo = cpu.work_ram[cpu.program_counter];
           cpu.program_counter += 1;
           let hi = cpu.work_ram[cpu.program_counter];
           let value = u16::from_le_bytes([hi, lo]);
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
       0x05 => dec(&mut cpu.registers.b),
       0x06 => {
           cpu.program_counter += 1;
           let d8 = cpu.work_ram[cpu.program_counter];
           ld8(&mut cpu.registers.b, d8);
       },
       0x07 => rlca(cpu),
       0x08 => {
           cpu.program_counter += 1;
           let lo = cpu.work_ram[cpu.program_counter];
           cpu.program_counter += 1;
           let hi = cpu.work_ram[cpu.program_counter];
           let address = u16::from_le_bytes([hi, lo]) as usize;
           let lo_sp = cpu.stack_ptr as u8;
           ld_to_memory(lo_sp, address, cpu);
       }
    
       _ => todo!(), 
    }
}
