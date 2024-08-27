use crate::display;
use crate::registers;
type Byte = u8;
// both ram areas are 8KiB in size so using the same type alias makes sense
type RAMArea = [Byte; 8192];
type Tile = [u8; 2];

const TILE_DATA_START: u16 = 0x8000;
const TILE_DATA_END: u16 = 0x97FF;

pub struct CPU {
   pub work_ram: RAMArea,
   pub video_ram: RAMArea,
   pub display: display::Display,
   pub stack_ptr: usize,
   pub registers: registers::Registers,
}
