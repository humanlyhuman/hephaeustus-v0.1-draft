#[derive(Debug, Clone, Copy)]
pub enum Trap {
    IllegalInstruction,
    CapViolation,
    OutOfBounds,
    DivideByZero,
    Syscall(u64),
}

pub fn trap_name(t: Trap) -> &'static str {
    match t {
        Trap::IllegalInstruction => "Illegal Instruction",
        Trap::CapViolation => "Capability Violation",
        Trap::OutOfBounds => "Out Of Bounds",
        Trap::DivideByZero => "Divide By Zero",
        Trap::Syscall(_) => "Syscall",
    }
}
