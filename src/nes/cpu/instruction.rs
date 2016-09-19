use super::opcode::{Opcode, AddrMode};

pub struct Instruction {
    pub opcode: Opcode,
    pub other_bytes: usize, // up to 2 additional bytes?
    pub addrmode: AddrMode,
}

impl Instruction {
    pub fn new(byte: u8) -> Instruction {
        let op = Opcode::convert(byte);
        let mode = AddrMode::convert(byte);
        let bytes = Opcode::num_bytes(op, mode);

        Instruction {
            opcode: op,
            other_bytes: bytes - 1,
            addrmode: mode,
        }
    }
}
