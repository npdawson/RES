use super::Cpu;
use super::Ppu;
use super::Cart;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Nes {
    cpu: Cpu,
    ppu: Ppu,
    cart: Cart
}

impl Nes {
    pub fn new(rom_filename: String) -> Nes {
        let rom_file = File::open(rom_filename).unwrap();
        let cart = Cart::new(&mut rom_file);
        Nes {
            cpu: Cpu::new(cart),
            ppu: Ppu::new(cart),
            cart: cart,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        // self.ppu.reset();
    }

    pub fn step(&mut self) {
        self.cpu.step();
        // self.ppu.step(); // ppu will step 3 cycles at a time
    }
}
