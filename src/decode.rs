// emulator/src/decode.rs
use crate::isa::Inst;

pub fn decode(raw: u16) -> Inst {
    let imm4 = (raw & 0xF) as u8;
    Inst {
        opcode: ((raw >> 12) & 0xF) as u8,
        rd:     ((raw >>  8) & 0xF) as u8,
        rs1:    ((raw >>  4) & 0xF) as u8,
        rs2:    imm4,  // Low 4 bits are either rs2 or immediate
        imm:    (imm4 as i8) << 4 >> 4,   // Sign-extend 4-bit to 8-bit
    }
}