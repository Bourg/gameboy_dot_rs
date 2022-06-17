use crate::memory::MemoryMapped;

const RAM_GATE_REGISTER_ADDRESS_START: u16 = 0x0000;
const RAM_GATE_REGISTER_ADDRESS_END: u16 = 0x1FFF;
const BANK_1_REGISTER_ADDRESS_START: u16 = 0x2000;
const BANK_1_REGISTER_ADDRESS_END: u16 = 0x3FFF;
const BANK_2_REGISTER_ADDRESS_START: u16 = 0x4000;
const BANK_2_REGISTER_ADDRESS_END: u16 = 0x5FFF;
const MODE_REGISTER_ADDRESS_START: u16 = 0x6000;
const MODE_REGISTER_ADDRESS_END: u16 = 0x7FFF;

// Note: As implemented, this only supports the common memory bank controller MBC1
pub struct Rom {
    ram_gate_register: bool,
    bank_register_1: u8,
    bank_register_2: u8,
    mode_register: bool,
}

impl Rom {
    // TODO actually loading a rom should parse the header
    fn new() -> Rom {
        Rom {
            ram_gate_register: false,
            bank_register_1: 1,
            bank_register_2: 0,
            mode_register: false,
        }
    }
}

impl MemoryMapped for Rom {
    fn read_byte(&self, _address: u16) -> u8 {
        todo!("Unimplemented")
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            RAM_GATE_REGISTER_ADDRESS_START..=RAM_GATE_REGISTER_ADDRESS_END => {
                self.ram_gate_register = value & 0xF == 0b1010;
            }
            BANK_1_REGISTER_ADDRESS_START..=BANK_1_REGISTER_ADDRESS_END => {
                let mut value = value & 0x1F;

                // Zero-bit adjustment - 0 is not a valid value and is coerced to 1
                if value == 0 {
                    value = 1;
                }

                self.bank_register_1 = value;
            }
            BANK_2_REGISTER_ADDRESS_START..=BANK_2_REGISTER_ADDRESS_END => {
                self.bank_register_2 = value & 0x3;
            }
            MODE_REGISTER_ADDRESS_START..=MODE_REGISTER_ADDRESS_END => {
                self.mode_register = value & 0x1 == 1;
            }
            _ => {
                panic!(
                    "ROM is only addressable in range 0x0000..=0x7FFF, but was written at {:#06X}",
                    address
                )
            }
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

    #[test]
    fn test_bank_1() {
        /*
               MBC1 doesn’t allow the BANK1 register to contain zero (bit pattern 0b00000), so the initial value at reset
        is 0b00001 and attempting to write 0b00000 will write 0b00001 instead. This makes it impossible to read
        banks 0x00, 0x20, 0x40 and 0x60 from the 0x4000-0x7FFF memory area, because those bank numbers have
        0b00000 in the lower bits. Due to the zero value adjustment, requesting any of these banks actually requests
        the next bank (e.g. 0x21 instead of 0x20)
                */
    }
}
