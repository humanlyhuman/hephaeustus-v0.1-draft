pub fn opcode(name: &str) -> Option<u8> {
    Some(match name {
        "add"     => 0x0,
        "addi"    => 0x1,
        "div"     => 0x2,
        "sub"     => 0x3,
        "mul"     => 0x4,
        "ld"      => 0x5,
        "st"      => 0x6,
        "br"      => 0x7,
        "brz"     => 0x8,
        "jmp"     => 0x9,
        "call"    => 0xA,
        "ret"     => 0xB,
        "syscall" => 0xC,
        "cap.null"   => 0xD,
        "cap.copy"   => 0xE,
        "cap.offset" => 0xF,
        _ => return None,
    })
}

pub fn reg_index(s: &str) -> Option<u8> {
    if s.starts_with('r') {
        let idx: u8 = s[1..].parse().ok()?;
        if idx < 16 { Some(idx) } else { None }
    } else {
        None
    }
}

pub fn cap_index(s: &str) -> Option<u8> {
    if s.starts_with('c') {
        let idx: u8 = s[1..].parse().ok()?;
        if idx < 8 { Some(idx) } else { None }
    } else {
        None
    }
}
