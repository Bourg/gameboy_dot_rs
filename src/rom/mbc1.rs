use crate::memory::MemoryMapped;

const RAM_GATE_REGISTER_ADDRESS_START: u16 = 0x0000;
const RAM_GATE_REGISTER_ADDRESS_END: u16 = 0x1FFF;
const BANK_1_REGISTER_ADDRESS_START: u16 = 0x2000;
const BANK_1_REGISTER_ADDRESS_END: u16 = 0x3FFF;
const BANK_2_REGISTER_ADDRESS_START: u16 = 0x4000;
const BANK_2_REGISTER_ADDRESS_END: u16 = 0x5FFF;
const MODE_REGISTER_ADDRESS_START: u16 = 0x6000;
const MODE_REGISTER_ADDRESS_END: u16 = 0x7FFF;

const LOW_ROM_BANK_ADDRESS_START: u16 = 0x0000;
const LOW_ROM_BANK_ADDRESS_END: u16 = 0x3FFF;
const HIGH_ROM_BANK_ADDRESS_START: u16 = 0x4000;
const HIGH_ROM_BANK_ADDRESS_END: u16 = 0x7FFF;

// Note: As implemented, this only supports the common memory bank controller MBC1
// It does not support MBC1M (aka "multicart"), MBC2, MBC3, MBC30, MBC5, MBC6, MBC7, etc...
pub struct Mbc1 {
    ram_gate_register: bool,
    bank_register_1: u8,
    bank_register_2: u8,
    mode_register: bool,
}

impl Mbc1 {
    // TODO actually loading a rom should parse the header
    fn new() -> Mbc1 {
        Mbc1 {
            ram_gate_register: false,
            bank_register_1: 1,
            bank_register_2: 0,
            mode_register: false,
        }
    }

    // TODO how should this behave when the bank number would be greater than the number of banks on the chip?
    // TODO this does not account for "multicart" cartridges
    fn active_rom_bank_number(&self, address: u16) -> u8 {
        match address {
            LOW_ROM_BANK_ADDRESS_START..=LOW_ROM_BANK_ADDRESS_END => {
                if !self.mode_register {
                    0
                } else {
                    // Bank register 2 is 2-bit, so shifting 5 left will never overflow
                    self.bank_register_2 << 5
                }
            }
            HIGH_ROM_BANK_ADDRESS_START..=HIGH_ROM_BANK_ADDRESS_END => {
                (self.bank_register_2 << 5) + self.bank_register_1
            }
            _ => {
                panic!(
                    "ROM is only readable in the range {:#06X}..={:#06X}, but a read was attempted at {:#06X}",
                    LOW_ROM_BANK_ADDRESS_START,
                    HIGH_ROM_BANK_ADDRESS_END,
                    address);
            }
        }
    }
}

impl MemoryMapped for Mbc1 {
    fn read_byte(&self, _address: u16) -> u8 {
        // TODO NEXT - READING ROM
        // ROM header first
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
        let mut rom = Mbc1::new();

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
        let mut rom = Mbc1::new();

        assert_eq!(1, rom.bank_register_1);

        // Writing within the lower 5 bits is written as-is
        rom.write_byte(0x3CCC, 0b0011);
        assert_eq!(0b0011, rom.bank_register_1);
        rom.write_byte(0x2000, 0b11100);
        assert_eq!(0b11100, rom.bank_register_1);

        // Writing out of the lower 5 bits only keeps the lowest 5 bits
        rom.write_byte(0x3210, 0b11110010);
        assert_eq!(0b10010, rom.bank_register_1);

        // Writing 0 actually writes 1
        rom.write_byte(0x3FED, 0);
        assert_eq!(1, rom.bank_register_1);
    }

    #[test]
    fn test_bank_2() {
        let mut rom = Mbc1::new();

        assert_eq!(0, rom.bank_register_2);

        // Any 3-bit writes
        for value in 0b11..=0 {
            rom.write_byte(0x4000, value);
            assert_eq!(value, rom.bank_register_2);
        }

        rom.write_byte(0x5FFF, 0b11111010);
        assert_eq!(0b10, rom.bank_register_2);
    }

    #[test]
    fn test_mode() {
        let mut rom = Mbc1::new();

        assert_eq!(false, rom.mode_register);

        // Writing a 1 sets the flag
        rom.write_byte(0x7FFF, 0b1);
        assert_eq!(true, rom.mode_register);

        // Writing a 3 unsets the flag beause the lowest bit is zero
        rom.write_byte(0x6000, 0b10);
        assert_eq!(false, rom.mode_register);

        // All but lowest bit zeroed, still unset
        rom.write_byte(0x6789, 0xFE);
        assert_eq!(false, rom.mode_register);

        // All bits set, flag set
        rom.write_byte(0x6789, 0xFF);
        assert_eq!(true, rom.mode_register);
    }

    #[test]
    fn test_active_rom_bank_number() {
        let mut rom = Mbc1::new();

        rom.write_byte(BANK_1_REGISTER_ADDRESS_START, 0b10010);
        rom.write_byte(BANK_2_REGISTER_ADDRESS_START, 0b01);

        // Active banks when the mode is 0
        assert_eq!(0, rom.active_rom_bank_number(LOW_ROM_BANK_ADDRESS_START));
        assert_eq!(
            0b0110010,
            rom.active_rom_bank_number(HIGH_ROM_BANK_ADDRESS_END)
        );

        // Active banks when the mode is 1
        rom.write_byte(MODE_REGISTER_ADDRESS_START, 0x1);
        assert_eq!(
            0b0100000,
            rom.active_rom_bank_number(LOW_ROM_BANK_ADDRESS_END)
        );
        assert_eq!(
            0b0110010,
            rom.active_rom_bank_number(HIGH_ROM_BANK_ADDRESS_END)
        );

        // Try some more values
        rom.write_byte(BANK_1_REGISTER_ADDRESS_START, 0b00100);
        rom.write_byte(BANK_2_REGISTER_ADDRESS_START, 0b10);

        // Active banks when mode is 1 (left over from previous part of the test, not reset)
        assert_eq!(0b1000000, rom.active_rom_bank_number(0x369C));
        assert_eq!(68, rom.active_rom_bank_number(0x72A7));

        // Active banks when mode is 0
        rom.write_byte(MODE_REGISTER_ADDRESS_START, 0x0);
        assert_eq!(0, rom.active_rom_bank_number(0x369C));
        assert_eq!(68, rom.active_rom_bank_number(0x72A7));

        // Test that it is actually impossible to get bank 0x20 in the 0x4000..=0x7999 address range
        // Would need bank2 == 0b01, bank1 == 0b00000, but bank1 can never be zeroed
        rom.write_byte(BANK_2_REGISTER_ADDRESS_START, 0b01);
        rom.write_byte(BANK_1_REGISTER_ADDRESS_START, 0b00000);
        // Even though we wrote in the bank number 0b0100000, we get 0b0100001 back
        // This is because bank1 cannot be 0
        assert_eq!(
            0x21,
            rom.active_rom_bank_number(HIGH_ROM_BANK_ADDRESS_START)
        );

        // However, you can get to bank 0x20 by addressing 0x0000..=0x3FFF in mode 1
        rom.write_byte(MODE_REGISTER_ADDRESS_START, 0b1);
        rom.write_byte(BANK_2_REGISTER_ADDRESS_START, 0b01);
        assert_eq!(0x20, rom.active_rom_bank_number(0x1234));
    }
}
