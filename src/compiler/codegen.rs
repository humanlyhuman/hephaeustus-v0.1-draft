use super::ir::*;
use super::regalloc::*;

pub fn generate(prog: &IRProgram) -> Result<Vec<u16>, String> {
    let mut code = Vec::new();

    for func in &prog.functions {
        let alloc = allocate_registers(&func.instrs);

        for inst in &func.instrs {
            match inst {
                IRInst::LoadImm(dst, val) => {
                    let rd = alloc.get(dst).unwrap();
                    code.push(encode_addi(*rd, 0, *val as i8));
                }
                IRInst::Add(dst, a, b) => {
                    let rd = alloc.get(dst).unwrap();
                    let rs1 = alloc.get(a).unwrap();
                    let rs2 = alloc.get(b).unwrap();
                    code.push(encode_add(*rd, *rs1, *rs2));
                }
                IRInst::Sub(dst, a, b) => {
                    let rd = alloc.get(dst).unwrap();
                    let rs1 = alloc.get(a).unwrap();
                    let rs2 = alloc.get(b).unwrap();
                    code.push(encode_sub(*rd, *rs1, *rs2));
                }
                IRInst::Mul(dst, a, b) => {
                    let rd = alloc.get(dst).unwrap();
                    let rs1 = alloc.get(a).unwrap();
                    let rs2 = alloc.get(b).unwrap();
                    code.push(encode_mul(*rd, *rs1, *rs2));
                }
                IRInst::Div(dst, a, b) => {
                    let rd = alloc.get(dst).unwrap();
                    let rs1 = alloc.get(a).unwrap();
                    let rs2 = alloc.get(b).unwrap();
                    code.push(encode_div(*rd, *rs1, *rs2));
                }
                IRInst::Ret(v) => {
                    let rs = alloc.get(v).unwrap();
                    if *rs != 1 {
                        code.push(encode_addi(1, *rs, 0));
                    }
                    code.push(encode_syscall(0));
                }
            }
        }
    }

    Ok(code)
}

fn encode_add(rd: u8, rs1: u8, rs2: u8) -> u16 {
    (0x0 << 12) | ((rd as u16) << 8) | ((rs1 as u16) << 4) | (rs2 as u16)
}

fn encode_sub(rd: u8, rs1: u8, rs2: u8) -> u16 {
    (0x3 << 12) | ((rd as u16) << 8) | ((rs1 as u16) << 4) | (rs2 as u16)
}

fn encode_mul(rd: u8, rs1: u8, rs2: u8) -> u16 {
    (0x4 << 12) | ((rd as u16) << 8) | ((rs1 as u16) << 4) | (rs2 as u16)
}

fn encode_div(rd: u8, rs1: u8, rs2: u8) -> u16 {
    (0x2 << 12) | ((rd as u16) << 8) | ((rs1 as u16) << 4) | (rs2 as u16)
}

fn encode_addi(rd: u8, rs1: u8, imm: i8) -> u16 {
    (0x1 << 12) | ((rd as u16) << 8) | ((rs1 as u16) << 4) | ((imm as u8) as u16)
}

fn encode_syscall(n: u8) -> u16 {
    (0xC << 12) | ((n as u16) << 8)
}
