use crate::memory::MemoryMapped;
use crate::ram::Ram;

// TODO learn how to write a macro to generate <start, end, size> constants
const INTERRUPT_START: u16 = 0x0000;
const ROM_HEADER_START: u16 = 0x0100;
const PROGRAM_START: u16 = 0x150;
const DISPLAY_START: u16 = 0x8000;
const EXPANSION_START: u16 = 0xA000;
const WORK_RAM_START: u16 = 0xC000;
const WORK_PROHIBITED_START: u16 = 0xE000;
const OAM_START: u16 = 0xFE00;
const CPU_PROHIBITED_START: u16 = 0xFEA0;
const CPU_RAM_START: u16 = 0xFF00;
const STACK_RAM_START: u16 = 0xFF80;
const END: u16 = 0x10000;

pub struct Bus {
    work_ram: Ram<0x2000>,
}

impl Bus {
    fn new() -> Bus {
        Bus {
            work_ram: Ram::new(),
        }
    }
}

impl MemoryMapped for Bus {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..DISPLAY_START =>
            _ => todo!("Unimplemented")
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        todo!("Unimplemented")
    }
}
