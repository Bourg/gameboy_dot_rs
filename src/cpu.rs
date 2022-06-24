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
