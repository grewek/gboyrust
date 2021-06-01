const ZERO_BIT_MASK: u8 = 0x01 << ZERO_BIT_INDEX;
const NEGATIVE_BIT_MASK: u8 = 0x01 << NEGATIVE_BIT_INDEX;
const HALF_CARRY_BIT_MASK: u8 = 0x01 << HALF_CARRY_BIT_INDEX;
const CARRY_BIT_MASK: u8 = 0x01 << CARRY_BIT_INDEX;

const ZERO_BIT_INDEX: u8 = 7;
const NEGATIVE_BIT_INDEX: u8 = 6;
const HALF_CARRY_BIT_INDEX: u8 = 5;
const CARRY_BIT_INDEX: u8 = 4;

pub struct Alu {
    flags: u8,
}

impl Default for Alu {
    fn default() -> Self {
        Self { flags: 0x00 }
    }
}

impl Alu {
    pub fn shift_right_8(&mut self, a: u8) -> u8 {
        let carry = a & 0x01;
        let result = a >> 1;

        self.toggle_zero(result);
        self.clear_negative();
        self.clear_half_carry();
        self.toggle_carry(carry == 0x01);

        result
    }

    pub fn rotate_right_8(&mut self, a: u8) -> u8 {
        let carry = ((self.flags & CARRY_BIT_MASK) >> 4) & 0x01;
        let next_carry = a & 0x01;
        let mut result = a >> 1;
        result |= carry << 7;

        self.clear_zero();
        self.clear_negative();
        self.clear_half_carry();
        self.toggle_carry(next_carry == 0x01);

        result
    }

    pub fn rotate_left_8(&mut self, a: u8) -> u8 {
        let next_carry = (a >> 7) & 0x01;
        let result = a.rotate_left(1);

        self.clear_zero();
        self.clear_negative();
        self.clear_half_carry();
        self.toggle_carry(next_carry == 0x01);

        result
    }

    pub fn or_8(&mut self, a: u8, b: u8) -> u8 {
        let result = a | b;

        self.toggle_zero(result);
        self.clear_negative();
        self.clear_half_carry();
        self.clear_carry();

        result
    }

    pub fn xor_8(&mut self, a: u8, b: u8) -> u8 {
        let result = a ^ b;

        self.toggle_zero(result);
        self.clear_negative();
        self.clear_half_carry();
        self.clear_carry();

        result
    }

    pub fn and_8(&mut self, a: u8, b: u8) -> u8 {
        let result = a & b;

        self.toggle_zero(result);
        self.clear_negative();
        self.set_half_carry();
        self.clear_carry();

        result
    }

    pub fn add_16(&mut self, a: u16, b: u16) -> u16 {
        let (result, overflow) = a.overflowing_add(b);

        self.clear_negative();
        //TODO: Half Carry !
        self.toggle_carry(overflow);

        result
    }

    pub fn adc_8(&mut self, a: u8, b: u8) -> u8 {
        let carry = (self.flags >> 4) & 0x01;

        let (b_with_carry, overflow_bc) = b.overflowing_add(carry);
        let (result, overflow) = a.overflowing_add(b_with_carry);

        self.toggle_zero(result);
        self.clear_negative();
        //TODO: Half Carry !
        self.toggle_carry(overflow_bc || overflow);

        result
    }

    pub fn add_8(&mut self, a: u8, b: u8) -> u8 {
        let (result, overflow) = a.overflowing_add(b);

        self.toggle_zero(result);
        self.clear_negative();
        //TODO: Half Carry !
        self.toggle_carry(overflow);

        result
    }

    pub fn sbc_8(&mut self, a: u8, b: u8) -> u8 {
        let carry = (self.flags >> 4) & 0x01;

        let (b_with_carry, overflow_bc) = b.overflowing_add(carry);
        let (result, overflow) = a.overflowing_sub(b_with_carry);

        self.toggle_zero(result);
        self.set_negative();
        //TODO: Half Carry !
        self.toggle_carry(overflow_bc || overflow);

        result
    }

    pub fn sub_8(&mut self, a: u8, b: u8) -> u8 {
        let (result, overflow) = a.overflowing_sub(b);

        self.toggle_zero(result);
        self.set_negative();
        //TODO: Half Carry
        self.toggle_carry(overflow);

        result
    }

    pub fn inc_16(&self, value: u16) -> u16 {
        value.overflowing_add(1).0
    }

    pub fn inc_8(&mut self, value: u8) -> u8 {
        let new = value.overflowing_add(1).0;

        //TODO: Half Carry
        self.toggle_zero(new);
        self.clear_negative();
        new
    }

    pub fn cp_8(&mut self, accu: u8, value: u8) -> u8 {
        let (result, overflow) = accu.overflowing_sub(value);

        self.toggle_zero(result);
        self.set_negative();
        //TODO: Half Carry !
        self.toggle_carry(overflow);

        self.flags
    }

    fn toggle_carry(&mut self, overflowed: bool) {
        if overflowed {
            self.set_carry();
        } else {
            self.clear_carry();
        }
    }
    fn toggle_zero(&mut self, value: u8) {
        if value == 0x00 {
            self.set_zero();
        } else {
            self.clear_zero();
        };
    }

    fn clear_zero(&mut self) {
        self.flags &= !ZERO_BIT_MASK;
    }

    fn set_zero(&mut self) {
        self.flags |= ZERO_BIT_MASK;
    }

    fn set_carry(&mut self) {
        self.flags |= CARRY_BIT_MASK;
    }

    fn clear_carry(&mut self) {
        self.flags &= !CARRY_BIT_MASK;
    }

    fn set_half_carry(&mut self) {
        self.flags |= HALF_CARRY_BIT_MASK;
    }

    fn clear_half_carry(&mut self) {
        self.flags &= !HALF_CARRY_BIT_MASK;
    }

    fn clear_negative(&mut self) {
        self.flags &= !NEGATIVE_BIT_MASK;
    }

    fn set_negative(&mut self) {
        self.flags |= NEGATIVE_BIT_MASK;
    }

    pub fn check_zero_flag(&self) -> bool {
        (self.flags >> ZERO_BIT_INDEX) & 0x01 == 0x01
    }

    pub fn check_negative_flag(&self) -> bool {
        (self.flags >> NEGATIVE_BIT_INDEX) & 0x01 == 0x01
    }

    pub fn check_half_carry_flag(&self) -> bool {
        (self.flags >> HALF_CARRY_BIT_INDEX) & 0x01 == 0x01
    }

    pub fn check_carry_flag(&self) -> bool {
        (self.flags >> CARRY_BIT_INDEX) & 0x01 == 0x01
    }

    pub fn dec_16(&self, value: u16) -> u16 {
        value.overflowing_sub(1).0
    }

    pub fn dec_8(&mut self, value: u8) -> u8 {
        let new = value.overflowing_sub(1).0;

        self.toggle_zero(new);
        self.set_negative();
        new
    }

    pub fn restore_flags(&mut self, new_flags: u8) {
        self.flags = new_flags;
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_right_8bit() {
        let mut alu = Alu::default();
        let value_zero = 0b00_00_00_00;

        let result = alu.rotate_right_8(value_zero);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let value_one = 0b00_00_00_01;

        let result = alu.rotate_right_8(value_one);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_00_00);
        assert_eq!(flags, 0b00_01_00_00);

        let result = alu.rotate_right_8(result);
        let flags = alu.flags();

        assert_eq!(result, 0b10_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8(result);
        let flags = alu.flags();

        assert_eq!(result, 0b01_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_10_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_01_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_10_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_01_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_00_10);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_00_01);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8(value_one);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_00_00);
        assert_eq!(flags, 0b00_01_00_00);
    }

    #[test]
    fn test_cp_8bit() {
        let mut cpu = Alu::default();
        let flags = cpu.cp_8(0x80, 0x80);

        assert_eq!(flags, 0b11_00_00_00);

        let flags = cpu.cp_8(0x80, 0xFF);

        assert_eq!(flags, 0b01_01_00_00);

        let flags = cpu.cp_8(0x79, 0x20);

        assert_eq!(flags, 0b01_00_00_00);
    }

    #[test]
    fn test_toggle_carry() {
        let mut alu = Alu::default();

        alu.flags = 0b00_01_00_00;
        alu.toggle_carry(false);

        assert_eq!(alu.flags, 0b00_00_00_00);

        alu.toggle_carry(true);
        assert_eq!(alu.flags, 0b00_01_00_00);

        alu.flags = 0b11_01_00_00;

        alu.toggle_carry(false);
        assert_eq!(alu.flags, 0b11_00_00_00);

        alu.toggle_carry(true);
        assert_eq!(alu.flags, 0b11_01_00_00);
    }

    #[test]
    fn test_toggle_zero() {
        let mut alu = Alu::default();

        alu.flags = 0b10_00_00_00;

        alu.toggle_zero(0x01);
        assert_eq!(alu.flags, 0b00_00_00_00);

        alu.toggle_zero(0x00);
        assert_eq!(alu.flags, 0b10_00_00_00);

        alu.flags = 0b11_01_00_00;

        alu.toggle_zero(0x01);
        assert_eq!(alu.flags, 0b01_01_00_00);
        alu.toggle_zero(0x00);
        assert_eq!(alu.flags, 0b11_01_00_00);
    }

    #[test]
    fn test_check_flags() {
        let mut alu = Alu::default();

        alu.flags = 0b11_11_00_00;

        assert_eq!(alu.check_zero_flag(), true);
        assert_eq!(alu.check_negative_flag(), true);
        assert_eq!(alu.check_half_carry_flag(), true);
        assert_eq!(alu.check_carry_flag(), true);

        alu.flags = 0b01_01_00_00;

        assert_eq!(alu.check_zero_flag(), false);
        assert_eq!(alu.check_negative_flag(), true);
        assert_eq!(alu.check_half_carry_flag(), false);
        assert_eq!(alu.check_carry_flag(), true);

        alu.flags = 0b010_10_00_00;

        assert_eq!(alu.check_zero_flag(), true);
        assert_eq!(alu.check_negative_flag(), false);
        assert_eq!(alu.check_half_carry_flag(), true);
        assert_eq!(alu.check_carry_flag(), false);
    }
}
