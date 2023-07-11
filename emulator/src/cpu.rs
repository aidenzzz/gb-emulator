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
    /*
        Constructor for CPU struct.
     */
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

    /*
        Helper functions for implementing instructions. 

        Starting with 8 BIT ARITHMETIC AND LOGICAL OPERATIONS
     */

    // ADC - Add with carry
    // Adds the value of the carry flag to the accumulator.
    // Z 0 H C
    fn adc(&mut self, num: u8) {
        let mut carry = 0;
        if self.fl_c {
            carry = 1;
        }

        let result = self.reg_a.wrapping_add(num).wrapping_add(carry);

        self.fl_zero = result == 0;
        self.fl_sub = false;
        self.fl_hc = (self.reg_a & 0xF) + (num & 0xF) + carry > 0xF;
        self.fl_c = (self.reg_a as u16) + (num as u16) + (carry as u16) > 0xFF;

        self.reg_a = result;
    }

    // ADD - Add
    // Adds the value of the operand to the accumulator.
    // Z 0 H C
    fn add(&mut self, num: u8) {
        let result = self.reg_a.wrapping_add(num);

        self.fl_zero = result == 0;
        self.fl_sub = false;
        self.fl_hc = (self.reg_a & 0xF) + (num & 0xF) > 0xF;
        self.fl_c = (self.reg_a as u16) + (num as u16) > 0xFF;

        self.reg_a = result;
    }

    // AND - Logical AND
    // Logically ANDs the value of the operand with the accumulator.
    // Z 0 1 0
    fn and(&mut self, num: u8) {
        let result = self.reg_a & num;

        self.fl_zero = result == 0;
        self.fl_sub = false;
        self.fl_hc = true;
        self.fl_c = false;

        self.reg_a = result;
    }

    // CP - Compare
    // Compares the value of the operand with the accumulator.
    // Z 1 H C 
    fn cp(&mut self, num: u8) {
        let result = self.reg_a.wrapping_sub(num);

        self.fl_zero = result == 0;
        self.fl_sub = true;
        self.fl_hc = (self.reg_a & 0x0f) < (num & 0x0f);
        self.fl_c = self.reg_a < num;
    }

    // for the increment and decrement instructions, i felt like it was easier to return the value rather than deal with mutable references and lifetimes.

    // DEC - Decrement
    // Decrements the value of the operand by 1.
    // Z 1 H -
    fn dec(&mut self, num: u8) -> u8 {
        let result = num.wrapping_sub(1);

        self.fl_zero = result == 0;
        self.fl_sub = true;
        self.fl_hc = (num & 0xF) == 0;
        result
    }

    // INC - Increment
    // Increments the value of the operand by 1.
    // Z 0 H -
    fn inc(&mut self, num: u8) -> u8 {
        let result = num.wrapping_add(1);

        self.fl_zero = result == 0;
        self.fl_sub = false;
        self.fl_hc = (num & 0xF) == 0xF;
        result
    }

    // OR - Logical OR
    // Logically ORs the value of the operand with the accumulator.
    // Z 0 0 0
    fn or(&mut self, num: u8) {
        let result = self.reg_a | num;

        self.fl_zero = result == 0;
        self.fl_sub = false;
        self.fl_hc = false;
        self.fl_c = false;

        self.reg_a = result;
    }

    // SBC - Subtract with carry
    // Subtracts the value of the carry flag from the accumulator.
    // Z 1 H C
    fn sbc(&mut self, num: u8) {
        let mut carry = 0;
        if self.fl_c {
            carry = 1;
        }

        let result = self.reg_a.wrapping_sub(num).wrapping_sub(carry);

        self.fl_zero = result == 0;
        self.fl_sub = true;
        self.fl_hc = (self.reg_a & 0xF) < (num & 0xF) + carry;
        self.fl_c = (self.reg_a as u16) < (num as u16) + (carry as u16);

        self.reg_a = result;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adc() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x01;
        cpu.fl_c = true;
        cpu.adc(0x01);
        assert_eq!(cpu.reg_a, 0x03);
        assert_eq!(cpu.fl_zero, false);
        assert_eq!(cpu.fl_sub, false);
        assert_eq!(cpu.fl_hc, false);
        assert_eq!(cpu.fl_c, false);

        cpu.reg_a = 0xFF;
        cpu.fl_c = true;
        cpu.adc(0x01);
        assert_eq!(cpu.reg_a, 0x01);
        assert_eq!(cpu.fl_zero, false);
        assert_eq!(cpu.fl_sub, false);
        assert_eq!(cpu.fl_hc, true);
        assert_eq!(cpu.fl_c, true);
    }

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x01;
        cpu.add(0x01);
        assert_eq!(cpu.reg_a, 0x02);
        assert_eq!(cpu.fl_zero, false);
        assert_eq!(cpu.fl_sub, false);
        assert_eq!(cpu.fl_hc, false);
        assert_eq!(cpu.fl_c, false);

        cpu.reg_a = 0xFF;
        cpu.add(0x01);
        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.fl_zero, true);
        assert_eq!(cpu.fl_sub, false);
        assert_eq!(cpu.fl_hc, true);
        assert_eq!(cpu.fl_c, true);
    }

    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x01;
        cpu.and(0x01);
        assert_eq!(cpu.reg_a, 0x01);
        assert_eq!(cpu.fl_zero, false);
        assert_eq!(cpu.fl_sub, false);
        assert_eq!(cpu.fl_hc, true);
        assert_eq!(cpu.fl_c, false);

        cpu.reg_a = 0xFF;
        cpu.and(0x00);
        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.fl_zero, true);
        assert_eq!(cpu.fl_sub, false);
        assert_eq!(cpu.fl_hc, true);
        assert_eq!(cpu.fl_c, false);
    }

    #[test]
    fn test_cp() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x01;
        cpu.cp(0x01);
        assert_eq!(cpu.reg_a, 0x01);
        assert_eq!(cpu.fl_zero, true);
        assert_eq!(cpu.fl_sub, true);
        assert_eq!(cpu.fl_hc, false);
        assert_eq!(cpu.fl_c, false);

        cpu.reg_a = 0x01;
        cpu.cp(0x02);
        assert_eq!(cpu.reg_a, 0x01);
        assert_eq!(cpu.fl_zero, false);
        assert_eq!(cpu.fl_sub, true);
        assert_eq!(cpu.fl_hc, true);
        assert_eq!(cpu.fl_c, true);

        cpu.reg_a = 0x02;
        cpu.cp(0x01);
        assert_eq!(cpu.reg_a, 0x02);
        assert_eq!(cpu.fl_zero, false);
        assert_eq!(cpu.fl_sub, true);
        assert_eq!(cpu.fl_hc, false);
        assert_eq!(cpu.fl_c, false);
    }

    #[test]
    fn test_dec() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x01;
        cpu.reg_a = cpu.dec(cpu.reg_a);
        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.fl_zero, true);
        assert_eq!(cpu.fl_sub, true);
        assert_eq!(cpu.fl_hc, false);
        assert_eq!(cpu.fl_c, false);

        cpu.reg_a = 0x00;
        cpu.reg_a = cpu.dec(cpu.reg_a);
        assert_eq!(cpu.reg_a, 0xFF);
        assert_eq!(cpu.fl_zero, false);
        assert_eq!(cpu.fl_sub, true);
        assert_eq!(cpu.fl_hc, true);
        assert_eq!(cpu.fl_c, false);
    }

    #[test]
    fn test_inc() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x01;
        cpu.reg_a = cpu.inc(0x01);
        assert_eq!(cpu.reg_a, 0x02);
        assert_eq!(cpu.fl_zero, false);
        assert_eq!(cpu.fl_sub, false);
        assert_eq!(cpu.fl_hc, false);
        assert_eq!(cpu.fl_c, false);

        cpu.reg_a = 0xFF;
        cpu.reg_a = cpu.inc(0xFF);
        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.fl_zero, true);
        assert_eq!(cpu.fl_sub, false);
        assert_eq!(cpu.fl_hc, true);
        assert_eq!(cpu.fl_c, false);
    }

    #[test]
    fn test_or() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x01;
        cpu.or(0x01);
        assert_eq!(cpu.reg_a, 0x01);
        assert_eq!(cpu.fl_zero, false);
        assert_eq!(cpu.fl_sub, false);
        assert_eq!(cpu.fl_hc, false);
        assert_eq!(cpu.fl_c, false);

        cpu.reg_a = 0x00;
        cpu.or(0x00);
        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.fl_zero, true);
        assert_eq!(cpu.fl_sub, false);
        assert_eq!(cpu.fl_hc, false);
        assert_eq!(cpu.fl_c, false);
    }

    #[test]
    fn test_sbc() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x01;
        cpu.fl_c = true;
        cpu.sbc(0x01);
        assert_eq!(cpu.reg_a, 0xFF);
        assert_eq!(cpu.fl_zero, false);
        assert_eq!(cpu.fl_sub, true);
        assert_eq!(cpu.fl_hc, true);
        assert_eq!(cpu.fl_c, true);

        cpu.reg_a = 0x00;
        cpu.fl_c = true;
        cpu.sbc(0x00);
        assert_eq!(cpu.reg_a, 0xFF);
        assert_eq!(cpu.fl_zero, false);
        assert_eq!(cpu.fl_sub, true);
        assert_eq!(cpu.fl_hc, true);
        assert_eq!(cpu.fl_c, true);
    }
}