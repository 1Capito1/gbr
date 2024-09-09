use crate::display;
use crate::registers;
type Byte = u8;
// both ram areas are 8KiB in size so using the same type alias makes sense
type RAMArea = [Byte; 8192];
type Tile = [u8; 2];

const TILE_DATA_START: u16 = 0x8000;
const TILE_DATA_END: u16 = 0x97FF;

/// Represents the state of the CPU
pub enum CpuState {
    STOP,
    HALT,
    CONTINUE,
}

pub struct CPU {
    pub work_ram: RAMArea,
    pub video_ram: RAMArea,
    pub display: display::Display,
    pub stack_ptr: usize,
    pub program_counter: usize,
    pub registers: registers::Registers,
    pub state: CpuState,
}

impl CPU {
    /// moves the PC 2 bytes, returning a u16 of the two passed bytes
    pub fn get_next_two_bytes(&mut self) -> u16 {
        self.program_counter += 1;
        let lo = self.work_ram[self.program_counter];
        self.program_counter += 1;
        let hi = self.work_ram[self.program_counter];
        return u16::from_le_bytes([hi, lo]);
    }

    /// moves the PC 1 byte, returning a u8 of the passed byte
    pub fn get_next_one_byte(&mut self) -> u8 {
        self.program_counter += 1;
        return self.work_ram[self.program_counter];
    }
}
