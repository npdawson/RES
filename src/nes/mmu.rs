use nes::Cart;
use nes::Ppu;

use byteorder::{LittleEndian, ByteOrder};

pub struct Mmu<'a> {
    cart: &'a mut Cart,
    wram: Box<[u8]>,
    vram: Box<[u8]>,
    ppu: &'a mut Ppu
}

impl<'a> Mmu<'a> {
    pub fn read_byte(&self, addr: usize) -> u8 {
        match addr {
            0x0000 ... 0x07FF => self.wram[addr],
            0x0800 ... 0x0FFF => self.wram[addr - 0x0800],
            0x1000 ... 0x17FF => self.wram[addr - 0x1000],
            0x1800 ... 0x1FFF => self.wram[addr - 0x1800],
            0x2000 ... 0x3FFF => self.ppu.read_reg(addr),
            0x4014            => panic!("Can't read from OAM DMA!"),
            0x4000 ... 0x4017 =>
                panic!("APU and IO Regs not yet implemented!"), // TODO APU and IO Regs
            0x4018 ... 0x401F =>
                panic!("CPU Test Mode not yet implemented!"), // TODO CPU Test Mode
            0x4020 ... 0xFFFF => self.cart.prg_rb(addr),
        }
    }

    pub fn read_word(&self, addr: usize) -> u16 {
        match addr {
            0x0000 ... 0x07FF =>
                LittleEndian::read_u16(&self.wram[addr..]),
            0x0800 ... 0x0FFF =>
                LittleEndian::read_u16(&self.wram[addr - 0x0800..]),
            0x1000 ... 0x17FF =>
                LittleEndian::read_u16(&self.wram[addr - 0x1000..]),
            0x1800 ... 0x1FFF =>
                LittleEndian::read_u16(&self.wram[addr - 0x1800..]),
            0x2000 ... 0x3FFF => self.ppu.read_reg(addr) as u16, // TODO error?
            0x4014            => panic!("Can't read from OAM DMA!"),
            0x4000 ... 0x4017 =>
                panic!("APU and IO Regs not yet implemented!"), // TODO APU and IO Regs
            0x4018 ... 0x401F =>
                panic!("CPU Test Mode not yet implemented!"), // TODO CPU Test Mode
            0x4020 ... 0xFFFF => self.cart.prg_rw(addr),
        }
    }
}
