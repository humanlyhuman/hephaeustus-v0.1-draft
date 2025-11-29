use crate::{cpu::CPU, mem::Memory, decode, isa::Inst};
use crate::cap::Capability;
use crate::trap::Trap;

pub fn execute(cpu: &mut CPU, mem: &mut Memory, raw: u16) {
    let inst = decode::decode(raw);

    match inst.opcode {
        0x0 => op_add(cpu, &inst),
        0x1 => op_addi(cpu, &inst),
        0x2 => op_div(cpu, &inst),
        0x3 => op_sub(cpu, &inst),
        0x4 => op_mul(cpu, &inst),
        0x5 => op_ld(cpu, mem, &inst),
        0x6 => op_st(cpu, mem, &inst),
        0x7 => op_br(cpu, &inst),
        0x8 => op_brz(cpu, &inst),
        0x9 => op_jmp(cpu, &inst),
        0xA => op_call(cpu, &inst),
        0xB => op_ret(cpu, &inst),
        0xC => op_syscall(cpu, &inst),
        0xD => cap_null(cpu, &inst),
        0xE => cap_copy(cpu, &inst),
        0xF => cap_offset(cpu, &inst),
        _ => cpu.raise_trap(Trap::IllegalInstruction),
    }
}

fn op_add(cpu: &mut CPU, i: &Inst) {
    cpu.r[i.rd as usize] = cpu.r[i.rs1 as usize].wrapping_add(cpu.r[i.rs2 as usize]);
}

fn op_addi(cpu: &mut CPU, i: &Inst) {
    cpu.r[i.rd as usize] = cpu.r[i.rs1 as usize].wrapping_add(i.imm as i64 as u64);
}

fn op_div(cpu: &mut CPU, i: &Inst) {
    let a = cpu.r[i.rs1 as usize];
    let b = cpu.r[i.rs2 as usize];
    if b == 0 {
        cpu.raise_trap(Trap::DivideByZero);
        return;
    }
    cpu.r[i.rd as usize] = a / b;
}

fn op_sub(cpu: &mut CPU, i: &Inst) {
    cpu.r[i.rd as usize] = cpu.r[i.rs1 as usize].wrapping_sub(cpu.r[i.rs2 as usize]);
}

fn op_mul(cpu: &mut CPU, i: &Inst) {
    cpu.r[i.rd as usize] = cpu.r[i.rs1 as usize].wrapping_mul(cpu.r[i.rs2 as usize]);
}

fn op_ld(cpu: &mut CPU, mem: &mut Memory, i: &Inst) {
    let cap = &cpu.c[2];
    let addr = cpu.r[i.rs1 as usize].wrapping_add(i.imm as i64 as u64);

    match mem.load64(addr, cap) {
        Ok(v) => cpu.r[i.rd as usize] = v,
        Err(t) => cpu.raise_trap(t),
    }
}

fn op_st(cpu: &mut CPU, mem: &mut Memory, i: &Inst) {
    let cap = &cpu.c[2];
    let addr = cpu.r[i.rs1 as usize].wrapping_add(i.imm as i64 as u64);
    let val  = cpu.r[i.rs2 as usize];

    if let Err(t) = mem.store64(addr, val, cap) {
        cpu.raise_trap(t);
    }
}

fn op_br(cpu: &mut CPU, i: &Inst) {
    if cpu.r[i.rs1 as usize] == cpu.r[i.rs2 as usize] {
        cpu.pc = cpu.pc.wrapping_add((i.imm as i64 * 2) as u64);
    }
}

fn op_brz(cpu: &mut CPU, i: &Inst) {
    if cpu.r[i.rs1 as usize] == 0 {
        cpu.pc = cpu.pc.wrapping_add((i.imm as i64 * 2) as u64);
    }
}


fn op_jmp(cpu: &mut CPU, i: &Inst) {
    if i.rs1 == 0 {
        // PC-relative jump (used for labels): pc += imm * 2
        cpu.pc = cpu.pc.wrapping_add((i.imm as i64 * 2) as u64);
    } else {
        // Register + offset jump
        cpu.pc = cpu.r[i.rs1 as usize].wrapping_add(i.imm as i64 as u64);
    }
}

fn op_call(cpu: &mut CPU, i: &Inst) {
    cpu.r[15] = cpu.pc;
    if i.rs1 == 0 {
        cpu.pc = cpu.pc.wrapping_add((i.imm as i64 * 2) as u64);
    } else {
        cpu.pc = cpu.r[i.rs1 as usize].wrapping_add(i.imm as i64 as u64);
   ();
    }
}

fn op_ret(cpu: &mut CPU, _i: &Inst) {
    cpu.pc = cpu.r[15];
}

fn op_syscall(cpu: &mut CPU, i: &Inst) {
    let n = cpu.r[i.rs1 as usize];
    cpu.raise_trap(Trap::Syscall(n));
}

fn cap_null(cpu: &mut CPU, inst: &Inst) {
    cpu.c[inst.rd as usize] = Capability::null();
}

fn cap_copy(cpu: &mut CPU, inst: &Inst) {
    cpu.c[inst.rd as usize] = cpu.c[inst.rs1 as usize];
}

fn cap_offset(cpu: &mut CPU, inst: &Inst) {
    let src = cpu.c[inst.rs1 as usize];

    if !src.valid || src.sealed {
        cpu.raise_trap(Trap::CapViolation);
        return;
    }

    let new_offset = src.offset.wrapping_add(inst.imm as i64 as u64);

    if !src.in_bounds(new_offset, 0) {
        cpu.raise_trap(Trap::OutOfBounds);
        return;
    }

    cpu.c[inst.rd as usize] = Capability {
        offset: new_offset,
        ..src
    };
}
