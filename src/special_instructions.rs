use std::process::exit;
use super::state::*;

pub fn halt(state: &mut State) {
    println!("Program halted");
    println!("Reg B: {}", state.regs.b);
    println!("Reg C: {}", state.regs.c);
    println!("Reg D: {}", state.regs.d);
    println!("Reg E: {}", state.regs.e);
    println!("Reg H: {}", state.regs.h);
    println!("Reg L: {}", state.regs.l);
    exit(0);
}