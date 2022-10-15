use super::state::*;

/// MOV - Move reg | mem, reg | mem
pub fn mov(opcode: i8, state: &mut State) {
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

/// MVI - Move imidiate, reg | mem
pub fn mvi(opcode: i8, state: &mut State) {
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