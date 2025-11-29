#[derive(Clone, Copy, Debug)]
pub struct Capability {
    pub base: u64,
    pub length: u64,
    pub offset: u64,
    pub perms: u8,
    pub valid: bool,
    pub sealed: bool,
}

impl Capability {
    pub fn null() -> Self {
        Capability {
            base: 0,
            length: 0,
            offset: 0,
            perms: 0,
            valid: false,
            sealed: false,
        }
    }

    pub fn can_read(&self) -> bool { self.perms & 1 != 0 }
    pub fn can_write(&self) -> bool { self.perms & 2 != 0 }
    pub fn can_exec(&self) -> bool { self.perms & 4 != 0 }
    pub fn can_seal(&self) -> bool { self.perms & 0x80 != 0 }

    pub fn in_bounds(&self, off: u64, size: u64) -> bool {
        off.checked_add(size).map_or(false, |end| end <= self.length)
    }

    pub fn get_address(&self) -> u64 {
        self.base.wrapping_add(self.offset)
    }
}
