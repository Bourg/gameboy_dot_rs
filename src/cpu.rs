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

        match instruction {
            // 00xxx110
            0x06 => {
                self.b = self.read_byte_advance_pc(bus);
                2
            }
            0x0E => {
                self.c = self.read_byte_advance_pc(bus);
                2
            }
            0x16 => {
                self.d = self.read_byte_advance_pc(bus);
                2
            }
            0x1E => {
                self.e = self.read_byte_advance_pc(bus);
                2
            }
            0x26 => {
                self.h = self.read_byte_advance_pc(bus);
                2
            }
            0x2E => {
                self.l = self.read_byte_advance_pc(bus);
                2
            }
            0x3E => {
                self.a = self.read_byte_advance_pc(bus);
                2
            }

            0x40 => {
                self.b = self.b;
                1
            }
            0x41 => {
                self.b = self.c;
                1
            }
            0x42 => {
                self.b = self.d;
                1
            }
            0x43 => {
                self.b = self.e;
                1
            }
            0x44 => {
                self.b = self.h;
                1
            }
            0x45 => {
                self.b = self.l;
                1
            }
            0x47 => {
                self.b = self.a;
                1
            }
            0x48 => {
                self.c = self.b;
                1
            }
            0x49 => {
                self.c = self.c;
                1
            }
            0x4A => {
                self.c = self.d;
                1
            }
            0x4B => {
                self.c = self.e;
                1
            }
            0x4C => {
                self.c = self.h;
                1
            }
            0x4D => {
                self.c = self.l;
                1
            }
            0x4F => {
                self.c = self.a;
                1
            }
            0x50 => {
                self.d = self.b;
                1
            }
            0x51 => {
                self.d = self.c;
                1
            }
            0x52 => {
                self.d = self.d;
                1
            }
            0x53 => {
                self.d = self.e;
                1
            }
            0x54 => {
                self.d = self.h;
                1
            }
            0x55 => {
                self.d = self.l;
                1
            }
            0x57 => {
                self.d = self.a;
                1
            }
            0x58 => {
                self.e = self.b;
                1
            }
            0x59 => {
                self.e = self.c;
                1
            }
            0x5A => {
                self.e = self.d;
                1
            }
            0x5B => {
                self.e = self.e;
                1
            }
            0x5C => {
                self.e = self.h;
                1
            }
            0x5D => {
                self.e = self.l;
                1
            }
            0x5F => {
                self.e = self.a;
                1
            }
            0x60 => {
                self.h = self.b;
                1
            }
            0x61 => {
                self.h = self.c;
                1
            }
            0x62 => {
                self.h = self.d;
                1
            }
            0x63 => {
                self.h = self.e;
                1
            }
            0x64 => {
                self.h = self.h;
                1
            }
            0x65 => {
                self.h = self.l;
                1
            }
            0x67 => {
                self.h = self.a;
                1
            }
            0x68 => {
                self.l = self.b;
                1
            }
            0x69 => {
                self.l = self.c;
                1
            }
            0x6A => {
                self.l = self.d;
                1
            }
            0x6B => {
                self.l = self.e;
                1
            }
            0x6C => {
                self.l = self.h;
                1
            }
            0x6D => {
                self.l = self.l;
                1
            }
            0x6F => {
                self.l = self.a;
                1
            }
            0x78 => {
                self.a = self.b;
                1
            }
            0x79 => {
                self.a = self.c;
                1
            }
            0x7A => {
                self.a = self.d;
                1
            }
            0x7B => {
                self.a = self.e;
                1
            }
            0x7C => {
                self.a = self.h;
                1
            }
            0x7D => {
                self.a = self.l;
                1
            }
            0x7F => {
                self.a = self.a;
                1
            }
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
