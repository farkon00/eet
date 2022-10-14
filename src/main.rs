use std::collections::HashMap;

pub mod state;
pub mod special_instructions;

use state::*;

const PROGRAM: [i8;1] = [0x76];
const STATIC_INSTRUCTIONS: [(i8, fn(&mut State)); 1] = [
    (0x76, special_instructions::halt as fn(&mut State))
];

fn run_instruction(opcode: i8, state: &mut State) {
    let static_instr = HashMap::from(STATIC_INSTRUCTIONS);
    match static_instr.get(&opcode) {
        Some(func) => func(state),
        None => panic!("Opcode not implemented")
    }
}

fn main() {

    let mut state: State = State {
        regs: Registers { pc: 0, sp: -1, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0 },
        alu: Alu { acc: 0, act: 0, tmp: 0, zero: false, carry: false, sign: false, parity: false, acarry: false },
        memory: [0; 0xffff]
    };

    // Load program into the memory
    for i in 0..PROGRAM.len() {
        state.memory[i] = PROGRAM[i]; 
    }

    loop {
        state.regs.pc += 1;
        run_instruction(state.memory[(state.regs.pc-1) as usize], &mut state)
    }
}