use super::ir::*;
use std::collections::HashMap;

pub fn allocate_registers(instrs: &[IRInst]) -> HashMap<String, u8> {
    let mut alloc = HashMap::new();
    let mut next_reg = 2u8; // r0 = zero, r1 = return

    for inst in instrs {
        match inst {
            IRInst::LoadImm(dst, _) |
            IRInst::Add(dst, _, _) |
            IRInst::Sub(dst, _, _) |
            IRInst::Mul(dst, _, _) |
            IRInst::Div(dst, _, _) => {
                if !alloc.contains_key(dst) {
                    alloc.insert(dst.clone(), next_reg);
                    next_reg += 1;
                }
            }
            _ => {}
        }
    }

    alloc
}
