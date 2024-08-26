pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    flags: Flags
}

/// Lower half of the AF Register
pub struct Flags {
    /// Bit 7: Zero flag: This bit is set only if the
    /// result of an operation is zero.
    /// Used by conditional jumps
    z: u8,
    
    /// Bit 6: Subtraction Flag(BCD): Used by DAA instruction only
    /// indicates whether the previous instruction had been a subtraction
    n: u8,

    /// Bit 5: Half Carry flag(BCD): Used by DAA instruction only
    /// indicates carry for the lower 4 bits of the result
    h: u8,

    /// Bit 4: Carry flag:
    /// Flag is set when:
    ///     The result of 8-bit addition is higher than 0xFF
    ///     The result of 16-bit addition is higher than 0xFFFF
    ///     The result of a subtration or comparison is less than 0
    ///     When a shift/rotate operation shifts out a 1 bit
    /// 
    /// Used by conditional jumps and instructions such as ADC, SBC, RL, RLA, etc.
    c: u8
}

impl Registers {
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
            | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8
            | self.e as u16
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
            | self.l as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}
