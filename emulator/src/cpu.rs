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

    fn stack_push(value: u16, sp: &mut u16, memory: &mut [u8]) {
        memory[*sp as usize] = (value >> 8) as u8;
        *sp = sp.wrapping_sub(1);
        memory[*sp as usize] = value as u8;
    }
    
    fn stack_pop(sp: &mut u16, memory: &[u8]) -> u16 {
        *sp = sp.wrapping_add(1);
        let lo = memory[*sp as usize] as u16;
        *sp = sp.wrapping_add(1);
        let hi = memory[*sp as usize] as u16;
        (hi << 8) | lo
    }

    fn read_next_byte(&mut self) -> u8 {
        let byte = self.mem_bus[self.sreg_pc as usize];
        self.sreg_pc += 1;
        byte
    }

    fn read_next_word(&mut self) -> u16 {
        let lo = self.read_next_byte() as u16;
        let hi = self.read_next_byte() as u16;
        (hi << 8) | lo
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

    // Load instructions

    fn load<T>(dest: &mut T, src: T)
    where
        T: Copy,
    {
        *dest = src;
    }

    // Jumps and subroutines instructions

    fn jp(&mut self, addr: u16) {
        self.sreg_pc = addr;
    }

    fn jp_cond(&mut self, cond: bool, addr: u16) {
        if cond {
            self.sreg_pc = addr;
        }
    }

    fn jr(&mut self, offset: i8) {
        self.sreg_pc = self.sreg_pc.wrapping_add(offset as u16);
    }

    fn jr_cond(&mut self, cond: bool, offset: i8) {
        if cond {
            self.sreg_pc = self.sreg_pc.wrapping_add(offset as u16);
        }
    }

    fn call(&mut self, addr: u16) {
        CPU::stack_push(self.sreg_pc + 2, &mut self.sreg_sp, &mut self.mem_bus);
        self.sreg_pc = addr;
    }

    fn call_cond(&mut self, cond: bool, addr: u16) {
        if cond {
            CPU::stack_push(self.sreg_pc + 2, &mut self.sreg_sp, &mut self.mem_bus);
            self.sreg_pc = addr;
        }
    }

    fn ret(&mut self) {
        self.sreg_pc = CPU::stack_pop(&mut self.sreg_sp, &mut self.mem_bus);
    }


    fn ret_cond(&mut self, cond: bool) {
        if cond {
            self.sreg_pc = CPU::stack_pop(&mut self.sreg_sp, &mut self.mem_bus);
        }
    }

    fn rst(&mut self, addr: u16) {
        CPU::stack_push(self.sreg_pc, &mut self.sreg_sp, &mut self.mem_bus);
        self.sreg_pc = addr;
    }

    // INSTRUCTION SET

    fn inst_nop(&mut self) {
        // do nothing
    }

    fn inst_ld_bc_nn(&mut self) {
        let nn = self.read_next_word();
        self.sreg_pc += 2;
        self.reg_b = (nn >> 8) as u8;
        self.reg_c = nn as u8;
    }

    fn inst_ld_bc_a(&mut self) {
        let addr = ((self.reg_b as u16) << 8) | self.reg_c as u16;
        self.mem_bus[addr as usize] = self.reg_a;
        self.sreg_pc += 1;
    }

    fn inst_inc_bc(&mut self) {
        let bc = ((self.reg_b as u16) << 8) | self.reg_c as u16;
        let res = bc.wrapping_add(1);
        self.reg_b = (res >> 8) as u8;
        self.reg_c = res as u8;
        self.sreg_pc += 1;
    }

    fn inst_inc_b(&mut self) {
        self.reg_b = self.inc(self.reg_b);
        self.sreg_pc += 1;
    }

    fn inst_dec_b(&mut self) {
        self.reg_b = self.dec(self.reg_b);
        self.sreg_pc += 1;
    }

    fn inst_ld_b_n(&mut self) {
        self.reg_b = self.read_next_byte();
        self.sreg_pc += 1;
    }

    fn inst_rlca(&mut self) {
        self.reg_a = self.rlc(self.reg_a);
        self.sreg_pc += 1;
    }

    fn inst_ld_nn_sp(&mut self) {
        let nn = self.read_next_word();
        self.sreg_pc += 2;
        self.mem_bus[nn as usize] = (self.sreg_sp & 0xFF) as u8;
        self.mem_bus[(nn + 1) as usize] = (self.sreg_sp >> 8) as u8;
    }

    fn inst_add_hl_bc(&mut self) {
        let hl = ((self.reg_h as u16) << 8) | self.reg_l as u16;
        let bc = ((self.reg_b as u16) << 8) | self.reg_c as u16;
        let res = hl.wrapping_add(bc);
        self.reg_h = (res >> 8) as u8;
        self.reg_l = res as u8;
        self.sreg_pc += 1;
    }

    fn inst_ld_a_bc(&mut self) {
        let addr = ((self.reg_b as u16) << 8) | self.reg_c as u16;
        self.reg_a = self.mem_bus[addr as usize];
        self.sreg_pc += 1;
    }
}