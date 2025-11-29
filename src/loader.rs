use crate::{cpu::CPU, mem::Memory, cap::Capability};
use std::fs;

pub fn load_osl_bin(cpu: &mut CPU, mem: &mut Memory, path: &str) -> Result<(), String> {
    let data = fs::read(path).map_err(|e| format!("cannot read {}: {}", path, e))?;

    if data.len() < 0x28 {
        return Err("binary too small".to_string());
    }

    let rd_u64 = |offset: usize| -> u64 {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&data[offset..offset + 8]);
        u64::from_le_bytes(buf)
    };

// CORRECT OFFSETS:
let entry       = rd_u64(0x00);
let text_base   = rd_u64(0x08);
let text_size   = rd_u64(0x10);  // 0x10 = 16 → third field → text_size
let data_base   = rd_u64(0x18);  // 0x18 = 24
let data_size   = rd_u64(0x20);  // 0x20 = 32

    if text_base.checked_add(text_size).map_or(true, |end| end as usize > mem.bytes.len()) {
        return Err(format!("text section out of bounds: base={:#x} size={:#x}", text_base, text_size));
    }

    if data_base.checked_add(data_size).map_or(true, |end| end as usize > mem.bytes.len()) {
        return Err(format!("data section out of bounds: base={:#x} size={:#x}", data_base, data_size));
    }

    let text_start = 0x28;
    let data_start = text_start + text_size as usize;

    if data_start.checked_add(data_size as usize).map_or(true, |end| end > data.len()) {
        return Err("binary file truncated".to_string());
    }

    mem.bytes[text_base as usize..(text_base + text_size) as usize]
        .copy_from_slice(&data[text_start..text_start + text_size as usize]);

    if data_size > 0 {
        mem.bytes[data_base as usize..(data_base + data_size) as usize]
            .copy_from_slice(&data[data_start..data_start + data_size as usize]);
    }

    cpu.pc = entry;

    cpu.c[1] = Capability {
        base: text_base,
        length: text_size,
        offset: 0,
        perms: 4,
        valid: true,
        sealed: false,
    };

    cpu.c[2] = Capability {
        base: data_base,
        length: data_size,
        offset: 0,
        perms: 3,
        valid: true,
        sealed: false,
    };

    Ok(())
}
