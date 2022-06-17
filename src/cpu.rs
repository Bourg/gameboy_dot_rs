use crate::bus::Bus;
use crate::memory::MemoryMapped;

// TODO is this correct?
const INITIAL_PC: u16 = 0x150;

// TODO probably some great opportunity to write opcodes as macros

pub struct Cpu {
    // General purpose registers
    // TODO what are the default values of these
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    // Program counter
    pc: u16,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            pc: INITIAL_PC,
        }
    }

    /// Execute the instruction at the current pc
    /// This includes reading the opcode and any necessary additional info from program memory.
    /// Returns the number of machine cycles the instruction takes to execute
    fn fetch_decode_execute(&mut self) -> u8 {
        // LD B, r'
        match opcode {
            0x40 => {
                2 /* TODO is a no-op b->b load still 2 cycles? */
            },
            0x41 => {
                self.c = self.b;
                2
            },
            0x
            _ => panic!("Unimplemented"),
        }
    }

    fn fetch_byte(&mut self, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }
}
