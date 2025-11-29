// src/main.rs – EMULATOR (the program that runs .oslbin files)

mod cpu;
mod cap;
mod mem;
mod isa;
mod decode;
mod exec;
mod loader;
mod trap;

use cpu::CPU;
use mem::Memory;
use loader::load_osl_bin;
use trap::{Trap, trap_name};
use std::env;

fn handle_syscall(cpu: &mut CPU, mem: &mut Memory, n: u64) {
    match n {
        0 => {
            println!("Program exited with code {}", cpu.r[1]);
            cpu.trap = Some(Trap::IllegalInstruction); // This tells the loop to stop
        }
        1 => {
            println!("{}", cpu.r[1]);
        }
        2 => {
            let addr = cpu.r[1];
            let cap = &cpu.c[2];
            let mut p = addr;
            loop {
                let b = match mem.load8(p, cap) {
                    Ok(v) => v,
                    Err(t) => {
                        cpu.raise_trap(t);
                        return;
                    }
                };
                if b == 0 { break; }
                print!("{}", b as char);
                p += 1;
            }
            println!();
        }
        _ => println!("Unknown syscall {}", n),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <program.oslbin>", args[0]);
        return;
    }

    let mut cpu = CPU::new();
    let mut mem = Memory::new(4 * 1024 * 1024);
    load_osl_bin(&mut cpu, &mut mem, &args[1]).expect("failed to load");

    println!("Loaded program, starting execution...\n");

    loop {
        // Handle any trap (syscall or error)
        if let Some(trap) = cpu.trap.take() {
            match trap {
                Trap::Syscall(n) => handle_syscall(&mut cpu, &mut mem, n),
                _ => {
                    println!("Trap: {}", trap_name(trap));
                    break;
                }
            }
        }

        // If exit syscall set a trap, we break out of the loop
        if cpu.trap.is_some() {
            break;
        }

        // Run one instruction
        cpu.step(&mut mem);
    }
}