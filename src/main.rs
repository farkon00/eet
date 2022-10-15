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
            else if opcode & 0b11000111 == 0b00000110 { // 0 1 D D D 1 1 0 where D D D - dst
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
            // INR, DCR - Increment or decrement reg | mem
            else if opcode & 0b11000110 == 0b00000100 { // 0 0 D D D 1 0 O where D D D - dst, O - is decrement
                let dst_code = (opcode & 0b00111000) >> 3;
                let oper = if opcode & 0b00000001 == 0 {1} else {-1};
                if dst_code == 6 { // Memory
                    let mem_dst = state.regs.get_pair(Registers::PAIR_H);
                    state.memory[mem_dst as u8 as usize] += oper;
                    state.alu.carry = !(state.memory[mem_dst as u8 as usize].checked_add(oper).is_none());
                }
                else { // Reg
                    let reg_dst = state.regs.get_ref_by_id(dst_code);
                    *reg_dst += oper;
                    state.alu.carry = !(reg_dst.checked_add(oper).is_none());
                }
            }
            // ADD r - Add r to A
            else if opcode & 0b11111000 == 0x80 { // 1 0 0 0 0 S S S where S S S - register to add
                let reg = state.regs.get_by_id(opcode & 7);
                state.alu.carry = state.regs.a.checked_add(reg).is_none();
                state.regs.a += reg;
            }
            // ADC r - Add r and carry to A
            else if opcode & 0b11111000 == 0x88 { // 1 0 0 0 1 S S S where S S S - register to add
                let reg = state.regs.get_by_id(opcode & 7);
                let carry = state.alu.carry as i8;
                state.alu.carry = state.regs.a.checked_add(reg + carry).is_none();
                state.regs.a += reg + carry;
            }
            // SUB r - Subtract r from A
            else if opcode & 0b11111000 == 0x90 { // 1 0 0 1 0 S S S where S S S - register to subtract
                let reg = state.regs.get_by_id(opcode & 7);
                state.alu.carry = reg > state.regs.a;
                state.regs.a -= reg;
            }
            // SBB r - Subtract r and borrow from to A
            else if opcode & 0b11111000 == 0x98 { // 1 0 0 1 1 S S S where S S S - register to subtract
                let reg = state.regs.get_by_id(opcode & 7);
                let borrow = state.alu.carry as i8;
                state.alu.carry = (reg + borrow) > state.regs.a;
                state.regs.a -= reg + borrow;
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
        if DEBUG {
            state.regs.print();
            state.alu.print_flags();
        }
    }
}