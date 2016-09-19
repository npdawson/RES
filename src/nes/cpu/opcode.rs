use num::FromPrimitive;

enum_from_primitive! {
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Opcode { // aaabbbcc, aaa and cc determine opcode, bbb is addressing mode
        Brk = 0x00,
        Ora = 0x01,
        Asl = 0x02,
        Php = 0x08,
        Bpl = 0x10,
        Clc = 0x18,
        Bit = 0x20,
        And = 0x21,
        Rol = 0x22,
        Plp = 0x28,
        Bmi = 0x30,
        Sec = 0x38,
        Jmp = 0x40,
        Eor = 0x41,
        Lsr = 0x42,
        Pha = 0x48,
        Bvc = 0x50,
        Cli = 0x58,
        // JmpAbs = 0x60, // TODO handle this when running Jmp instruction?
        Adc = 0x61,
        Ror = 0x62,
        Pla = 0x68,
        Bvs = 0x70,
        Sei = 0x78,
        Sty = 0x80,
        Sta = 0x81,
        Stx = 0x82,
        Dey = 0x88,
        Txa = 0x8A,
        Bcc = 0x90,
        Tya = 0x98,
        Txs = 0x9A,
        Ldy = 0xA0,
        Lda = 0xA1,
        Ldx = 0xA2,
        Tay = 0xA8,
        Tax = 0xAA,
        Bcs = 0xB0,
        Clv = 0xB8,
        Tsx = 0xBA,
        Cpy = 0xC0,
        Cmp = 0xC1,
        Dec = 0xC2,
        Iny = 0xC8,
        Dex = 0xCA,
        Bne = 0xD0,
        Cld = 0xD8,
        Cpx = 0xE0,
        Sbc = 0xE1,
        Inc = 0xE2,
        Inx = 0xE8,
        Nop = 0xEA,
        Beq = 0xF0,
        Sed = 0xF8
    }
}

impl Opcode {
    pub fn convert(byte: u8) -> Opcode {
        match Self::from_u8(byte) {
            Some(op) => return op,
            None => {}
        }
        let byte = byte & 0xE3;
        match Self::from_u8(byte) {
            Some(op) => op,
            None => panic!("Opcode not recognized: {:#X}", byte)
        }
    }

    pub fn num_bytes(op: Opcode, mode: AddrMode) -> usize {
        let mut bytes = match op {
            Opcode::Brk => 1,
            Opcode::Clc => 1,
            Opcode::Cld => 1,
            Opcode::Cli => 1,
            Opcode::Clv => 1,
            Opcode::Dex => 1,
            Opcode::Dey => 1,
            Opcode::Inx => 1,
            Opcode::Iny => 1,
            Opcode::Jmp => 3,
            // Opcode::Jsr => 3,
            Opcode::Nop => 1,
            Opcode::Pha => 1,
            Opcode::Php => 1,
            Opcode::Pla => 1,
            Opcode::Plp => 1,
            Opcode::Sec => 1,
            Opcode::Sed => 1,
            Opcode::Sei => 1,
            Opcode::Tax => 1,
            Opcode::Tay => 1,
            Opcode::Tsx => 1,
            Opcode::Txa => 1,
            Opcode::Txs => 1,
            Opcode::Tya => 1,
            _   => 2,
        };

        match mode {
            AddrMode::Impl => bytes -= 1,
            AddrMode::Acc => bytes -= 1,
            AddrMode::Abs => bytes += 1,
            AddrMode::AbsX => bytes += 1,
            AddrMode::AbsY => bytes += 1,
            _ => {}
        }

        bytes
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum AddrMode {
        Impl = 0x00, // Implied
        IndX = 0x01, // Indexed Indirect
        Imm2 = 0x02,
        Zero = 0x04,
        Imm1 = 0x09,
        Rel = 0x10,
        Acc = 0x0A,
        Abs = 0x0C,
        Abs2 = 0x0D,
        IndY = 0x11, // Indirect Indexed
        ZeroX = 0x14, // becomes zeropage, Y with STX and LDX
        Impl2 = 0x18,
        AbsY = 0x19,
        Impl3 = 0x1A,
        AbsX = 0x1C, // becomes absolute, Y with LDX
    }
}

impl AddrMode {
    pub fn convert(byte: u8) -> AddrMode {
        let byte = byte & 0x1F;
        match Self::from_u8(byte) {
            Some(mode) => mode,
            None => panic!("Address Mode not recognized: {:#X}", byte)
        }
    }
}
