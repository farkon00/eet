use std::process::exit;
use super::state::*;

pub fn halt(state: &mut State) {
    println!("Program halted");
    state.regs.print();
    exit(0);
}