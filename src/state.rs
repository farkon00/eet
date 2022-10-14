pub struct State {
    pub regs: Registers,
    pub alu: Alu,
    pub memory: [i8; 0xffff]
}

pub struct Registers {
    pub pc: i16,
    pub sp: i16,
    pub a:  i8,
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

impl Registers {
    pub fn get_ref_by_id<'a>(&'a mut self, id: i8) -> &'a mut i8 {
        match id {
            7 => &mut self.a,
            0 => &mut self.b,
            1 => &mut self.c,
            2 => &mut self.d,
            3 => &mut self.e,
            4 => &mut self.h,
            5 => &mut self.l,
            _ => panic!("Invalid register id")
        }
    } 

    pub fn get_by_id(&self, id: i8) -> i8 {
        match id {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            7 => self.a,
            _ => panic!("Invalid register id")
        }
    }

    pub fn print(&self) {
        println!("Reg A: {}", self.a);
        println!("Reg B: {}", self.b);
        println!("Reg C: {}", self.c);
        println!("Reg D: {}", self.d);
        println!("Reg E: {}", self.e);
        println!("Reg H: {}", self.h);
        println!("Reg L: {}", self.l);
    }
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