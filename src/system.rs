use crate::bus::Bus;
use crate::cartridge::mbc1::Mbc1;
use crate::cpu::Cpu;

pub enum Gas {
    UNLIMITED,
    LIMITED(usize),
}

pub struct System {
    bus: Bus,
    cpu: Cpu,
}

impl System {
    pub fn load_cartridge(cartridge: Mbc1) -> Self {
        System {
            bus: Bus::new(cartridge),
            cpu: Cpu::default(),
        }
    }

    pub fn run(&mut self) {
        self.run_with_gas(Gas::UNLIMITED)
    }

    pub fn run_with_gas(&mut self, mut gas: Gas) {
        loop {
            if let Gas::LIMITED(remaining_gas) = gas {
                if remaining_gas <= 0 {
                    return;
                }

                gas = Gas::LIMITED(remaining_gas - 1);
            }

            self.cpu.read_decode_execute(&mut self.bus);
        }
    }

    pub fn bus(&self) -> &Bus {
        &self.bus
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }
}
