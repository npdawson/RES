use std::io::Read;
use byteorder::{LittleEndian, ByteOrder};
use num::FromPrimitive;

pub struct Cart {
    header: Header,
    prg: Box<[u8]>,
    chr: Box<[u8]>,
    mmc: Mmc,
}

impl Cart {
    pub fn new(rom: &mut Read) -> Cart {
        let mut header = [0u8; 16];
        rom.read_exact(&mut header);

        let header = Header::new(header);

        let prg_bytes = header.prg_size() * 16384;
        let mut prg = vec![0u8; prg_bytes];
        rom.read_exact(&mut prg);

        let chr_bytes = header.chr_size() * 8192;
        let mut chr = vec![0u8; chr_bytes];
        rom.read_exact(&mut chr);

        let mapper = header.flags_7 & 0xF0 | header.flags_6 >> 4;

        Cart {
            header: header,
            prg: prg.into_boxed_slice(),
            chr: chr.into_boxed_slice(),
            mmc: Mmc::new(mapper, header),
        }
    }

    pub fn header_rb(&self, addr: usize) -> u8 {
        self.header.read_byte(addr)
    }

    pub fn prg_rb(&self, addr: usize) -> u8 {
        self.prg[addr]
    }

    pub fn prg_rw(&self, addr: usize) -> u16 {
        LittleEndian::read_u16(&self.prg[addr..])
    }

    pub fn chr_rb(&self, addr: usize) -> u8 {
        self.chr[addr]
    }
}

pub struct Header {
    // magic 'N' 'E' 'S' '\x1a'
    id: [u8; 4],
    // number of 16k banks of PRG-ROM
    prg_size: u8,
    // number of 8k banks of CHR-ROM
    // 0 means CHR-RAM
    chr_size: u8,
    // Flags 6
    // bits 3 and 0: arrangement/mirroring
    //       0xx0 => vertical   /horizontal (CIRAM A10 = PPU A11)
    //       0xx1 => horizontal /vertical   (CIRAM A10 = PPU A10)
    //       1xxx => four-screen VRAM
    // bit 1: battery-backed PRG-RAM (0x6000-7FFF) or other persistent memory
    // bit 2: 512-byte trainer at 0x7000-0x71FF (stored befor PRG data)
    // bits 7-4: lower nybble of mapper number
    flags_6: u8,
    // Flags 7
    // bit 0: VS Unisystem
    // bit 1: PlayChoice-10 (8KB of Hint Screen data stored after CHR data)
    // bit 3-2: if equal to 2, flags 8-15 are in NES 2.0 format
    // bits 7-4: upper nybble of mapper number
    flags_7: u8,
    // Flags 9
    // bit 0: 0 = NTSC, 1 = PAL
    // other bits reserved
    // virtually no ROM images make use of this bit
    flags_9: u8,
    // Flags 10
    // bits 1-0: TV System 0 = NTSC, 2 = PAL, 1/3 = dual compatible
    // bit 4: PRG RAM (0x6000-7FFF) (0: present, 1: not present)
    // bit 5: 0 = board has no bus conflicts, 1 = board has bus conflicts
    flags_10: u8,
}

impl Header {
    pub fn new(header: [u8; 16]) -> Header {
        Header {
            id: [
                header[0],
                header[1],
                header[2],
                header[3]
            ],
            prg_size: header[4],
            chr_size: header[5],
            flags_6: header[6],
            flags_7: header[7],
            flags_9: header[9],
            flags_10: header[10]
        }
    }

    pub fn read_byte(&self, addr: usize) -> u8 {
        match addr {
            0 => self.id[0],
            1 => self.id[1],
            2 => self.id[2],
            3 => self.id[3],
            4 => self.prg_size,
            5 => self.chr_size,
            6 => self.flags_6,
            7 => self.flags_7,
            9 => self.flags_9,
            10 => self.flags_10,
            _ => 0
        }
    }

    pub fn prg_size(&self) -> usize {
        self.prg_size as usize
    }

    pub fn chr_size(&self) -> usize {
        self.chr_size as usize
    }
}

struct Mmc {
    mapper: Mapper,
    prg_banks: usize,
    curr_prg0: usize,
    curr_prg1: usize,
    chr_banks: usize,
    curr_chr0: usize,

}

impl Mmc {
    pub fn new(map: u8, header: Header) -> Mmc {
        Mmc {
            mapper: Mapper::from_u8(map).unwrap(),
            prg_banks: header.prg_size(),
            chr_banks: header.chr_size(),
            curr_prg0: 0,
            curr_prg1: header.prg_size(),
            curr_chr0: 0,
        }
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    enum Mapper {
        NROM = 0,
        MMC1 = 1,
        UxROM = 2,
        CNROM = 3,
        MMC3 = 4,
        MMC5 = 5,
    }
}
