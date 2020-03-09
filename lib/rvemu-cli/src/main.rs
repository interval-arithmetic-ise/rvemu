pub mod stdio;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

use rvemu_core::bus::*;
use rvemu_core::cpu::*;

use stdio::*;

/// Output current registers to the console.
fn dump_registers(cpu: &Cpu) {
    println!("{}", cpu.xregs);
    println!("---------------------------------------------------");
    println!("{}", cpu.fregs);
    println!("---------------------------------------------------");
    println!("pc: {}", cpu.pc);
}

/// Main function of RISC-V emulator for the CLI version.
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./rvemu-cli <binary-file-name>");
    }

    let mut file = File::open(&args[1])?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut cpu = Arc::new(Mutex::new(Cpu::new()));
    {
        cpu.lock().expect("failed to get a mutable CPU.").pc = DRAM_BASE;
    }

    let mut bus = Bus::new();
    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    // Make a new thread for the standard input.
    let cloned_cpu = cpu.clone();
    let stdin_thread = thread::spawn(move || {
        stdin(cloned_cpu);
    });

    {
        // TODO: Get the lock inside the start function.
        let mut cpu = cpu.lock().expect("failed to get a mutable CPU.");
        cpu.start(&mut bus);

        dump_registers(&cpu);
    }

    let _ = stdin_thread.join();
    Ok(())
}
