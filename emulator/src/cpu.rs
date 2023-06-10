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
            0x01 => self.inst_ld_bc_nn(),
            0x02 => self.inst_ld_bc_a(),
            0x03 => self.inst_inc_bc(),
            0x04 => self.inst_inc_b(),
            0x05 => self.inst_dec_b(),
            0x06 => self.inst_ld_b_n(),
            0x07 => self.inst_rlca(),
            0x08 => self.inst_ld_a16_sp(),
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

    fn inst_ld_bc_nn(&mut self) {
        let imm_data = self.read_next_byte();
        self.reg_b = imm_data;
        let imm_data = self.read_next_byte();
        self.reg_c = imm_data;
    }

    fn inst_ld_bc_a(&mut self) {
        let address = (self.reg_b as u16) << 8 | self.reg_c as u16;
        self.mem_bus[address as usize] = self.reg_a;
    }

    fn inst_inc_bc(&mut self) {
        let bc_val = (self.reg_b as u16) << 8 | self.reg_c as u16;
        let incremented_val = bc_val.wrapping_add(1);
        self.reg_b = (incremented_val << 8) as u8;
        self.reg_c = incremented_val as u8;
    }

    fn inst_inc_b(&mut self) {
        self.reg_b = self.reg_b.wrapping_add(1);
        self.fl_zero = self.reg_b == 0;
        self.fl_sub = false;
        self.fl_hc = (self.reg_b & 0x0F) == 0x00;
    }

    fn inst_dec_b(&mut self) {
        self.reg_b = self.reg_b.wrapping_sub(1);
        self.fl_zero = self.reg_b == 0;
        self.fl_sub = true;
        self.fl_hc = (self.reg_b & 0x0F) == 0x0F;
    }

    fn inst_ld_b_n(&mut self) {
        let imm_data = self.read_next_byte();
        self.reg_b = imm_data;
    }

    fn inst_rlca(&mut self) {
        let msb = (self.reg_a & 0x80) >> 7;
        self.fl_c = msb != 0;
        self.reg_a = (self.reg_a << 1) | msb;
        self.fl_zero = self.reg_a == 0;
        self.fl_sub = false;
        self.fl_hc = false;
    }

    fn inst_ld_a16_sp(&mut self) {
        let imm_data = self.read_next_byte();
        let imm_data2 = self.read_next_byte();
        let address = (imm_data as u16) << 8 | imm_data2 as u16;
        let address2 = address + 1;
        self.mem_bus[address as usize] = self.sreg_sp as u8;
        self.mem_bus[address2 as usize] = (self.sreg_sp >> 8) as u8;
    }
}