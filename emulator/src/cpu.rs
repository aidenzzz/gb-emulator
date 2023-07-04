#![allow(dead_code)]
struct CPU {
    // Registers
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_l: u8,

    // Flags
    fl_zero: bool,
    fl_sub: bool,
    fl_hc: bool,
    fl_c: bool,

    // Special Registers
    sreg_pc: u16,
    sreg_sp: u16,

    mem_bus: [u8; 65536],
}

impl CPU {
    fn new() -> CPU {
        CPU {
            reg_a: 0x01,
            reg_b: 0xFF,
            reg_c: 0x13,
            reg_d: 0x00,
            reg_e: 0xC1,
            reg_h: 0x84,
            reg_l: 0x03,

            fl_zero: false,
            fl_sub: false,
            fl_hc: false,
            fl_c: false,

            sreg_pc: 0x0100,
            sreg_sp: 0xFFFE,

            mem_bus: [0; 65536],
        }
    }

    fn fetch_opcode(&mut self) -> u8 {
        let opcode = self.mem_bus[self.sreg_pc as usize];
        self.sreg_pc += 1;
        opcode
    }

    fn execute_instruction(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.inst_nop(),
            _ => panic!("Unsupported opcode: 0x{}", opcode),
        }
    }

    fn read_next_byte(&mut self) -> u8 {
        let byte = self.mem_bus[self.sreg_pc as usize];
        self.sreg_pc += 1;
        byte
    }

    fn inst_nop(&mut self) {
        // do nothing
    }

    // 8 bit arethmetic/logical instructions

    fn add(&mut self, val: u8) {
        let (res, overflow) = self.reg_a.overflowing_add(val);
        self.reg_a = res;
        self.fl_zero = self.reg_a == 0;
        self.fl_sub = false;
        self.fl_hc = (self.reg_a & 0x0F) + (val & 0x0F) > 0x0F;
        self.fl_c = overflow;
    }

    fn adc(&mut self, val: u8) {
        let (res, overflow) = self.reg_a.overflowing_add(val);
        let carry = if self.fl_c { 1 } else { 0 };
        let (res, overflow2) = res.overflowing_add(carry);
        self.reg_a = res;
        self.fl_zero = self.reg_a == 0;
        self.fl_sub = false;
        self.fl_hc = (self.reg_a & 0x0F) + (val & 0x0F) + carry > 0x0F;
        self.fl_c = overflow || overflow2;
    }

    fn and(&mut self, val: u8) {
        self.reg_a &= val;
        self.fl_zero = self.reg_a == 0;
        self.fl_sub = false;
        self.fl_hc = true;
        self.fl_c = false;
    }

    fn cp(&mut self, val: u8) {
        self.fl_zero = self.reg_a == val;
        self.fl_sub = true;
        self.fl_hc = (self.reg_a & 0x0F) < (val & 0x0F);
        self.fl_c = self.reg_a < val;
    }

    fn dec(&mut self, val: u8) -> u8 {
        let res = val.wrapping_sub(1);
        self.fl_zero = res == 0;
        self.fl_sub = true;
        self.fl_hc = (res & 0x0F) == 0x0F;
        res
    }

    fn inc(&mut self, val: u8) -> u8 {
        let res = val.wrapping_add(1);
        self.fl_zero = res == 0;
        self.fl_sub = false;
        self.fl_hc = (res & 0x0F) == 0;
        res
    }

    fn or(&mut self, val: u8) {
        self.reg_a |= val;
        self.fl_zero = self.reg_a == 0;
        self.fl_sub = false;
        self.fl_hc = false;
        self.fl_c = false;
    }

    fn sbc(&mut self, val: u8) {
        let (res, overflow) = self.reg_a.overflowing_sub(val);
        let carry = if self.fl_c { 1 } else { 0 };
        let (res, overflow2) = res.overflowing_sub(carry);
        self.reg_a = res;
        self.fl_zero = self.reg_a == 0;
        self.fl_sub = true;
        self.fl_hc = (self.reg_a & 0x0F) < (val & 0x0F) + carry;
        self.fl_c = overflow || overflow2;
    }

    fn sub(&mut self, val: u8) {
        let (res, overflow) = self.reg_a.overflowing_sub(val);
        self.reg_a = res;
        self.fl_zero = self.reg_a == 0;
        self.fl_sub = true;
        self.fl_hc = (self.reg_a & 0x0F) < (val & 0x0F);
        self.fl_c = overflow;
    }

    fn xor(&mut self, val: u8) {
        self.reg_a ^= val;
        self.fl_zero = self.reg_a == 0;
        self.fl_sub = false;
        self.fl_hc = false;
        self.fl_c = false;
    }

    // 16 bit arithmetic instructions

    fn add_16(&mut self, val: u16) {
        let res = ((self.reg_h as u16) << 8) | (self.reg_l as u16);
        let (res, overflow) = res.overflowing_add(val);
        self.reg_h = (res >> 8) as u8;
        self.reg_l = res as u8;
        self.fl_sub = false;
        self.fl_hc = (self.reg_l & 0x0F) + (val as u8 & 0x0F) > 0x0F;
        self.fl_c = overflow;
    }

    fn inc_16(&mut self, val: u16) -> u16 {
        let res = val.wrapping_add(1);
        self.fl_zero = res == 0;
        self.fl_sub = false;
        self.fl_hc = (res & 0x0F) == 0;
        res
    }

    fn dec_16(&mut self, val: u16) -> u16 {
        let res = val.wrapping_sub(1);
        self.fl_zero = res == 0;
        self.fl_sub = true;
        self.fl_hc = (res & 0x0F) == 0x0F;
        res
    }

    // Bit operations instructions

    fn bit(&mut self, bit: u8, val: u8) {
        self.fl_zero = (val & (1 << bit)) == 0;
        self.fl_sub = false;
        self.fl_hc = true;
    }

    fn res(&mut self, bit: u8, val: u8) -> u8 {
        val & !(1 << bit)
    }

    fn set(&mut self, bit: u8, val: u8) -> u8 {
        val | (1 << bit)
    }

    fn swap(&mut self, val: u8) -> u8 {
        (val >> 4) | (val << 4)
    }

    // Bit shift instructions

    fn rl(&mut self, val: u8) -> u8 {
        let carry = if self.fl_c { 1 } else { 0 };
        let res = (val << 1) | carry;
        self.fl_zero = res == 0;
        self.fl_sub = false;
        self.fl_hc = false;
        self.fl_c = (val & 0x80) != 0;
        res
    }

    fn rlc(&mut self, val: u8) -> u8 {
        let res = (val << 1) | (val >> 7);
        self.fl_zero = res == 0;
        self.fl_sub = false;
        self.fl_hc = false;
        self.fl_c = (val & 0x80) != 0;
        res
    }

    fn rr(&mut self, val: u8) -> u8 {
        let carry = if self.fl_c { 1 } else { 0 };
        let res = (val >> 1) | (carry << 7);
        self.fl_zero = res == 0;
        self.fl_sub = false;
        self.fl_hc = false;
        self.fl_c = (val & 0x01) != 0;
        res
    }

    fn rrc(&mut self, val: u8) -> u8 {
        let res = (val >> 1) | ((val & 0x01) << 7);
        self.fl_zero = res == 0;
        self.fl_sub = false;
        self.fl_hc = false;
        self.fl_c = (val & 0x01) != 0;
        res
    }

    fn sla(&mut self, val: u8) -> u8 {
        let res = val << 1;
        self.fl_zero = res == 0;
        self.fl_sub = false;
        self.fl_hc = false;
        self.fl_c = (val & 0x80) != 0;
        res
    }

    fn sra(&mut self, val: u8) -> u8 {
        let res = (val >> 1) | (val & 0x80);
        self.fl_zero = res == 0;
        self.fl_sub = false;
        self.fl_hc = false;
        self.fl_c = (val & 0x01) != 0;
        res
    }

    fn srl(&mut self, val: u8) -> u8 {
        let res = val >> 1;
        self.fl_zero = res == 0;
        self.fl_sub = false;
        self.fl_hc = false;
        self.fl_c = (val & 0x01) != 0;
        res
    }
}