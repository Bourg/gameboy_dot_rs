use crate::memory::MemoryMapped;

const RAM_GATE_ADDRESS_START: u16 = 0x0000;
const RAM_GATE_ADDRESS_END: u16 = 0x1FFF;

// Note: As implemented, this only supports the common memory bank controller MBC1
pub struct Rom {
    // RAMG - Is access to SRAM allowed
    ram_gate_register: bool,
}

impl Rom {
    // TODO actually loading a rom should parse the header
    fn new() -> Rom {
        Rom {
            ram_gate_register: false,
        }
    }
}

impl MemoryMapped for Rom {
    fn read_byte(&self, _address: u16) -> u8 {
        todo!("Unimplemented")
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            RAM_GATE_ADDRESS_START..=RAM_GATE_ADDRESS_END => {
                self.ram_gate_register = value & 0xF == 0b1010;
            },
            _ => todo!("Unimplemented")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ramg() {
        let mut rom = Rom::new();

        // Register should start off
        assert_eq!(false, rom.ram_gate_register);

        // Writing arbitrary values to the range shouldn't change the flag
        rom.write_byte(0x1FFF, 0b11111111);
        assert_eq!(false, rom.ram_gate_register);
        rom.write_byte(0x1FFF, 0b10100101);
        assert_eq!(false, rom.ram_gate_register);

        // Writing, specifically, 0b1010 in the lower nibble and any higher nibble sets the flag
        rom.write_byte(0x1234, 0b11011010);
        assert_eq!(true, rom.ram_gate_register);

        // Writing something else will unset it again
        rom.write_byte(0x0000, 0b00000101);
        assert_eq!(false, rom.ram_gate_register);
    }
}