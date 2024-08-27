use crate::cpu::CPU;

/// Load register with value. value can be from a 8-bit register
/// or it can be an immediate value
fn ld8(register: &mut u8, value: u8) {
    *register = value;
}

/// same as ld() but with 16-bit registers
/// or immediate value
fn ld16(register: &mut u16, value: u16) {
    *register = value;
}

/// loads the value in work ram[address] into register.
/// invalid memory accesses will cause a panic
fn ld_from_memory(register: &mut u8, address: usize, cpu: &mut CPU) {
    if address >= cpu.work_ram.len() { 
        panic!("Address {:#x} outside of valid memory range. Max range {:#x}", address, cpu.work_ram.len()); 
    }
    *register = cpu.work_ram[address];
}

/// loads the value of register into work ram[address]
fn ld_to_memory(register: &mut u8, address: usize, cpu: &mut CPU) {
    if address >= cpu.work_ram.len() { 
        panic!("Address {:#x} outside of valid memory range. Max range {:#x}", address, cpu.work_ram.len()); 
    }
    cpu.work_ram[address] = *register;
}

/// push 16-bit register onto stack
fn push(register: &mut u16, cpu: &mut CPU) {
    let most_significant = ((*register >> 8) & 0xFF) as u8;
    let least_significant = (*register & 0xFF) as u8;
    cpu.stack_ptr -= 1;
    cpu.work_ram[cpu.stack_ptr] = most_significant;
    cpu.stack_ptr -= 1;
    cpu.work_ram[cpu.stack_ptr] = least_significant;
}

/// pop 16-bit register off of stack
fn pop(register: &mut u16, cpu: &mut CPU) {
    let most_significant = cpu.work_ram[cpu.stack_ptr] as u16;
    cpu.stack_ptr += 1;
    let least_significant = cpu.work_ram[cpu.stack_ptr] as u16;
    *register = (most_significant << 8) | least_significant;
    cpu.stack_ptr += 1;
}

/// add value of register B into register A
/// register B could also be an immediate value
fn add(register_a: &mut u8, register_b: u8, cpu: &mut CPU) {
    let (res, _) = register_a.overflowing_add(register_b);
    *register_a = res;

    cpu.registers.flags.z = *register_a == 0;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = ((*register_a & 0xF) + (register_b & 0xF)) > 0xF;
}

/// add value at memory address to register, panicing if address is out of bounds
fn add_from_memory(register: &mut u8, address: usize, cpu: &mut CPU) {
    if address >= cpu.work_ram.len() {
        panic!("Address {:#x} outside of valid memory range. Max range {:#x}", address, cpu.work_ram.len());
    }
    let (res, _) = register.overflowing_add(cpu.work_ram[address]);
    *register = res;

    cpu.registers.flags.z = *register == 0;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = ((*register & 0xF) + (cpu.work_ram[address] & 0xF)) > 0xF;
}

/// ADD with carry. If there is overflow, set carry flag to true, else false
fn addc(register: &mut u8, register_b: u8, cpu: &mut CPU) {
    let carry = if cpu.registers.flags.c { 1 } else { 0 };
    let (res, overflow) = register.overflowing_add(register_b.wrapping_add(carry));
    *register = res;

    cpu.registers.flags.z = *register == 0;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = ((*register & 0xF) + (register_b & 0xF)) > 0xF;
    cpu.registers.flags.c = overflow;
}

/// ADD from memory with carry. If there is overflow, set carry flag to true, else false
fn addc_from_memory(register: &mut u8, address: usize, cpu: &mut CPU) {
    let carry = if cpu.registers.flags.c { 1 } else { 0 };
    let (res, overflow) = register.overflowing_add(cpu.work_ram[address].wrapping_add(carry));
    *register = res;

    cpu.registers.flags.z = *register == 0;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = ((*register & 0xF) + (cpu.work_ram[address] & 0xF)) > 0xF;
    cpu.registers.flags.c = overflow;
}
