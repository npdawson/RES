use super::Cpu;
use super::Ppu;

pub struct Nes {
    cpu: Cpu,
    ppu: Ppu
}

impl Nes {
    pub fn new() -> Nes {
        Nes {
            cpu: Cpu::new(),
            ppu: Ppu::new()
        }
    }
}
