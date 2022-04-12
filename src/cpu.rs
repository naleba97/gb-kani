struct Cpu {
    reg_file: RegFile,
}

#[rustfmt::skip]
struct RegFile {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

enum Register8Bit {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum Register16Bit {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl RegFile {
    pub fn store_8bit_reg(&mut self, reg: Register8Bit, val: u8) {
        match reg {
            Register8Bit::A => self.a = val,
            Register8Bit::F => self.f.store_flags(val),
            Register8Bit::B => self.b = val,
            Register8Bit::C => self.c = val,
            Register8Bit::D => self.d = val,
            Register8Bit::E => self.e = val,
            Register8Bit::H => self.h = val,
            Register8Bit::L => self.l = val,
        }
    }

    pub fn load_8bit_reg(&mut self, reg: Register8Bit) -> u8 {
        match reg {
            Register8Bit::A => self.a,
            Register8Bit::F => self.f.load_flags(),
            Register8Bit::B => self.b,
            Register8Bit::C => self.c,
            Register8Bit::D => self.d,
            Register8Bit::E => self.e,
            Register8Bit::H => self.h,
            Register8Bit::L => self.l,
        }
    }


    pub fn load_16bit_reg(&mut self, reg: Register16Bit, val: u16) {
        match reg {
            Register16Bit::AF => {
                self.a = (val & 0xFF00) as u8;
                self.f.store_flags((val & 0x00FF) as u8);
            },
            Register16Bit::BC => {
                self.b = (val & 0xFF00) as u8;
                self.c = (val & 0x00FF) as u8;
            },
            Register16Bit::DE => {
                self.d = (val & 0xFF00) as u8;
                self.e = (val & 0x00FF) as u8;
            },
            Register16Bit::HL => {
                self.h = (val & 0xFF00) as u8;
                self.l = (val & 0x00FF) as u8;
            },
            Register16Bit::SP => {
                self.sp = val;
            },
            Register16Bit::PC => {
                self.pc = val;
            },
        }
    }
}

struct Flags {
    f: u8,
}

impl Flags {
    pub fn store_flags(&mut self, val: u8) {
        self.f = val;
    }

    pub fn load_flags(&mut self) -> u8 {
        self.f
    }

    /// Set the zero flag.
    pub fn set_z(&mut self) {
        self.f = self.f | 0x80;
    }

    /// Clear the zero flag.
    pub fn clear_z(&mut self) {
        self.f = self.f & !0x80;
    }

    /// Get the zero flag.
    pub fn get_z(&mut self) -> bool {
        (self.f & 0x80) > 0
    }

    /// Set the subtraction flag.
    pub fn set_n(&mut self) {
        self.f = self.f | 0x40;
    }

    /// Clears the subtraction flag.
    pub fn clear_n(&mut self) {
        self.f = self.f & !0x40;
    }

    /// Gets the subtraction flag.
    pub fn get_n(&mut self) -> bool {
        (self.f & 0x40) > 0
    }

    /// Set the half carry flag.
    pub fn set_h(&mut self) {
        self.f = self.f | 0x20;
    }

    /// Clear the half carry flag.
    pub fn clear_h(&mut self) {
        self.f = self.f & !0x20;
    }

    /// Get the half carry flag.
    pub fn get_h(&mut self) -> bool {
        (self.f & 0x20) > 0
    }

    /// Set the carry flag.
    pub fn set_c(&mut self) {
        self.f = self.f | 0x10;
    }

    /// Clears the carry flag.
    pub fn clear_c(&mut self) {
        self.f = self.f & !0x10;
    }

    /// Gets the carry flag.
    pub fn get_c(&mut self) -> bool {
        (self.f & 0x10) > 0
    }
}



