use crate::parser::*;
use crate::opcodes::*;
use std::collections::HashMap;

pub fn emit(insts: &[Inst]) -> Result<Vec<u16>, String> {
    let mut out = Vec::new();
    let mut labels = HashMap::new();

    let mut pc = 0u64;
    for inst in insts {
        match inst {
            Inst::Label(s) => {
                labels.insert(s.clone(), pc);
            }
            Inst::Op(_, _) => pc += 2,
        }
    }

    let mut current_pc = 0u64;
    for inst in insts {
        if let Inst::Op(name, args) = inst {
            let op = opcode(name)
                .ok_or_else(|| format!("unknown instruction '{}'", name))?;

            let mut rd = 0u8;
            let mut rs1 = 0u8;
            let mut imm4 = 0u8;

            // Parse based on instruction type
            match name.as_str() {
                "syscall" => {
                    // syscall rs1 - syscall number in rs1
                    if args.is_empty() {
                        return Err("syscall requires register argument".to_string());
                    }
                    match &args[0] {
                        Arg::Reg(r) => rs1 = reg_index(r)
                            .ok_or_else(|| format!("invalid register '{}'", r))?,
                        _ => return Err("syscall requires register".to_string()),
                    }
                }
                "ret" => {
                    // ret has no arguments
                }
                "brz" => {
                    // brz rs1, label/imm
                    if args.len() < 2 {
                        return Err("brz requires 2 arguments".to_string());
                    }
                    match &args[0] {
                        Arg::Reg(r) => rs1 = reg_index(r)
                            .ok_or_else(|| format!("invalid register '{}'", r))?,
                        _ => return Err("brz arg1 must be register".to_string()),
                    }
                    match &args[1] {
                        Arg::Imm(v) => imm4 = *v as i8 as u8,
                        Arg::Label(s) => {
                            let target = *labels.get(s)
                                .ok_or_else(|| format!("undefined label '{}'", s))?;
                            let offset = ((target as i64 - (current_pc as i64 + 2)) / 2) as i8;
                            imm4 = offset as u8;
                        }
                        _ => return Err("brz arg2 must be immediate or label".to_string()),
                    }
                }
                "br" => {
                    // br rs1, rs2, label/imm
                    if args.len() < 3 {
                        return Err("br requires 3 arguments".to_string());
                    }
                    match &args[0] {
                        Arg::Reg(r) => rs1 = reg_index(r)
                            .ok_or_else(|| format!("invalid register '{}'", r))?,
                        _ => return Err("br arg1 must be register".to_string()),
                    }
                    match &args[1] {
                        Arg::Reg(r) => rd = reg_index(r)
                            .ok_or_else(|| format!("invalid register '{}'", r))?,
                        _ => return Err("br arg2 must be register".to_string()),
                    }
                    match &args[2] {
                        Arg::Imm(v) => imm4 = *v as i8 as u8,
                        Arg::Label(s) => {
                            let target = *labels.get(s)
                                .ok_or_else(|| format!("undefined label '{}'", s))?;
                            let offset = ((target as i64 - (current_pc as i64 + 2)) / 2) as i8;
                            imm4 = offset as u8;
                        }
                        _ => return Err("br arg3 must be immediate or label".to_string()),
                    }
                }
            "jmp" | "call" => {
                // first argument = register (r0 = PC-relative)
                rs1 = match &args[0] {
                    Arg::Reg(r) => reg_index(r)
                        .ok_or_else(|| format!("invalid register '{}'", r))?,
                    _ => return Err(format!("{} needs register as first argument", name)),
                };

                // second argument (optional) = imm or label
                let offset_i8 = if args.len() > 1 {
                    match &args[1] {
                        Arg::Imm(n) => {
                            let v = *n as i8;
                            if v < -8 || v > 7 {
                                return Err(format!("offset {} does not fit in 4-bit signed immediate", v));
                            }
                            v
                        }
                        Arg::Label(lab) => {
                            let target = *labels.get(lab)
                                .ok_or_else(|| format!("undefined label '{}'", lab))?;
                            let diff = (target as i64 - (current_pc as i64 + 2)) / 2;
                            if diff < -8 || diff > 7 {
                                return Err(format!("jump to {} is too far (±7 instructions max)", lab));
                            }
                            diff as i8
                        }
                        _ => return Err("second argument must be immediate or label".to_string()),
                    }
                } else {
                    0i8
                };

                imm4 = offset_i8 as u8;
            }
                _ => {
                    // Standard 3-operand format: op rd, rs1, rs2/imm
                    if !args.is_empty() {
                        match &args[0] {
                            Arg::Reg(r) => rd = reg_index(r)
                                .ok_or_else(|| format!("invalid register '{}'", r))?,
                            Arg::Cap(c) => rd = cap_index(c)
                                .ok_or_else(|| format!("invalid capability '{}'", c))?,
                            _ => {}
                        }
                    }
                    if args.len() > 1 {
                        match &args[1] {
                            Arg::Reg(r) => rs1 = reg_index(r)
                                .ok_or_else(|| format!("invalid register '{}'", r))?,
                            Arg::Cap(c) => rs1 = cap_index(c)
                                .ok_or_else(|| format!("invalid capability '{}'", c))?,
                            _ => {}
                        }
                    }
                    if args.len() > 2 {
                        match &args[2] {
                            Arg::Reg(r) => imm4 = reg_index(r)
                                .ok_or_else(|| format!("invalid register '{}'", r))?,
                            Arg::Cap(c) => imm4 = cap_index(c)
                                .ok_or_else(|| format!("invalid capability '{}'", c))?,
                            Arg::Imm(v) => imm4 = *v as i8 as u8,
                            _ => {}
                        }
                    }
                }
            }

            let raw = ((op as u16) << 12)
                | ((rd as u16) << 8)
                | ((rs1 as u16) << 4)
                | ((imm4 as u16) & 0xF);  // Mask to 4 bits to avoid overflow into rs1

            out.push(raw);
            current_pc += 2;
        }
    }

    Ok(out)
}