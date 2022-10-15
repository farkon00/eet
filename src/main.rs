#![allow(overflowing_literals)] // Useful for literals for decoder and program
use std::collections::HashMap;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

pub mod state;
pub mod arithmetics;
pub mod mov;
pub mod special_instructions;

use state::*;

const STATIC_INSTRUCTIONS: [(u8, fn(&mut State)); 1] = [
    (0x76, special_instructions::halt as fn(&mut State))
];
const DEBUG: bool = false;

fn run_instruction(opcode: u8, state: &mut State) {
    let static_instr = HashMap::from(STATIC_INSTRUCTIONS);
    match static_instr.get(&opcode) {
        Some(func) => func(state),
        None => {
            if opcode & 0b11000000 == 0x40 { // 0 1 D D D S S S where D D D - dst, S S S - src
                mov::mov(opcode, state);
            }
            else if opcode & 0b11000111 == 0b00000110 { // 0 1 D D D 1 1 0 where D D D - dst
                mov::mvi(opcode, state);
            }
            else if opcode & 0b11000110 == 0b00000100 { // 0 0 D D D 1 0 O where D D D - dst, O - is decrement
                arithmetics::inr_dcr(opcode, state);
            }
            else if opcode & 0b11111000 == 0x80 { // 1 0 0 0 0 S S S where S S S - src to add
                arithmetics::add(opcode, state);
            }
            else if opcode & 0b11111000 == 0x88 { // 1 0 0 0 1 S S S where S S S - src to add
                arithmetics::adc(opcode, state);
            }
            else if opcode & 0b11111000 == 0x90 { // 1 0 0 1 0 S S S where S S S - src to subtract
                arithmetics::sub(opcode, state);
            }
            else if opcode & 0b11111000 == 0x98 { // 1 0 0 1 1 S S S where S S S - src to subtract
                arithmetics::sbb(opcode, state);
            }
            else {
                println!("Op code: {}", opcode);
                panic!("Not implemented op code")
            }
        }
    }
}

fn load_file(file_path: &str) -> Vec<u8> {
    let f = File::open(file_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    
    reader.read_to_end(&mut buffer).unwrap();
    
    if buffer[buffer.len()-1] != 0x76 { // No halt at the end
        buffer.push(0x76);
    }

    buffer
}

fn main() {

    let mut state: State = State {
        regs: Registers { pc: 0, sp: 0xffff, a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0 },
        alu: Alu { acc: 0, act: 0, tmp: 0, zero: false, carry: false, sign: false, parity: false, acarry: false },
        memory: [0; 0xffff]
    };

    let program = load_file("./test.bin");
    // Load program into the memory
    for i in 0..program.len() {
        state.memory[i] = program[i] as i8;
    }

    loop {
        state.regs.pc += 1;
        run_instruction(state.memory[(state.regs.pc-1) as usize] as u8, &mut state);
        if DEBUG {
            state.regs.print();
            state.alu.print_flags();
        }
    }
}