use super::state::*;

/// INR, DCR - Increment or decrement reg | mem
pub fn inr_dcr(opcode: i8, state: &mut State) {
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

/// ADD r - Add r to A
pub fn add(opcode: i8, state: &mut State) {
    let reg = state.regs.get_by_id(opcode & 7);
    state.alu.carry = state.regs.a.checked_add(reg).is_none();
    state.regs.a += reg;
}

/// ADC r - Add r and carry to A
pub fn adc(opcode: i8, state: &mut State) {
    let reg = state.regs.get_by_id(opcode & 7);
    let carry = state.alu.carry as i8;
    state.alu.carry = state.regs.a.checked_add(reg + carry).is_none();
    state.regs.a += reg + carry;
} 

/// SUB r - Subtract r from A
pub fn sub(opcode: i8, state: &mut State) {
    let reg = state.regs.get_by_id(opcode & 7);
    state.alu.carry = reg > state.regs.a;
    state.regs.a -= reg;
} 

/// SBB r - Subtract r and borrow from A
pub fn sbb(opcode: i8, state: &mut State) {
    let reg = state.regs.get_by_id(opcode & 7);
    let borrow = state.alu.carry as i8;
    state.alu.carry = (reg + borrow) > state.regs.a;
    state.regs.a -= reg + borrow;
}