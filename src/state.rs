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
    pub const PAIR_B: i8 = 0;
    pub const PAIR_D: i8 = 1;
    pub const PAIR_H: i8 = 2;

    pub fn get_ref_by_id<'a>(&'a mut self, id: u8) -> &'a mut i8 {
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

    pub fn get_by_id(&self, id: u8) -> i8 {
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

    /// Ids in this one are internal for this emulator, not i8080 architecture
    /// BC - 0, DE - 1, HL - 2
    /// Returns value in little endian
    pub fn get_pair(&self, id: i8) -> i16 {
        let (regh, regl) = match id {
            Registers::PAIR_B => (self.b, self.c),
            Registers::PAIR_D => (self.d, self.e),
            Registers::PAIR_H => (self.h, self.l),
            _ => panic!("Invalid internal id of a pair")
        };
        ((regl as i16) << 8) | (regh as i16)
    }

    pub fn print(&self) {
        println!("");
        println!("A   B   C   D   E   H   L");
        let regs: [i8;7] = [
            self.a, self.b, self.c, self.d, self.e, self.h, self.l
        ];
        for reg in regs {
            let s = format!("{}", reg as u8);
            print!("{}", s);
            print!("{}", " ".repeat(4 - s.len()));
        }
        println!("");
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

impl Alu {
    pub fn print_flags(&self) {
        println!("Zero   Carry  Sign   Parity ACarry");
        let flags: [bool;5] = [
            self.zero, self.carry, self.sign, self.parity, self.acarry
        ];
        for flag in flags {
            print!("{}      ", (flag as u8));
        }
        println!("");
    }
}