use crate::bus::Bus;
use crate::memory::MemoryMapped;

const DEFAULT_PC: u16 = 0x100; // TODO support running a boot ROM

pub struct Cpu {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub pc: u16,
    // TODO sp
}

impl Cpu {
    // TODO need a deep dive on timing
    /// Performs one read->decode->execute cycle on the CPU
    /// Returns the number of machine cycles the instruction takes to execute
    pub fn read_decode_execute(&mut self, bus: &mut Bus) -> u8 {
        let instruction = self.read_byte_advance_pc(bus);

        // Macro for implementing loads that are repetitive across multiple registers
        macro_rules! ld {
            ($a: ident immediate value) => {{
                self.$a = self.read_byte_advance_pc(bus);
                2
            }};
            ($a: ident, [hl]) => {{
                self.$a = bus.read_byte(self.hl());
                2
            }};
            ([hl], $a: ident) => {{
                bus.write_byte(self.hl(), self.$a);
                2
            }};
            ($a: ident, $b: ident) => {{
                self.$a = self.$b;
                1
            }};
        }

        match instruction {
            0x02 => {
                bus.write_byte(self.bc(), self.a);
                2
            }
            0x06 => ld!(b immediate value),
            0x0A => {
                self.a = bus.read_byte(self.bc());
                2
            }
            0x0E => ld!(c immediate value),
            0x12 => {
                bus.write_byte(self.de(), self.a);
                2
            }
            0x16 => ld!(d immediate value),
            0x1A => {
                self.a = bus.read_byte(self.de());
                2
            }
            0x1E => ld!(e immediate value),
            0x26 => ld!(h immediate value),
            0x2E => ld!(l immediate value),
            0x36 => {
                bus.write_byte(self.hl(), self.read_byte_advance_pc(bus));
                3
            }
            0x3E => ld!(a immediate value),
            0x40 => ld!(b, b),
            0x41 => ld!(b, c),
            0x42 => ld!(b, d),
            0x43 => ld!(b, e),
            0x44 => ld!(b, h),
            0x45 => ld!(b, l),
            0x46 => ld!(b, [hl]),
            0x47 => ld!(b, a),
            0x48 => ld!(c, b),
            0x49 => ld!(c, c),
            0x4A => ld!(c, d),
            0x4B => ld!(c, e),
            0x4C => ld!(c, h),
            0x4D => ld!(c, l),
            0x4E => ld!(c, [hl]),
            0x4F => ld!(c, a),
            0x50 => ld!(d, b),
            0x51 => ld!(d, c),
            0x52 => ld!(d, d),
            0x53 => ld!(d, e),
            0x54 => ld!(d, h),
            0x55 => ld!(d, l),
            0x56 => ld!(d, [hl]),
            0x57 => ld!(d, a),
            0x58 => ld!(e, b),
            0x59 => ld!(e, c),
            0x5A => ld!(e, d),
            0x5B => ld!(e, e),
            0x5C => ld!(e, h),
            0x5D => ld!(e, l),
            0x5E => ld!(e, [hl]),
            0x5F => ld!(e, a),
            0x60 => ld!(h, b),
            0x61 => ld!(h, c),
            0x62 => ld!(h, d),
            0x63 => ld!(h, e),
            0x64 => ld!(h, h),
            0x65 => ld!(h, l),
            0x66 => ld!(h, [hl]),
            0x67 => ld!(h, a),
            0x68 => ld!(l, b),
            0x69 => ld!(l, c),
            0x6A => ld!(l, d),
            0x6B => ld!(l, e),
            0x6C => ld!(l, h),
            0x6D => ld!(l, l),
            0x6E => ld!(l, [hl]),
            0x6F => ld!(l, a),
            0x70 => ld!([hl], b),
            0x71 => ld!([hl], c),
            0x72 => ld!([hl], d),
            0x73 => ld!([hl], e),
            0x74 => ld!([hl], h),
            0x75 => ld!([hl], l),
            0x77 => ld!([hl], a),
            0x78 => ld!(a, b),
            0x79 => ld!(a, c),
            0x7A => ld!(a, d),
            0x7B => ld!(a, e),
            0x7C => ld!(a, h),
            0x7D => ld!(a, l),
            0x7E => ld!(a, [hl]),
            0x7F => ld!(a, a),
            0xC3 => {
                self.pc = self.read_word_advance_pc(bus);
                4
            }
            0xE2 => {
                bus.write_byte(self.c_as_high_ram_address(), self.a);
                2
            }
            0xEA => {
                bus.write_byte(self.read_word_advance_pc(bus), self.a);
                4
            }
            0xF2 => {
                self.a = bus.read_byte(self.c_as_high_ram_address());
                2
            }
            0xFA => {
                self.a = bus.read_byte(self.read_word_advance_pc(bus));
                4
            }
            _ => unimplemented!(
                "unimplemented opcode {:#04X} at address {:#04X}",
                instruction,
                self.pc - 1
            ),
        }
    }

    fn read_byte_advance_pc(&mut self, bus: &Bus) -> u8 {
        let byte = bus.read_byte(self.pc);
        self.pc += 1;
        byte
    }

    fn read_word_advance_pc(&mut self, bus: &Bus) -> u16 {
        let least_significant_byte = self.read_byte_advance_pc(bus);
        let most_significant_byte = self.read_byte_advance_pc(bus);

        u16::from_le_bytes([least_significant_byte, most_significant_byte])
    }

    fn bc(&self) -> u16 {
        Cpu::compound_register(self.b, self.c)
    }

    fn de(&self) -> u16 {
        Cpu::compound_register(self.d, self.e)
    }

    fn hl(&self) -> u16 {
        Cpu::compound_register(self.h, self.l)
    }

    fn get_and_increment_hl(&mut self) -> u16 {
        let hl = self.hl();

        let (next_l, l_overflow) = self.l.overflowing_add(1);
        if l_overflow {
            self.h = self.h.wrapping_add(1);
        }

        self.l = next_l;

        hl
    }

    fn compound_register(r1: u8, r2: u8) -> u16 {
        ((r1 as u16) << 8) + (r2 as u16)
    }

    fn c_as_high_ram_address(&self) -> u16 {
        0xFF00 + self.c as u16
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            pc: DEFAULT_PC,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_and_increment_hl() {
        let mut cpu = Cpu::default();

        // Simplest case
        assert_eq!(0x0000, cpu.get_and_increment_hl());
        assert_eq!(0x0001, cpu.hl());

        // Test rollover from l to h
        cpu.h = 0xCE;
        cpu.l = 0xFE;
        assert_eq!(0xCEFE, cpu.get_and_increment_hl());
        assert_eq!(0xCEFF, cpu.get_and_increment_hl());
        assert_eq!(0xCF00, cpu.get_and_increment_hl());
        assert_eq!(0xCF01, cpu.hl());

        // Test full rollover
        cpu.h = 0xFF;
        cpu.l = 0xFE;
        assert_eq!(0xFFFE, cpu.get_and_increment_hl());
        assert_eq!(0xFFFF, cpu.get_and_increment_hl());
        assert_eq!(0x0000, cpu.get_and_increment_hl());
        assert_eq!(0x0001, cpu.get_and_increment_hl());
    }
}
