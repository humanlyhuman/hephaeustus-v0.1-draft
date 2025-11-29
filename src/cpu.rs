use crate::cap::Capability;
use crate::mem::Memory;
use crate::trap::Trap;

pub struct CPU {
    pub r: [u64; 16],
    pub c: [Capability; 8],
    pub pc: u64,
    pub trap: Option<Trap>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            r: [0; 16],
            c: [Capability::null(); 8],
            pc: 0,
            trap: None,
        }
    }

    pub fn raise_trap(&mut self, t: Trap) {
        self.trap = Some(t);
    }

    pub fn is_trapped(&self) -> bool {
        self.trap.is_some()
    }

    pub fn step(&mut self, mem: &mut Memory) {
        if self.is_trapped() {
            return;
        }

        let inst = match mem.fetch16(self.pc, &self.c[1]) {
            Ok(v) => v,
            Err(t) => {
                self.raise_trap(t);
                return;
            }
        };

        self.pc = self.pc.wrapping_add(2);

        crate::exec::execute(self, mem, inst);
    }
}
