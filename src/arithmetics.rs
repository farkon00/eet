use super::state::*;

/// INR, DCR - Increment or decrement reg | mem
pub fn inr_dcr(opcode: u8, state: &mut State) {
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

fn get_arithm_oper_src(opcode: u8, state: &State) -> i8 {
    let code = opcode & 0b111;
    if code == 6 { // Memory
        state.memory[state.regs.get_pair(Registers::PAIR_H) as u8 as usize]
    }
    else {
        state.regs.get_by_id(code)
    }
}

/// ADD r|M - Add register or memory to A
pub fn add(opcode: u8, state: &mut State) {
    let reg = get_arithm_oper_src(opcode, state);
    state.alu.carry = state.regs.a.checked_add(reg).is_none();
    state.regs.a += reg;
}

/// ADC r| - Add register or memory and carry to A
pub fn adc(opcode: u8, state: &mut State) {
    let reg = get_arithm_oper_src(opcode, state);
    let carry = state.alu.carry as i8;
    state.alu.carry = state.regs.a.checked_add(reg + carry).is_none();
    state.regs.a += reg + carry;
} 

/// SUB r|M - Subtract register or memory from A
pub fn sub(opcode: u8, state: &mut State) {
    let reg = get_arithm_oper_src(opcode, state);
    state.alu.carry = reg > state.regs.a;
    state.regs.a -= reg;
} 

/// SBB r|M - Subtract register or memory and borrow from A
pub fn sbb(opcode: u8, state: &mut State) {
    let reg = get_arithm_oper_src(opcode, state);
    let borrow = state.alu.carry as i8;
    state.alu.carry = (reg + borrow) > state.regs.a;
    state.regs.a -= reg + borrow;
}

/// ANA r|M - And register or memory with A
pub fn ana(opcode: u8, state: &mut State) {
    let reg = get_arithm_oper_src(opcode, state);
    state.regs.a = state.regs.a & reg;
}

/// XRA r|M - Xor register or memory with A
pub fn xra(opcode: u8, state: &mut State) {
    let reg = get_arithm_oper_src(opcode, state);
    state.regs.a = state.regs.a ^ reg;
}

/// ORA r|M - Or register or memory with A
pub fn ora(opcode: u8, state: &mut State) {
    let reg = get_arithm_oper_src(opcode, state);
    state.regs.a = state.regs.a | reg;
}