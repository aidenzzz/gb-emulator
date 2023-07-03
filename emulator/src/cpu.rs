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
}