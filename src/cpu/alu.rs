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
    pub fn unsigned_byte_to_bcd(&mut self, a: u8) -> u8 {
        //NOTE: This is not my algorithm i translated eric haskins to a rust version the original can be found
        //      at: https://ehaskins.com/2018-01-30%20Z80%20DAA/

        let mut result = a;
        let mut correction: i8 = 0;

        if self.check_half_carry_flag() || !self.check_negative_flag() && (a & 0x0f) > 9 {
            correction |= 0x06;
        }

        if self.check_carry_flag() || !self.check_negative_flag() && a > 0x99 {
            correction |= 0x60;
            self.set_carry();
        }

        result = if self.check_negative_flag() {
            result.overflowing_add(-correction as u8).0
        } else {
            result.overflowing_add(correction as u8).0
        };

        result &= 0xFF;

        self.toggle_zero(result);
        self.clear_half_carry();

        result
    }
    pub fn shift_right_arithmetic_8(&mut self, a: u8) -> u8 {
        let carry = a & 0x01;
        let seventh_bit = (a >> 7) & 0x01;
        let result = (a >> 1) | (seventh_bit << 7);

        self.toggle_zero(result);
        self.clear_negative();
        self.clear_half_carry();
        self.toggle_carry(carry == 0x01);

        result
    }

    pub fn shift_right_logic_8(&mut self, a: u8) -> u8 {
        let carry = a & 0x01;
        let result = a >> 1;

        self.toggle_zero(result);
        self.clear_negative();
        self.clear_half_carry();
        self.toggle_carry(carry == 0x01);

        result
    }

    pub fn shift_left_8(&mut self, a: u8) -> u8 {
        let carry = (a >> 7) & 0x01;
        let result = a << 1;

        self.toggle_zero(result);
        self.clear_negative();
        self.clear_half_carry();
        self.toggle_carry(carry == 0x01);

        result
    }

    pub fn rotate_right_8_carry(&mut self, a: u8) -> u8 {
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

    pub fn rotate_left_8_carry(&mut self, a: u8) -> u8 {
        let carry = ((self.flags & CARRY_BIT_MASK) >> 4) & 0x01;
        let next_carry = (a >> 7) & 0x01;
        let mut result = a << 1;
        result |= carry;

        self.clear_zero();
        self.clear_negative();
        self.clear_half_carry();
        self.toggle_carry(next_carry == 0x01);

        result
    }

    pub fn rotate_right_8(&mut self, a: u8) -> u8 {
        let next_carry = a & 0x01;
        let result = a.rotate_right(1);

        self.toggle_zero(result);
        self.clear_negative();
        self.clear_half_carry();
        self.toggle_carry(next_carry == 0x01);

        result
    }
    pub fn rotate_left_8(&mut self, a: u8) -> u8 {
        let next_carry = (a >> 7) & 0x01;
        let result = a.rotate_left(1);

        self.toggle_zero(result);
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

    pub fn not_8(&mut self, a: u8) -> u8 {
        let result = !a;

        self.set_negative();
        self.set_half_carry();

        result
    }

    pub fn add_16(&mut self, a: u16, b: u16) -> u16 {
        let (result, overflow) = a.overflowing_add(b);

        self.clear_negative();
        //TODO: Half Carry !
        let carried_into_high_nybble = (a & 0x0FFF).overflowing_add(b & 0x0FFF).0;
        self.toggle_half_carry(carried_into_high_nybble > 0x0FFF);
        self.toggle_carry(overflow);

        result
    }

    pub fn add_signed_byte_16(&mut self, a: u16, b: i8) -> u16 {
        let (result, overflow) = (a as i16).overflowing_add(b as i16);

        self.clear_negative();

        let carried_into_high_nybble = ((a as i16) & 0x0FFF).overflowing_add((b as i16) & 0x0FFF).0;
        self.toggle_half_carry(carried_into_high_nybble > 0x0FFF);
        self.toggle_carry(overflow);

        result as u16
    }

    pub fn adc_8(&mut self, a: u8, b: u8) -> u8 {
        let carry = (self.flags >> 4) & 0x01;

        let (b_with_carry, overflow_bc) = b.overflowing_add(carry);
        let (result, overflow) = a.overflowing_add(b_with_carry);

        self.toggle_zero(result);
        self.clear_negative();

        let carried_into_high_nybble = (a & 0x0F).overflowing_add(b & 0x0F).0 + carry;
        self.toggle_half_carry(carried_into_high_nybble > 0x0F);

        self.toggle_carry(overflow_bc || overflow);

        result
    }

    pub fn add_8(&mut self, a: u8, b: u8) -> u8 {
        let (result, overflow) = a.overflowing_add(b);

        self.toggle_zero(result);
        self.clear_negative();

        let carried_into_high_nybble = (a & 0x0F).overflowing_add(b & 0x0F).0;
        self.toggle_half_carry(carried_into_high_nybble > 0x0F);

        self.toggle_carry(overflow);

        result
    }

    pub fn sbc_8(&mut self, a: u8, b: u8) -> u8 {
        let carry = (self.flags >> 4) & 0x01;

        let (b_with_carry, overflow_bc) = b.overflowing_add(carry);
        let (result, overflow) = a.overflowing_sub(b_with_carry);

        self.toggle_zero(result);
        self.set_negative();

        let carried_into_high_nybble = (a & 0x0F)
            .overflowing_sub(b & 0x0F)
            .0
            .overflowing_sub(carry)
            .0;

        self.toggle_half_carry(carried_into_high_nybble > 0x0F);

        self.toggle_carry(overflow_bc || overflow);

        result
    }

    pub fn sub_8(&mut self, a: u8, b: u8) -> u8 {
        let (result, overflow) = a.overflowing_sub(b);

        self.toggle_zero(result);
        self.set_negative();

        let carried_into_high_nybble = (a & 0x0F).overflowing_sub(b & 0x0F).0;
        self.toggle_half_carry(carried_into_high_nybble > 0x0F);

        self.toggle_carry(overflow);

        result
    }

    pub fn inc_16(&self, value: u16) -> u16 {
        value.overflowing_add(1).0
    }

    pub fn inc_8(&mut self, value: u8) -> u8 {
        let new = value.overflowing_add(1).0;

        //TODO: Half Carry
        let carried_into_high_nybble = (value & 0x0F).overflowing_add(1).0;
        self.toggle_half_carry(carried_into_high_nybble > 0x0F);

        self.toggle_zero(new);
        self.clear_negative();
        new
    }

    pub fn cp_8(&mut self, accu: u8, value: u8) {
        let (result, overflow) = accu.overflowing_sub(value);

        self.toggle_zero(result);
        self.set_negative();

        let carried_into_high_nybble = (accu & 0x0F).overflowing_sub(value & 0x0F).0;
        self.toggle_half_carry(carried_into_high_nybble > 0x0F);

        self.toggle_carry(overflow);
    }

    pub fn toggle_carry(&mut self, overflowed: bool) {
        if overflowed {
            self.set_carry();
        } else {
            self.clear_carry();
        }
    }

    pub fn toggle_zero(&mut self, value: u8) {
        if value == 0x00 {
            self.set_zero();
        } else {
            self.clear_zero();
        };
    }

    pub fn toggle_half_carry(&mut self, carried_into: bool) {
        if carried_into {
            self.set_half_carry();
        } else {
            self.clear_half_carry();
        }
    }

    pub fn clear_zero(&mut self) {
        self.flags &= !ZERO_BIT_MASK;
    }

    fn set_zero(&mut self) {
        self.flags |= ZERO_BIT_MASK;
    }

    pub fn set_carry(&mut self) {
        self.flags |= CARRY_BIT_MASK;
    }

    pub fn clear_carry(&mut self) {
        self.flags &= !CARRY_BIT_MASK;
    }

    pub fn set_half_carry(&mut self) {
        self.flags |= HALF_CARRY_BIT_MASK;
    }

    pub fn clear_half_carry(&mut self) {
        self.flags &= !HALF_CARRY_BIT_MASK;
    }

    pub fn clear_negative(&mut self) {
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

        let carried_into_high_nybble = (value & 0x0f).overflowing_sub(1).0;
        self.toggle_half_carry(carried_into_high_nybble > 0x0F);
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

        let result = alu.rotate_right_8_carry(value_zero);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let value_one = 0b00_00_00_01;

        let result = alu.rotate_right_8_carry(value_one);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_00_00);
        assert_eq!(flags, 0b00_01_00_00);

        let result = alu.rotate_right_8_carry(result);
        let flags = alu.flags();

        assert_eq!(result, 0b10_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8_carry(result);
        let flags = alu.flags();

        assert_eq!(result, 0b01_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8_carry(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_10_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8_carry(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_01_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8_carry(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_10_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8_carry(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_01_00);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8_carry(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_00_10);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8_carry(result);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_00_01);
        assert_eq!(flags, 0b00_00_00_00);

        let result = alu.rotate_right_8_carry(value_one);
        let flags = alu.flags();

        assert_eq!(result, 0b00_00_00_00);
        assert_eq!(flags, 0b00_01_00_00);
    }

    #[test]
    fn test_cp_8bit() {
        let mut alu = Alu::default();
        alu.cp_8(0x80, 0x80);

        assert_eq!(alu.flags, 0b11_00_00_00);

        alu.cp_8(0x80, 0xFF);

        assert_eq!(alu.flags, 0b01_11_00_00);

        alu.cp_8(0x79, 0x20);

        assert_eq!(alu.flags, 0b01_00_00_00);
    }

    #[test]
    fn test_toggle_carry() {
        let mut alu = Alu {
            flags: 0b00_01_00_00,
        };
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
        let mut alu = Alu {
            flags: 0b10_00_00_00,
        };

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
        let mut alu = Alu {
            flags: 0b11_11_00_00,
        };

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
