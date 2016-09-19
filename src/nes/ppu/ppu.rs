use nes::Cart;

pub struct Ppu {
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {}
    }

    pub fn read_reg(&self, addr: usize) -> u8 {
        0 // TODO
    }
}
