pub struct State {
    pub regs: Registers,
    pub alu: Alu,
    pub memory: [i8; 0xffff]
}

pub struct Registers {
    pub pc: i16,
    pub sp: i16,
    pub b:  i8,
    pub c:  i8,
    pub d:  i8,
    pub e:  i8,
    pub h:  i8,
    pub l:  i8,
    /* I am not sure if we will need those
    pub w:  i8,
    pub z:  i8*/
}

pub struct Alu {
    pub acc: i8,
    pub act: i8,
    pub tmp: i8,
    pub zero: bool,
    pub carry: bool,
    pub sign: bool,
    pub parity: bool,
    pub acarry: bool
}