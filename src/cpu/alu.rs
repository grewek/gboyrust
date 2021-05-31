use super::register::SizedArg;

const ZERO_BIT_MASK: u8 = 0x01 << 7;
const NEGATIVE_BIT_MASK: u8 = 0x01 << 6;
const HALF_CARRY_BIT_MASK: u8 = 0x01 << 5;
const CARRY_BIT_MASK: u8 = 0x01 << 4;

pub struct Alu {
    flags: u8,
}

impl Default for Alu {
    fn default() -> Self {
        Self { flags: 0x00 }
    }
}

impl Alu {
    pub fn shift_right_8(&mut self, a: u8) -> (u8, u8) {
        let carry = a & 0x01;
        let result = a >> 1;

        self.toggle_zero(result);
        self.clear_negative();
        self.clear_half_carry();
        self.toggle_carry(carry == 0x01);

        (result, self.flags)
    }

    pub fn rotate_right_8(&mut self, a: u8) -> (u8, u8) {
        let carry = ((self.flags & CARRY_BIT_MASK) >> 4) & 0x01;
        let next_carry = a & 0x01;
        let mut result = a >> 1;
        result |= carry << 7;

        self.clear_zero();
        self.clear_negative();
        self.clear_half_carry();
        self.toggle_carry(next_carry == 0x01);

        (result, self.flags)
    }

    pub fn rotate_left_8(&mut self, a: u8) -> (u8, u8) {
        let next_carry = (a >> 7) & 0x01;
        let mut result = a.rotate_left(1);

        self.clear_zero();
        self.clear_negative();
        self.clear_half_carry();
        self.toggle_carry(next_carry == 0x01);

        (result, self.flags) // 0000 | 1101 = 1101 // 1111 | 1101 = 1101 // 1111 | 0000 = 1111
    }

    fn or_8(&mut self, a: u8, b: u8) -> u8 {
        let result = a | b;

        self.toggle_zero(result);
        self.clear_negative();
        self.clear_half_carry();
        self.clear_carry();

        result
    }
    fn xor_8(&mut self, a: u8, b: u8) -> u8 {
        let result = a ^ b;

        self.toggle_zero(result);
        self.clear_negative();
        self.clear_half_carry();
        self.clear_carry();

        result
    }

    fn and_8(&mut self, a: u8, b: u8) -> u8 {
        let result = a & b;

        self.toggle_zero(result);
        self.clear_negative();
        self.set_half_carry();
        self.clear_carry();

        result
    }
    fn add_16(&mut self, a: u16, b: u16) -> u16 {
        let (result, overflow) = a.overflowing_add(b);

        self.clear_negative();
        //TODO: Half Carry !
        self.toggle_carry(overflow);

        result
    }

    fn adc_8(&mut self, a: u8, b: u8) -> u8 {
        let carry = (self.flags >> 4) & 0x01;

        let (b_with_carry, overflow_bc) = b.overflowing_add(carry);
        let (result, overflow) = a.overflowing_add(b_with_carry);

        self.toggle_zero(result);
        self.clear_negative();
        //TODO: Half Carry !
        self.toggle_carry(overflow_bc || overflow);

        result
    }
    fn add_8(&mut self, a: u8, b: u8) -> u8 {
        let (result, overflow) = a.overflowing_add(b);

        self.toggle_zero(result);
        self.clear_negative();
        //TODO: Half Carry !
        self.toggle_carry(overflow);

        result
    }

    //fn sub_16(&mut self, a: u16, b: u16) -> u16 {
    //    a.overflowing_sub(b).0
    //}

    fn sbc_8(&mut self, a: u8, b: u8) -> u8 {
        let carry = (self.flags >> 4) & 0x01;

        let (b_with_carry, overflow_bc) = b.overflowing_add(carry);
        let (result, overflow) = a.overflowing_sub(b_with_carry);

        self.toggle_zero(result);
        self.set_negative();
        //TODO: Half Carry !
        self.toggle_carry(overflow_bc || overflow);

        result
    }

    fn sub_8(&mut self, a: u8, b: u8) -> u8 {
        let (result, overflow) = a.overflowing_sub(b);

        self.toggle_zero(result);
        self.set_negative();
        //TODO: Half Carry
        self.toggle_carry(overflow);

        result
    }

    fn inc_16(&self, value: u16) -> u16 {
        value.overflowing_add(1).0
    }

    fn inc_8(&mut self, value: u8) -> u8 {
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

    fn dec_16(&self, value: u16) -> u16 {
        value.overflowing_sub(1).0
    }

    fn dec_8(&mut self, value: u8) -> u8 {
        let new = value.overflowing_sub(1).0;

        self.toggle_zero(new);
        self.set_negative();
        new
    }

    pub fn restore_flags(&mut self, new_flags: u8) {
        self.flags = new_flags;
    }

    pub fn sub_sized_args(&mut self, a: SizedArg, b: SizedArg) -> (SizedArg, Option<u8>) {
        match (a, b) {
            //(SizedArg::Size16(a), SizedArg::Size16(b)) => {
            //    (SizedArg::Size16(self.sub_16(a, b)), None)
            //}
            (SizedArg::Size8(a), SizedArg::Size8(b)) => {
                (SizedArg::Size8(self.sub_8(a, b)), Some(self.flags))
            }
            _ => panic!(
                "Tried to subtract values of different sizes i.e. a 8bit value with a 16bit value !"
            ),
        }
    }
    pub fn add_sized_args(&mut self, a: SizedArg, b: SizedArg) -> (SizedArg, Option<u8>) {
        match (a, b) {
            (SizedArg::Size16(a), SizedArg::Size16(b)) => {
                (SizedArg::Size16(self.add_16(a, b)), Some(self.flags))
            }
            (SizedArg::Size8(a), SizedArg::Size8(b)) => {
                (SizedArg::Size8(self.add_8(a, b)), Some(self.flags))
            }
            _ => panic!(
                "Tried to add values of different sizes i.e. a 8bit value with a 16bit value !"
            ),
        }
    }
    pub fn inc_sized_arg(&mut self, arg: SizedArg) -> (SizedArg, Option<u8>) {
        match arg {
            SizedArg::Size16(value) => (SizedArg::Size16(self.inc_16(value)), None),
            SizedArg::Size8(value) => (SizedArg::Size8(self.inc_8(value)), Some(self.flags)),
        }
    }

    pub fn dec_sized_arg(&mut self, arg: SizedArg) -> (SizedArg, Option<u8>) {
        match arg {
            SizedArg::Size16(value) => (SizedArg::Size16(self.dec_16(value)), None),
            SizedArg::Size8(value) => (SizedArg::Size8(self.dec_8(value)), Some(self.flags)),
        }
    }

    pub fn adc_sized_arg(&mut self, a: SizedArg, b: SizedArg) -> (SizedArg, Option<u8>) {
        match (a, b) {
            (SizedArg::Size8(a), SizedArg::Size8(b)) => {
                (SizedArg::Size8(self.adc_8(a, b)), Some(self.flags))
            }
            _ => panic!("blegh"),
        }
    }
    pub fn sbc_sized_arg(&mut self, a: SizedArg, b: SizedArg) -> (SizedArg, Option<u8>) {
        match (a, b) {
            (SizedArg::Size8(a), SizedArg::Size8(b)) => {
                (SizedArg::Size8(self.sbc_8(a, b)), Some(self.flags))
            }
            _ => panic!("blah"),
        }
    }

    pub fn and_sized_arg(&mut self, a: SizedArg, b: SizedArg) -> (SizedArg, Option<u8>) {
        match (a, b) {
            (SizedArg::Size8(a), SizedArg::Size8(b)) => {
                (SizedArg::Size8(self.and_8(a, b)), Some(self.flags))
            }
            _ => panic!("Whoopsie"),
        }
    }

    pub fn xor_sized_arg(&mut self, a: SizedArg, b: SizedArg) -> (SizedArg, Option<u8>) {
        match (a, b) {
            (SizedArg::Size8(a), SizedArg::Size8(b)) => {
                (SizedArg::Size8(self.xor_8(a, b)), Some(self.flags))
            }
            _ => panic!("Oh my god please refactor these nonsensical functions..."),
        }
    }

    pub fn or_sized_arg(&mut self, a: SizedArg, b: SizedArg) -> (SizedArg, Option<u8>) {
        match (a, b) {
            (SizedArg::Size8(a), SizedArg::Size8(b)) => {
                (SizedArg::Size8(self.or_8(a, b)), Some(self.flags))
            }
            _ => panic!("..."),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_right_8bit() {
        let mut alu = Alu::default();
        let value_zero = 0b00_00_00_00;

        let (result, flags) = alu.rotate_right_8(value_zero);

        assert_eq!(result, 0b00_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let value_one = 0b00_00_00_01;

        let (result, flags) = alu.rotate_right_8(value_one);

        assert_eq!(result, 0b00_00_00_00);
        assert_eq!(flags, 0b00_01_00_00);

        let (result, flags) = alu.rotate_right_8(result);

        assert_eq!(result, 0b10_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (result, flags) = alu.rotate_right_8(result);

        assert_eq!(result, 0b01_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (result, flags) = alu.rotate_right_8(result);

        assert_eq!(result, 0b00_10_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (result, flags) = alu.rotate_right_8(result);

        assert_eq!(result, 0b00_01_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (result, flags) = alu.rotate_right_8(result);

        assert_eq!(result, 0b00_00_10_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (result, flags) = alu.rotate_right_8(result);

        assert_eq!(result, 0b00_00_01_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (result, flags) = alu.rotate_right_8(result);

        assert_eq!(result, 0b00_00_00_10);
        assert_eq!(flags, 0b00_00_00_00);

        let (result, flags) = alu.rotate_right_8(result);

        assert_eq!(result, 0b00_00_00_01);
        assert_eq!(flags, 0b00_00_00_00);

        let (result, flags) = alu.rotate_right_8(value_one);

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
}
