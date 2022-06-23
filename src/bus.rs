use crate::cartridge::mbc1::Mbc1;
use crate::memory::MemoryMapped;
use crate::ram::Ram;

const CARTRIDGE_ADDRESS_START: u16 = 0x0000;
const CARTRIDGE_ADDRESS_END: u16 = 0x7FFF;
const RAM_ADDRESS_START: u16 = 0xC000;
const RAM_ADDRESS_END: u16 = 0xDFFF;

pub struct Bus {
    pub cartridge: Mbc1,
    pub ram: Ram<0x2000>,
}

impl Bus {
    pub fn new(cartridge: Mbc1) -> Self {
        Bus {
            cartridge,
            ram: Ram::default(),
        }
    }
}

impl MemoryMapped for Bus {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            CARTRIDGE_ADDRESS_START..=CARTRIDGE_ADDRESS_END => self.cartridge.read_byte(address),
            RAM_ADDRESS_START..=RAM_ADDRESS_END => self.ram.read_byte(address - RAM_ADDRESS_START),
            _ => todo!("Memory address {:#06X} not mapped for bus reads", address),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            CARTRIDGE_ADDRESS_START..=CARTRIDGE_ADDRESS_END => {
                self.cartridge.write_byte(address, value)
            }
            RAM_ADDRESS_START..=RAM_ADDRESS_END => {
                self.ram.write_byte(address - RAM_ADDRESS_START, value)
            }
            _ => todo!("Memory address {:#06X} not mapped for bus writes", address),
        }
    }
}
