//
struct Cpu {
    registers: Registers,
}

struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8
}

impl Registers {
    fn get_af(&self) -> u16 {
        return ((self.a as u16) << 8) | self.f as u16;
    }

    fn get_bc(&self) -> u16 {
        return ((self.b as u16) << 8) | self.c as u16;
    }

    fn get_de(&self) -> u16 {
        return ((self.d as u16) << 8) | self.e as u16;
    }

    fn get_hl(&self) -> u16 {
        return ((self.h as u16) << 8) | self.l as u16;
    }

    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    fn get_flag_z(&self) -> bool {
        return (self.f & 0x80) != 0;
    }

    fn get_flag_n(&self) -> bool {
        return (self.f & 0x40) != 0;
    }

    fn get_flag_h(&self) -> bool {
        return (self.f & 0x20) != 0;
    }

    fn get_flag_c(&self) -> bool {
        return (self.f & 0x10) != 0;
    }

    fn set_flag_z(&mut self, value: bool) {
        if value {
            self.f |= 0x80;
        } else {
            self.f &= 0x7F;
        }
    }

    fn set_flag_n(&mut self, value: bool) {
        if value {
            self.f |= 0x40;
        } else {
            self.f &= 0xBF;
        }
    }

    fn set_flag_h(&mut self, value: bool) {
        if value {
            self.f |= 0x20;
        } else {
            self.f &= 0xDF;
        }
    }

    fn set_flag_c(&mut self, value: bool) {
        if value {
            self.f |= 0x10;
        } else {
            self.f &= 0xEF;
        }
    }
}