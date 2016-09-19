const RAM_SIZE: usize = 2048;

use super::instruction::Instruction;
use super::opcode::Opcode::*;
use super::opcode::AddrMode::*;

pub struct Cpu {
    // Registers
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    reg_pc: u16,
    reg_s: u8,
    reg_p: u8,
    rom: Box<[u8]>,
    // RAM
    ram: Box<[u8]>,
    // PPU Regs
    ppu_reg: Box<[u8]>,
}

impl Cpu {
    pub fn new(romfile: Box<[u8]>) -> Cpu {
        Cpu {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            reg_pc: 0, // TODO
            reg_s: 0xFD,
            reg_p: 0x34, // IRQ disabled
            rom: romfile,
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
            ppu_reg: vec![0; 8].into_boxed_slice(),
        }
    }

    pub fn reset(&mut self) {
        // TODO set proper values
        // self.reg_s -= 3;
        self.reg_pc = self.read_word(0xFFFC);
        // I flag (IRQ disable) set to true (OR with 0x04)
        // APU silenced (0x4015 = 0)
    }

    pub fn step(&mut self) {
        println!("{:#X}", self.reg_pc);
        let byte = self.read_byte(self.reg_pc as usize);
        let opcode = Instruction::new(byte);
        self.reg_pc += 1;
        self.run_instruction(opcode);
    }

    fn run_instruction(&mut self, op: Instruction) {
        match op.opcode {
            And => {
                match op.addrmode {
                    Imm1 => {
                        let imm = self.read_byte(self.reg_pc as usize);
                        self.reg_pc += 1;
                        let value = self.reg_a & imm;
                        self.reg_a = value;
                        let neg = value >> 7 != 0;
                        let zero = value == 0;
                        if neg { self.reg_p |= 1 << 7} else { self.reg_p &= !(1 << 7) };
                        if zero { self.reg_p |= 1 << 1 } else { self.reg_p &= !(1 << 1)};
                    },
                    _ => panic!("Address mode {:?} not implemented for {:?}",
                                op.addrmode, op.opcode)
                }
            },
            Beq => {
                if self.reg_p & (1 << 1) != 0 {
                    let offset = self.read_byte(self.reg_pc as usize) as i8;
                    self.reg_pc = self.reg_pc
                        .wrapping_add((offset as i16) as u16);
                } else {
                    self.reg_pc += 1;
                }
            },
            Cld => self.reg_p &= !(1 << 3),
            Lda => {
                match op.addrmode {
                    Abs2 => {
                        let addr = self.read_word(self.reg_pc as usize);
                        self.reg_pc += 2;
                        let value = self.read_byte(addr as usize);
                        let neg = value >> 7 != 0;
                        let zero = value == 0;
                        if neg { self.reg_p |= 1 << 7} else { self.reg_p &= !(1 << 7) };
                        if zero { self.reg_p |= 1 << 1 } else { self.reg_p &= !(1 << 1)};
                    },
                    Imm1 => {
                        let imm = self.read_byte(self.reg_pc as usize);
                        self.reg_pc += 1;
                        self.reg_a = imm;
                        let neg = imm >> 7 != 0;
                        let zero = imm == 0;
                        if neg { self.reg_p |= 1 << 7} else { self.reg_p &= !(1 << 7) };
                        if zero { self.reg_p |= 1 << 1 } else { self.reg_p &= !(1 << 1)};
                    },
                    _ => panic!("Address mode {:?} not implemented for {:?}",
                                op.addrmode, op.opcode)
                }
            },
            Ldx => {
                match op.addrmode {
                    Imm2 => {
                        let imm = self.read_byte(self.reg_pc as usize);
                        self.reg_pc += 1;
                        self.reg_x = imm;
let neg = imm >> 7 != 0;
                        let zero = imm == 0;
                        if neg { self.reg_p |= 1 << 7} else { self.reg_p &= !(1 << 7) };
                        if zero { self.reg_p |= 1 << 1 } else { self.reg_p &= !(1 << 1)};
                    },
                    _ => panic!("Address mode {:?} not implemented for {:?}",
                                op.addrmode, op.opcode),
                }
            },
            Sei => self.reg_p |= 1 << 2,
            Sta => {
                match op.addrmode {
                    Abs2 => {
                        let addr = self.read_word(self.reg_pc as usize);
                        self.reg_pc += 2;
                        let a = self.reg_a;
                        self.write_byte(addr as usize, a);
                    },
                    _ => panic!("Addr mode {:?} not impled for {:?}",
                                op.addrmode, op.opcode)
                }
            },
            Txs => self.reg_s = self.reg_x,
            _ => panic!("Operation not yet implemented: {:?}", op.opcode)
        }
    }

    fn read_ppu(&self, addr: usize) -> u8 {
        match addr {
            0x2000 ... 0x3FFF =>
                self.ppu_reg[addr & 0x7],
            _ => unreachable!()
        }
    }

    fn write_ppu(&mut self, addr: usize, value: u8) {
        match addr {
            0x2000 ... 0x3FFF =>
                self.ppu_reg[addr & 0x7] = value,
            _ => unreachable!()
        }
    }

    fn read_byte(&self, addr: usize) -> u8 {
        match addr {
            0x0000 ... 0x07FF => self.ram[addr],
            0x0800 ... 0x0FFF => self.ram[addr - 0x0800],
            0x1000 ... 0x17FF => self.ram[addr - 0x1000],
            0x1800 ... 0x1FFF => self.ram[addr - 0x1800],
            0x2000 ... 0x3FFF => self.read_ppu(addr),
            0x4000 ... 0x4017 => panic!("APU and IO Regs not implemented!"),
            0x4018 ... 0x401F => panic!("See CPU Test Mode!"),
            0x8000 ... 0xBFFF => self.rom[addr - 0x8000 + 0x10],
            0xC000 ... 0xFFFF => self.rom[addr - 0xC000 + 0x10],
            _ => unreachable!()
        }
    }

    fn write_byte(&mut self, addr: usize, value: u8) {
        match addr {
            0x0000 ... 0x07FF => self.ram[addr] = value,
            0x0800 ... 0x0FFF => self.ram[addr - 0x0800] = value,
            0x1000 ... 0x17FF => self.ram[addr - 0x1000] = value,
            0x1800 ... 0x1FFF => self.ram[addr - 0x1800] = value,
            0x2000 ... 0x3FFF => self.write_ppu(addr, value),
            // 0x2008 ... 0x3FFF => // mirrors of PPU regs, repeats every 8B
            0x4000 ... 0x4017 => panic!("APU and IO Regs not implemented!"),
            0x4018 ... 0x401F => panic!("See CPU Test Mode!"),
            0x4020 ... 0x7FFF => panic!("Cart RAM not implemented!"),
            0x8000 ... 0xFFFF => panic!("Can't write to ROM!"),
            _ => unreachable!()
        }
    }

    fn read_word(&self, addr: usize) -> u16 {
        (self.read_byte(addr + 1) as u16) << 8 | self.read_byte(addr) as u16
    }
}
