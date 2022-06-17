use crate::memory::MemoryMapped;

pub struct Ram<const N: usize> {
    ram: [u8; N],
}

impl<const N: usize> Ram<N> {
    pub fn new() -> Ram<N> {
        Ram { ram: [0; N] }
    }
}

impl<const N: usize> MemoryMapped for Ram<N> {
    fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value
    }
}
