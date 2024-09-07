use crate::cpu::CPU;

/// Load register with value. value can be from a 8-bit register
/// or it can be an immediate value
pub fn ld8(register: &mut u8, value: u8) {
    *register = value;
}

/// same as ld() but with 16-bit registers
/// or immediate value
pub fn ld16(register: &mut u16, value: u16) {
    *register = value;
}

/// loads the value in work ram[address] into register.
/// invalid memory accesses will cause a panic
pub fn ld_from_memory(register: &mut u8, address: usize, ram: [u8; 8192]) {
    if address >= ram.len() { 
        panic!("Address {:#x} outside of valid memory range. Max range {:#x}", address, ram.len()); 
    }
    *register = ram[address];
}

/// loads the value of register into work ram[address]
pub fn ld_to_memory(register: u8, address: usize, cpu: &mut CPU) {
    if address >= cpu.work_ram.len() { 
        panic!("Address {:#x} outside of valid memory range. Max range {:#x}", address, cpu.work_ram.len()); 
    }
    cpu.work_ram[address] = register;
}

pub fn inc8(register: &mut u8) { *register += 1; }
pub fn dec8(register: &mut u8) { *register -= 1;}

/// push 16-bit register onto stack
pub fn push(register: &mut u16, cpu: &mut CPU) {
    let most_significant = ((*register >> 8) & 0xFF) as u8;
    let least_significant = (*register & 0xFF) as u8;
    cpu.stack_ptr -= 1;
    cpu.work_ram[cpu.stack_ptr] = most_significant;
    cpu.stack_ptr -= 1;
    cpu.work_ram[cpu.stack_ptr] = least_significant;
}

/// pop 16-bit register off of stack
pub fn pop(register: &mut u16, cpu: &mut CPU) {
    let most_significant = cpu.work_ram[cpu.stack_ptr] as u16;
    cpu.stack_ptr += 1;
    let least_significant = cpu.work_ram[cpu.stack_ptr] as u16;
    *register = (most_significant << 8) | least_significant;
    cpu.stack_ptr += 1;
}

/// add value of register B into register A
/// register B could also be an immediate value
pub fn add8(register_a: &mut u8, register_b: u8, cpu: &mut CPU) {
    let (res, _) = register_a.overflowing_add(register_b);
    *register_a = res;

    cpu.registers.flags.z = *register_a == 0;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = ((*register_a & 0xF) + (register_b & 0xF)) > 0xF;
}
pub fn add16(register_a: &mut u16, register_b: u16, cpu: &mut CPU) {
    let (res, _) = register_a.overflowing_add(register_b);
    *register_a = res;

    cpu.registers.flags.z = *register_a == 0;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = ((*register_a & 0xF) + (register_b & 0xF)) > 0xF;
}

/// add value at memory address to register, panicing if address is out of bounds
pub fn add_from_memory(register: &mut u8, address: usize, cpu: &mut CPU) {
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
pub fn addc(register: &mut u8, register_b: u8, cpu: &mut CPU) {
    let carry = if cpu.registers.flags.c { 1 } else { 0 };
    let (res, overflow) = register.overflowing_add(register_b.wrapping_add(carry));
    *register = res;

    cpu.registers.flags.z = *register == 0;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = ((*register & 0xF) + (register_b & 0xF)) > 0xF;
    cpu.registers.flags.c = overflow;
}

/// ADD from memory with carry. If there is overflow, set carry flag to true, else false
pub fn addc_from_memory(register: &mut u8, address: usize, cpu: &mut CPU) {
    let carry = if cpu.registers.flags.c { 1 } else { 0 };
    let (res, overflow) = register.overflowing_add(cpu.work_ram[address].wrapping_add(carry));
    *register = res;

    cpu.registers.flags.z = *register == 0;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = ((*register & 0xF) + (cpu.work_ram[address] & 0xF)) > 0xF;
    cpu.registers.flags.c = overflow;
}

/// sub the value of register in to register A without carry
pub fn sub(register: &mut u8, cpu: &mut CPU) {
    let (res, _) = register.overflowing_sub(cpu.registers.a);
    cpu.registers.a = res;

    cpu.registers.flags.z = *register == 0;
    cpu.registers.flags.n = true;
    cpu.registers.flags.h = (*register & 0xF) < (cpu.registers.a & 0xF)
}

/// [sub] with carry
pub fn sbc(register: &mut u8, cpu: &mut CPU) {
    let carry = if cpu.registers.flags.c { 1 } else { 0 };
    let (res, borrow) = register.overflowing_sub(cpu.registers.a);
    let (res, borrow2) = res.overflowing_sub(carry);

    cpu.registers.flags.z = res == 0;
    cpu.registers.flags.n = true;
    cpu.registers.flags.h = (cpu.registers.a & 0xF) < (*register & 0xF) || ((res & 0xF) + carry) > (cpu.registers.a & 0xF);
    cpu.registers.flags.c = borrow || borrow2;
    cpu.registers.a = res;
}

/// logical AND with register into register A
pub fn and(register: u8, cpu: &mut CPU) {
    cpu.registers.a &= register;
}
/// logical OR with register into register A
pub fn or(register: u8, cpu: &mut CPU) {
    cpu.registers.a |= register;
}
/// logical XOR with A into A
pub fn xor(register: u8, cpu: &mut CPU) {
    cpu.registers.a ^= register;
}
/// compare, compares register with A.
/// Effectively a [sub] with the while ignoring the result
pub fn cp(register: u8, cpu: &mut CPU) {
    let (_, carry) = register.overflowing_sub(cpu.registers.a);
    cpu.registers.flags.z = register == 0;
    cpu.registers.flags.n = true;
    cpu.registers.flags.h = (register & 0xF) < (cpu.registers.a & 0xF);
    cpu.registers.flags.c = carry;
}

/// Jumps to address in register, may panic if attempting to access out of bounds memory
pub fn jp(register: u16, cpu: &mut CPU) {
    if register as usize >= cpu.work_ram.len() {
        panic!("out of bounds jump: attemped to jump to {}", register);
    }
    cpu.program_counter = register as usize;
}

/// Jumps to address in 8-bit register relative to program counter
pub fn jr(offset: i8, cpu: &mut CPU) {
    let target_address;
    if offset > 0 {
        target_address = cpu.program_counter.wrapping_add(offset as usize);
    }
    else {
        target_address = cpu.program_counter.wrapping_sub(offset as usize);
    }

    if target_address >= cpu.work_ram.len() {
        panic!("Out of bounds jump: attempted to jump to address 0x{:04X}", target_address);
    }
    cpu.program_counter = target_address;
}

/// pushes PC onto stack, then sets PC to address
pub fn call(address: u16, cpu: &mut CPU) {
    // push current pc onto the stack. since the stack is &[u8] and pc is u16 
    // the pc is split into two u8's
    cpu.stack_ptr -= 1;
    cpu.work_ram[cpu.stack_ptr] = (cpu.program_counter & 0xFF) as u8;
    cpu.stack_ptr -= 1;
    cpu.work_ram[cpu.stack_ptr] = ((cpu.program_counter >> 8) & 0xFF) as u8;

    // check if address is within range
    // since nothing can be done if the
    // address is outside of range, the
    // program panics
    if address as usize >= cpu.work_ram.len() {
        panic!("Out of bounds jump: attempted to jump to address 0x{:04X}", address);
    }

    cpu.program_counter = address as usize;
}

/// returns from a subroutine
/// incrementing the stack ptr
/// by two in the process
pub fn ret(cpu: &mut CPU) {
    cpu.stack_ptr += 1;
    let low_half = cpu.work_ram[cpu.stack_ptr];
    cpu.stack_ptr += 1;
    let high_half = cpu.work_ram[cpu.stack_ptr];
    
    cpu.program_counter = u16::from_le_bytes([high_half, low_half]) as usize;
}

/// increments the program counter by one
pub fn nop(cpu: &mut CPU) {
    cpu.program_counter += 1;
}

pub fn rlca(cpu: &mut CPU) {
    let carry = (cpu.registers.a & 0x80) != 0;

    cpu.registers.a = (cpu.registers.a << 1) | (carry as u8);

    cpu.registers.flags.z = cpu.registers.a == 0;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = false;
    cpu.registers.flags.c = carry;
}
/// ex A = b1011010 and carry = 1
/// = b10110101
/// = b0110101 and c = 1
pub fn rla(cpu: &mut CPU) {
    let carry_after_rotate = (cpu.registers.a & 0x80) != 0;
    
    cpu.registers.a = (cpu.registers.a << 1) | (cpu.registers.flags.c as u8);

    cpu.registers.a &= cpu.registers.c;
    cpu.registers.flags.c = carry_after_rotate;

    cpu.registers.flags.z = cpu.registers.a == 0;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = false;
}
/// rotate contents of register a to the right, with the circular bit
/// going into carry flag
pub fn rrca(cpu: &mut CPU) {
    let carry = cpu.registers.a & 0x01;
    cpu.registers.a = (cpu.registers.a >> 1) | (cpu.registers.a << 7) & 0xFF;
    cpu.registers.flags.c = carry == 1;

    cpu.registers.flags.z = false;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = false;
}
pub fn rra(cpu: &mut CPU) {
    let carry_after_rotate = (cpu.registers.a & 0x01) != 0;
    
    cpu.registers.a = (cpu.registers.a >> 1) | (cpu.registers.a << 7) & 0xFF;
    cpu.registers.flags.c = carry_after_rotate;

    cpu.registers.flags.z = cpu.registers.a == 0;
    cpu.registers.flags.n = false;
    cpu.registers.flags.h = false;
}

pub fn daa(cpu: &mut CPU) {
    let mut offset: u8 = 0;

    let a_val = &mut cpu.registers.a;
    let half_carry = cpu.registers.flags.h;
    let carry = cpu.registers.flags.c;
    let subtract = cpu.registers.flags.n;

    if (!subtract && (*a_val & 0xF > 0x09)) || half_carry {
        offset |= 0x06;
    }

    if (!subtract && (*a_val > 0x99)) || carry {
        offset |= 0x60;
        cpu.registers.flags.c = true;
    } else {
        cpu.registers.flags.c = false;
    }

    if !subtract {
        *a_val = a_val.wrapping_add(offset);
    }
    else {
        *a_val = a_val.wrapping_sub(offset);
    }
    cpu.registers.flags.z = *a_val == 0;
    cpu.registers.flags.h = false;
}

pub fn cpl(cpu: &mut CPU) {
    cpu.registers.a = !cpu.registers.a;
}
