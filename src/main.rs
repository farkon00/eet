#![allow(overflowing_literals)] // Useful for literals for decoder and program
use std::collections::HashMap;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

pub mod state;
pub mod special_instructions;

use state::*;

const STATIC_INSTRUCTIONS: [(i8, fn(&mut State)); 1] = [
    (0x76, special_instructions::halt as fn(&mut State))
];
const DEBUG: bool = false;

fn run_instruction(opcode: i8, state: &mut State) {
    let static_instr = HashMap::from(STATIC_INSTRUCTIONS);
    match static_instr.get(&opcode) {
        Some(func) => func(state),
        None => {
            // MOV - Move reg | mem, reg | mem
            if opcode & 0b11000000 == 0x40 { // 0 1 D D D S S S where D D D - dst, S S S - src
                let src_code = opcode & 0b00000111;
                let src = if src_code == 6 { // Memory
                    state.memory[state.regs.get_pair(Registers::PAIR_H) as u8 as usize]
                }
                else { // Register
                    state.regs.get_by_id(src_code)
                };

                let dst_code = (opcode & 0b00111000) >> 3;
                let dst = if dst_code == 6 {
                    &mut state.memory[state.regs.get_pair(Registers::PAIR_H) as u8 as usize]
                }
                else {
                    state.regs.get_ref_by_id(dst_code)
                };

                if dst_code == 6 && src_code == 6 {
                    panic!("Memory to memory move not allowed")
                }

                *dst = src;
            }
            // MVI - Move imidiate, reg | mem
            else if opcode & 0b11000110 == 0b00000110 { // 0 1 D D D 1 1 0 where D D D - dst
                let imidiate = state.memory[state.regs.pc as usize];
                state.regs.pc += 1;
                let dst_code = (opcode & 0b00111000) >> 3;
                if dst_code == 6 { // Memory
                    let mem_dst = state.regs.get_pair(Registers::PAIR_H);
                    state.memory[mem_dst as u8 as usize] = imidiate;
                }
                else { // Reg
                    let reg_dst = state.regs.get_ref_by_id(dst_code);
                    *reg_dst = imidiate;
                }
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
        run_instruction(state.memory[(state.regs.pc-1) as usize], &mut state);
        if DEBUG {state.regs.print();}
    }
}