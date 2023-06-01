const ZERO_FLAG: u8 = 0b1000_0000;
const SUBTRACT_FLAG: u8 = 0b0100_0000;
const HALF_CARRY_FLAG: u8 = 0b0010_0000;
const CARRY_FLAG: u8 = 0b0001_0000;

/*
    Struct representing the CPU's registers.
*/
struct Registers {
    a: u8, // Accumulator
    f: FlagRegister, // Flags
    b: u8, // B
    c: u8, // C
    d: u8, // D
    e: u8, // E
    h: u8, // H
    l: u8, // L
}

/*
    Struct representing specifically the CPU's flag register.
*/
struct FlagRegister {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

/*
    Implemnts getters and setters for the CPU's registers.
*/
impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

impl std::convert::From<FlagRegister> for u8 {
    fn from(flag: FlagRegister) -> u8 {
        // this could be optimized i think? 
        let mut result = 0;
        if flag.z { result |= ZERO_FLAG; }
        if flag.n { result |= SUBTRACT_FLAG; }
        if flag.h { result |= HALF_CARRY_FLAG; }
        if flag.c { result |= CARRY_FLAG; }
        result
    }
}

impl std::convert::From<u8> for FlagRegister {
    fn from(byte: u8) -> FlagRegister {
        FlagRegister {
            z: byte & ZERO_FLAG != 0,
            n: byte & SUBTRACT_FLAG != 0,
            h: byte & HALF_CARRY_FLAG != 0,
            c: byte & CARRY_FLAG != 0,
        }
    }
}