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

        macro_rules! ld_r_immediate {
            ($a: ident) => {{
                self.$a = self.read_byte_advance_pc(bus);
                2
            }};
        }

        macro_rules! ld_r_r {
            ($a: ident, $b: ident) => {{
                self.$a = self.$b;
                1
            }};
        }

        match instruction {
            0x06 => ld_r_immediate!(b),
            0x0E => ld_r_immediate!(c),
            0x16 => ld_r_immediate!(d),
            0x1E => ld_r_immediate!(e),
            0x26 => ld_r_immediate!(h),
            0x2E => ld_r_immediate!(l),
            0x3E => ld_r_immediate!(a),
            0x40 => ld_r_r!(b, b),
            0x41 => ld_r_r!(b, c),
            0x42 => ld_r_r!(b, d),
            0x43 => ld_r_r!(b, e),
            0x44 => ld_r_r!(b, h),
            0x45 => ld_r_r!(b, l),
            0x47 => ld_r_r!(b, a),
            0x48 => ld_r_r!(c, b),
            0x49 => ld_r_r!(c, c),
            0x4A => ld_r_r!(c, d),
            0x4B => ld_r_r!(c, e),
            0x4C => ld_r_r!(c, h),
            0x4D => ld_r_r!(c, l),
            0x4F => ld_r_r!(c, a),
            0x50 => ld_r_r!(d, b),
            0x51 => ld_r_r!(d, c),
            0x52 => ld_r_r!(d, d),
            0x53 => ld_r_r!(d, e),
            0x54 => ld_r_r!(d, h),
            0x55 => ld_r_r!(d, l),
            0x57 => ld_r_r!(d, a),
            0x58 => ld_r_r!(e, b),
            0x59 => ld_r_r!(e, c),
            0x5A => ld_r_r!(e, d),
            0x5B => ld_r_r!(e, e),
            0x5C => ld_r_r!(e, h),
            0x5D => ld_r_r!(e, l),
            0x5F => ld_r_r!(e, a),
            0x60 => ld_r_r!(h, b),
            0x61 => ld_r_r!(h, c),
            0x62 => ld_r_r!(h, d),
            0x63 => ld_r_r!(h, e),
            0x64 => ld_r_r!(h, h),
            0x65 => ld_r_r!(h, l),
            0x67 => ld_r_r!(h, a),
            0x68 => ld_r_r!(l, b),
            0x69 => ld_r_r!(l, c),
            0x6A => ld_r_r!(l, d),
            0x6B => ld_r_r!(l, e),
            0x6C => ld_r_r!(l, h),
            0x6D => ld_r_r!(l, l),
            0x6F => ld_r_r!(l, a),
            0x78 => ld_r_r!(a, b),
            0x79 => ld_r_r!(a, c),
            0x7A => ld_r_r!(a, d),
            0x7B => ld_r_r!(a, e),
            0x7C => ld_r_r!(a, h),
            0x7D => ld_r_r!(a, l),
            0x7F => ld_r_r!(a, a),
            0xC3 => {
                self.pc = self.read_word_advance_pc(bus);
                4
            }
            0xEA => {
                bus.write_byte(self.read_word_advance_pc(bus), self.a);
                4
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
