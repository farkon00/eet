use super::state::*;

fn set_zero_flag(result: i8, state: &mut State) {
    state.alu.zero = result == 0;
}

fn set_parity_flag(result: i8, state: &mut State) {
    state.alu.parity = result.count_ones() % 2 != 0;
}

fn set_sign_flag(result: i8, state: &mut State) {
    state.alu.sign = result < 0;
}

fn set_zps_flags(result: i8, state: &mut State) {
    set_zero_flag(result, state);
    set_parity_flag(result, state);
    set_sign_flag(result, state);
}

/// INR, DCR - Increment or decrement reg | mem
pub fn inr_dcr(opcode: u8, state: &mut State) {
    let dst_code = (opcode & 0b00111000) >> 3;
    let oper = if opcode & 0b00000001 == 0 {1} else {-1};
    let dst = {
        let dst = if dst_code == 6 { // Memory
            let mem_dst = state.regs.get_pair(Registers::PAIR_H);
            &mut state.memory[mem_dst as u8 as usize]
        }
        else { // Reg
            let reg_dst = state.regs.get_ref_by_id(dst_code);   
            reg_dst
        };
        *dst += oper;
        *dst
    };
    set_zps_flags(dst, state);
    state.alu.acarry = if oper == 1 {
        ((dst & 0xF) + 1) & 0x10 != 0
    }
    else {
        (dst & 0xF) - 1 < 0
    };
} 

fn get_alu_oper_src(opcode: u8, state: &mut State) -> i8 {
    if opcode & 0x40 == 0x40 { // Imidiate
        state.regs.pc += 1;
        return state.memory[(state.regs.pc-1) as usize];
    }
    let code = opcode & 0b111;
    if code == 6 { // Memory
        state.memory[state.regs.get_pair(Registers::PAIR_H) as u8 as usize]
    }
    else {
        state.regs.get_by_id(code)
    }
}

/// ADD r|M, ADI - Add operand to A
pub fn add(opcode: u8, state: &mut State) {
    let src = get_alu_oper_src(opcode, state);
    state.alu.carry = state.regs.a.checked_add(src).is_none();
    state.alu.acarry = ((state.regs.a & 0xF) + (src & 0xF)) & 0x10 != 0;
    state.regs.a += src;
    set_zps_flags(state.regs.a, state);
}

/// ADC r|M, ACI - Add operand and carry to A
pub fn adc(opcode: u8, state: &mut State) {
    let src = get_alu_oper_src(opcode, state);
    let carry = state.alu.carry as i8;
    
    state.alu.carry = state.regs.a.checked_add(src + carry).is_none();
    state.alu.acarry = ((state.regs.a & 0xF) + (src & 0xF) + carry) & 0x10 != 0;
    state.regs.a += src + carry;
    set_zps_flags(state.regs.a, state);
} 

/// SUB r|M, SUI - Subtract operand from A
pub fn sub(opcode: u8, state: &mut State) {
    let src = get_alu_oper_src(opcode, state);
    
    state.alu.carry = src > state.regs.a;
    state.alu.acarry = (state.regs.a & 0xF) - (src & 0xF) < 0;
    state.regs.a -= src;
    set_zps_flags(state.regs.a, state);
} 

/// SBB r|M, SBI - Subtract operand and borrow from A
pub fn sbb(opcode: u8, state: &mut State) {
    let src = get_alu_oper_src(opcode, state);
    let borrow = state.alu.carry as i8;

    state.alu.carry = (src + borrow) > state.regs.a;
    state.alu.acarry = (state.regs.a & 0xF) - (src & 0xF) - borrow < 0;
    state.regs.a -= src + borrow;
    set_zps_flags(state.regs.a, state);
}

/// ANA r|M, ANI - And operand with A
pub fn ana(opcode: u8, state: &mut State) {
    let src = get_alu_oper_src(opcode, state);
    state.regs.a = state.regs.a & src;

    set_zps_flags(state.regs.a, state);
    state.alu.carry = false;
    state.alu.acarry = false;
}

/// XRA r|M, XRI - Xor operand with A
pub fn xra(opcode: u8, state: &mut State) {
    let src = get_alu_oper_src(opcode, state);
    state.regs.a = state.regs.a ^ src;

    set_zps_flags(state.regs.a, state);
    state.alu.carry = false;
    state.alu.acarry = false;
}

/// ORA r|M, ORI - Or operand with A
pub fn ora(opcode: u8, state: &mut State) {
    let src = get_alu_oper_src(opcode, state);
    state.regs.a = state.regs.a | src;

    set_zps_flags(state.regs.a, state);
    state.alu.carry = false;
    state.alu.acarry = false;
}

/// CMP r|M, CPI - Compare operand with A and set flags
pub fn cmp(opcode: u8, state: &mut State) {
    let src = get_alu_oper_src(opcode, state);
    state.alu.zero = state.regs.a == src;
    state.alu.carry = state.regs.a < src;
    set_parity_flag(state.regs.a-src, state);
    set_sign_flag(state.regs.a-src, state);
    state.alu.acarry = (state.regs.a & 0xF) - (src & 0xF) < 0;
}