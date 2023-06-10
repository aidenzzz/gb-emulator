#![allow(dead_code)]
struct CPU {
    // Registers
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,

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

    
}