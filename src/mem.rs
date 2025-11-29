use crate::cap::Capability;
use crate::trap::Trap;

pub struct Memory {
    pub bytes: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory { bytes: vec![0; size] }
    }

    pub fn load8(&self, addr: u64, cap: &Capability) -> Result<u8, Trap> {
        self.check_read(addr, 1, cap)?;
        Ok(self.bytes[addr as usize])
    }

    pub fn load64(&self, addr: u64, cap: &Capability) -> Result<u64, Trap> {
        self.check_read(addr, 8, cap)?;

        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.bytes[addr as usize..addr as usize + 8]);
        Ok(u64::from_le_bytes(buf))
    }

    pub fn store64(&mut self, addr: u64, val: u64, cap: &Capability) -> Result<(), Trap> {
        self.check_write(addr, 8, cap)?;

        let bytes = val.to_le_bytes();
        self.bytes[addr as usize..addr as usize + 8].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn fetch16(&self, pc: u64, cap: &Capability) -> Result<u16, Trap> {
        self.check_exec(pc, 2, cap)?;

        let mut buf = [0u8; 2];
        buf.copy_from_slice(&self.bytes[pc as usize..pc as usize + 2]);
        Ok(u16::from_le_bytes(buf))
    }

    fn check_read(&self, addr: u64, size: u64, cap: &Capability) -> Result<(), Trap> {
        if !cap.valid || cap.sealed {
            return Err(Trap::CapViolation);
        }
        if !cap.can_read() {
            return Err(Trap::CapViolation);
        }
        self.check_bounds(addr, size, cap)
    }

    fn check_write(&self, addr: u64, size: u64, cap: &Capability) -> Result<(), Trap> {
        if !cap.valid || cap.sealed {
            return Err(Trap::CapViolation);
        }
        if !cap.can_write() {
            return Err(Trap::CapViolation);
        }
        self.check_bounds(addr, size, cap)
    }

    fn check_exec(&self, addr: u64, size: u64, cap: &Capability) -> Result<(), Trap> {
        if !cap.valid || cap.sealed {
            return Err(Trap::CapViolation);
        }
        if !cap.can_exec() {
            return Err(Trap::CapViolation);
        }
        self.check_bounds(addr, size, cap)
    }

    fn check_bounds(&self, addr: u64, size: u64, cap: &Capability) -> Result<(), Trap> {
        let off = addr.wrapping_sub(cap.base);

        if !cap.in_bounds(off, size) {
            return Err(Trap::OutOfBounds);
        }

        let end = addr.checked_add(size).ok_or(Trap::OutOfBounds)?;
        if end as usize > self.bytes.len() {
            return Err(Trap::OutOfBounds);
        }

        Ok(())
    }
}
