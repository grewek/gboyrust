const ZERO_BIT_MASK: u8 = 0x01 << ZERO_BIT_INDEX;
const NEGATIVE_BIT_MASK: u8 = 0x01 << NEGATIVE_BIT_INDEX;
const HALF_CARRY_BIT_MASK: u8 = 0x01 << HALF_CARRY_BIT_INDEX;
const CARRY_BIT_MASK: u8 = 0x01 << CARRY_BIT_INDEX;

const ZERO_BIT_INDEX: u8 = 7;
const NEGATIVE_BIT_INDEX: u8 = 6;
const HALF_CARRY_BIT_INDEX: u8 = 5;
const CARRY_BIT_INDEX: u8 = 4;

pub enum FlagState {
    Toggle(bool),
    Set,
    Clear,
    Untouched,
}

pub fn unsigned_byte_to_bcd(mut flags: u8, a: u8) -> (u8, u8) {
    //NOTE: This is not my algorithm i translated eric haskins to a rust version the original can be found
    //      at: https://ehaskins.com/2018-01-30%20Z80%20DAA/

    //TODO: Improve this
    let mut result = a;
    let mut correction: i8 = 0;
    let mut carry_should_be_set = false;

    if check_half_carry_flag(flags) || !check_negative_flag(flags) && (a & 0x0f) > 9 {
        correction |= 0x06;
    }

    if check_carry_flag(flags) || !check_negative_flag(flags) && a > 0x99 {
        correction |= 0x60;
        carry_should_be_set = true;
    }

    result = if check_negative_flag(flags) {
        result.overflowing_add(-correction as u8).0
    } else {
        result.overflowing_add(correction as u8).0
    };

    result &= 0xFF;

    flags = clear_zero(flags);
    flags = clear_half_carry(flags);
    flags = clear_carry(flags);
    flags = toggle_zero(flags, result == 0x00);

    flags = if carry_should_be_set { set_carry(flags) } else { flags };

    (flags, result)
}
pub fn shift_right_arithmetic_8(mut flags: u8, a: u8) -> (u8, u8) {
    let carry = a & 0x01;
    let seventh_bit = (a >> 7) & 0x01;
    let result = (a >> 1) | (seventh_bit << 7);

    flags = change_flags(flags, FlagState::Toggle(result == 0x00), 
                FlagState::Clear, 
                FlagState::Clear, 
                FlagState::Toggle(carry == 0x01));

    (flags, result)
}

pub fn shift_right_logic_8(mut flags: u8, a: u8) -> (u8, u8) {
    let carry = a & 0x01;
    let result = a >> 1;

    flags = change_flags(flags, FlagState::Toggle(result == 0x00),
                FlagState::Clear, 
                FlagState::Clear, 
                FlagState::Toggle(carry == 0x01));

    (flags, result)
}

pub fn shift_left_8(mut flags: u8, a: u8) -> (u8, u8) {
    let carry = (a >> 7) & 0x01;
    let result = a << 1;

    flags = change_flags(flags, FlagState::Toggle(result == 0x00),
                FlagState::Clear,
                FlagState::Clear,
                FlagState::Toggle(carry == 0x01));

    (flags, result)
}

pub fn rotate_left_8_carry(mut flags: u8, a: u8) -> (u8, u8) {
    let carry = ((flags & CARRY_BIT_MASK) >> 4) & 0x01;
    let next_carry = (a >> 7) & 0x01;
    let mut result = a << 1;
    result |= carry;

    flags = change_flags(flags,
                FlagState::Clear,
                FlagState::Clear,
                FlagState::Clear,
                FlagState::Toggle(next_carry == 0x01));

    (flags, result)
}

pub fn rotate_right_8_carry(mut flags: u8, a: u8) -> (u8, u8) {
    let carry = ((flags & CARRY_BIT_MASK) >> 4) & 0x01;
    let next_carry = a & 0x01;
    let mut result = a >> 1;
    result |= carry << 7;

    flags = change_flags(flags, 
                FlagState::Clear, 
                FlagState::Clear, 
                FlagState::Clear, 
                FlagState::Toggle(next_carry == 0x01));

    (flags, result)
}
pub fn rotate_right_8(mut flags: u8, a: u8) -> (u8, u8) {
    let next_carry = a & 0x01;
    let result = a.rotate_right(1);

    flags = change_flags(flags, 
                FlagState::Clear, 
                FlagState::Clear, 
                FlagState::Clear, 
                FlagState::Toggle(next_carry == 0x01));

    (flags, result)
}
pub fn rotate_left_8(mut flags: u8, a: u8) -> (u8, u8) {
    let next_carry = (a >> 7) & 0x01;
    let result = a.rotate_left(1);

    flags = change_flags(flags,
                FlagState::Toggle(result == 0x00),
                FlagState::Clear,
                FlagState::Clear,
                FlagState::Toggle(next_carry == 0x01));

    (flags, result)
}

pub fn or_8(mut flags: u8, a: u8, b: u8) -> (u8, u8) {
    let result = a | b;

    flags = change_flags(flags,
                FlagState::Toggle(result == 0x00),
                FlagState::Clear,
                FlagState::Clear,
                FlagState::Clear);


    (flags, result)
}

pub fn xor_8(mut flags: u8, a: u8, b: u8) -> (u8, u8) {
    let result = a ^ b;

    flags = change_flags(flags,
                FlagState::Toggle(result == 0x00),
                FlagState::Clear,
                FlagState::Clear,
                FlagState::Clear);

    (flags, result)
}

pub fn and_8(mut flags: u8, a: u8, b: u8) -> (u8, u8) {
    let result = a & b;

    flags = change_flags(flags,
                FlagState::Toggle(result == 0x00),
                FlagState::Clear,
                FlagState::Set,
                FlagState::Clear);

    (flags, result)
}

pub fn not_8(mut flags: u8, a: u8) -> (u8, u8) {
    let result = !a;

    flags = change_flags(flags,
                FlagState::Untouched,
                FlagState::Set,
                FlagState::Set,
                FlagState::Untouched);

    (flags, result)
}

pub fn add_16(mut flags: u8, a: u16, b: u16) -> (u8, u16) {
    let (result, overflow) = a.overflowing_add(b);

    let carried_into_high_nybble = (a & 0x0FFF).overflowing_add(b & 0x0FFF).0;

    flags = change_flags(flags,
                FlagState::Untouched,
                FlagState::Clear,
                FlagState::Toggle(carried_into_high_nybble > 0x0FFF),
                FlagState::Toggle(overflow));

    (flags, result)
}

pub fn add_8(mut flags: u8, a: u8, b: u8) -> (u8, u8) {
    let (result, overflow) = a.overflowing_add(b);
    let carried_into_high_nybble = (a & 0x0F).overflowing_add(b & 0x0F).0;

    flags = change_flags(flags,
                FlagState::Toggle(result == 0x00),
                FlagState::Clear,
                FlagState::Toggle(carried_into_high_nybble > 0x0F),
                FlagState::Toggle(overflow));


    (flags, result)
}

pub fn add_signed_byte_16(mut flags: u8, a: u16, b: i8) -> (u8, u16) {
    let (result, overflow) = (a as i16).overflowing_add(b as i16);
    let carried_into_high_nybble = ((a as i16) & 0x0FFF).overflowing_add((b as i16) & 0x0FFF).0;

    flags = change_flags(flags,
                FlagState::Untouched,
                FlagState::Clear,
                FlagState::Toggle(carried_into_high_nybble > 0x0FFF),
                FlagState::Toggle(overflow));

    (flags, result as u16)
}

pub fn adc_8(mut flags: u8, a: u8, b: u8) -> (u8, u8) {
    let carry = (flags >> 4) & 0x01;

    let (b_with_carry, overflow_bc) = b.overflowing_add(carry);
    let (result, overflow) = a.overflowing_add(b_with_carry);
    let carried_into_high_nybble = (a & 0x0F).overflowing_add(b & 0x0F).0 + carry;

    flags = change_flags(flags,
                FlagState::Toggle(result == 0x00),
                FlagState::Clear,
                FlagState::Toggle(carried_into_high_nybble > 0x0F),
                FlagState::Toggle(overflow_bc || overflow));

    (flags, result)
}

pub fn sbc_8(mut flags: u8, a: u8, b: u8) -> (u8, u8) {
    let carry = (flags >> 4) & 0x01;
    let (b_with_carry, overflow_bc) = b.overflowing_add(carry);
    let (result, overflow) = a.overflowing_sub(b_with_carry);
    let carried_into_high_nybble = (a & 0x0F)
        .overflowing_sub(b & 0x0F)
        .0
        .overflowing_sub(carry)
        .0;

    flags = change_flags(flags,
                FlagState::Toggle(result == 0x00),
                FlagState::Set,
                FlagState::Toggle(carried_into_high_nybble > 0x0F),
                FlagState::Toggle(overflow_bc || overflow));

    (flags, result)
}
pub fn sub_8(mut flags: u8, a: u8, b: u8) -> (u8, u8) {
    let (result, overflow) = a.overflowing_sub(b);
    let carried_into_high_nybble = (a & 0x0F).overflowing_sub(b & 0x0F).0;

    flags = change_flags(flags,
                FlagState::Toggle(result == 0x00),
                FlagState::Set,
                FlagState::Toggle(carried_into_high_nybble > 0x0F),
                FlagState::Toggle(overflow));

    (flags, result)
}

pub fn inc_16(value: u16) -> u16 {
    //TODO: Does this really change no flags at all ?
    value.overflowing_add(1).0
}

pub fn inc_8(mut flags: u8, value: u8) -> (u8, u8) {
    let new = value.overflowing_add(1).0;
    let carried_into_high_nybble = (value & 0x0F).overflowing_add(1).0;
    flags = change_flags(flags,
                FlagState::Toggle(new == 0x00),
                FlagState::Clear,
                FlagState::Toggle(carried_into_high_nybble > 0x0F),
                FlagState::Untouched);

    (flags, new)
}

pub fn cp_8(mut flags: u8, accu: u8, value: u8) -> u8 {
    let (result, overflow) = accu.overflowing_sub(value);
    let carried_into_high_nybble = (accu & 0x0F).overflowing_sub(value & 0x0F).0;

    flags = change_flags(flags,
                FlagState::Toggle(result == 0x00),
                FlagState::Set,
                FlagState::Toggle(carried_into_high_nybble > 0x0F),
                FlagState::Toggle(overflow));

    flags
}

pub fn dec_16(value: u16) -> u16 {
    value.overflowing_sub(1).0
}

pub fn dec_8(mut flags: u8, value: u8) -> (u8, u8) {
    let new = value.overflowing_sub(1).0;
    let carried_into_high_nybble = (value & 0x0f).overflowing_sub(1).0;

    flags = change_flags(flags,
                FlagState::Toggle(new == 0x00),
                FlagState::Set,
                FlagState::Toggle(carried_into_high_nybble > 0x0F),
                FlagState::Untouched);

    (flags, new)
}

pub fn toggle_carry(mut flags: u8, cond: bool) -> u8 {
    if cond {
        flags = set_carry(flags);
    } else {
        flags = clear_carry(flags);
    }

    flags
}
pub fn toggle_negative(mut flags: u8, cond: bool) -> u8 {
    todo!()
}

pub fn toggle_zero(mut flags: u8, cond: bool) -> u8 {
    if cond {
        flags = set_zero(flags);
    } else {
        flags = clear_zero(flags);
    };

    flags
}

pub fn toggle_half_carry(mut flags: u8, cond: bool) -> u8 {
    
    //value > 0x0F 
    if cond {
        flags = set_half_carry(flags);
    } else {
        flags = clear_half_carry(flags);
    }

    flags
}

pub fn change_flags(mut flags: u8, z_flag: FlagState, n_flag: FlagState,
                h_flag: FlagState, c_flag: FlagState) -> u8 {
    flags = match z_flag {
        FlagState::Toggle(result) => toggle_zero(flags, result),
        FlagState::Set => set_zero(flags),
        FlagState::Clear => clear_zero(flags),
        FlagState::Untouched => flags,
    };

    flags = match n_flag {
        FlagState::Toggle(result) => toggle_negative(flags, result),
        FlagState::Set => set_negative(flags),
        FlagState::Clear => clear_negative(flags),
        FlagState::Untouched => flags,
    };

    flags = match h_flag {
        FlagState::Toggle(result) => toggle_half_carry(flags, result),
        FlagState::Set => set_half_carry(flags),
        FlagState::Clear => clear_half_carry(flags),
        FlagState::Untouched => flags,
    };

    flags = match c_flag {
        FlagState::Toggle(result) => toggle_carry(flags, result),
        FlagState::Set => set_carry(flags),
        FlagState::Clear => clear_carry(flags),
        FlagState::Untouched => flags,
    };

    flags
}
pub fn clear_zero(mut flags: u8) -> u8 {
    flags &= !ZERO_BIT_MASK;
    flags
}

pub fn set_zero(mut flags: u8) -> u8 {
    flags |= ZERO_BIT_MASK;
    flags
}

pub fn set_carry(mut flags: u8) -> u8 {
    flags |= CARRY_BIT_MASK;
    flags
}

pub fn clear_carry(mut flags: u8) -> u8 {
    flags &= !CARRY_BIT_MASK;
    flags
}

pub fn set_half_carry(mut flags: u8) -> u8 {
    flags |= HALF_CARRY_BIT_MASK;
    flags
}

pub fn clear_half_carry(mut flags: u8) -> u8 {
    flags &= !HALF_CARRY_BIT_MASK;
    flags
}

pub fn clear_negative(mut flags: u8) -> u8 {
    flags &= !NEGATIVE_BIT_MASK;
    flags
}

pub fn set_negative(mut flags: u8) -> u8 {
    flags |= NEGATIVE_BIT_MASK;
    flags
}

pub fn check_zero_flag(flags: u8) -> bool {
    (flags >> ZERO_BIT_INDEX) & 0x01 == 0x01
}

pub fn check_negative_flag(flags: u8) -> bool {
    (flags >> NEGATIVE_BIT_INDEX) & 0x01 == 0x01
}

pub fn check_half_carry_flag(flags: u8) -> bool {
    (flags >> HALF_CARRY_BIT_INDEX) & 0x01 == 0x01
}

pub fn check_carry_flag(flags: u8) -> bool {
    (flags >> CARRY_BIT_INDEX) & 0x01 == 0x01
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_right_8bit() {
        let flags = 0b0000_0000;
        let value_zero = 0b00_00_00_00;

        let (flags, result) = rotate_right_8_carry(flags, value_zero);

        assert_eq!(result, 0b00_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let value_one = 0b00_00_00_01;

        let (flags, result) = rotate_right_8_carry(flags, value_one);

        assert_eq!(result, 0b00_00_00_00);
        assert_eq!(flags, 0b00_01_00_00);

        let (flags, result) = rotate_right_8_carry(flags, result);

        assert_eq!(result, 0b10_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (flags, result) = rotate_right_8_carry(flags, result);

        assert_eq!(result, 0b01_00_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (flags, result) = rotate_right_8_carry(flags, result);

        assert_eq!(result, 0b00_10_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (flags, result) = rotate_right_8_carry(flags, result);

        assert_eq!(result, 0b00_01_00_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (flags, result) = rotate_right_8_carry(flags, result);

        assert_eq!(result, 0b00_00_10_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (flags, result) = rotate_right_8_carry(flags, result);

        assert_eq!(result, 0b00_00_01_00);
        assert_eq!(flags, 0b00_00_00_00);

        let (flags, result) = rotate_right_8_carry(flags, result);

        assert_eq!(result, 0b00_00_00_10);
        assert_eq!(flags, 0b00_00_00_00);

        let (flags, result) = rotate_right_8_carry(flags, result);

        assert_eq!(result, 0b00_00_00_01);
        assert_eq!(flags, 0b00_00_00_00);

        let (flags, result) = rotate_right_8_carry(flags, value_one);

        assert_eq!(result, 0b00_00_00_00);
        assert_eq!(flags, 0b00_01_00_00);
    }

    #[test]
    fn test_cp_8bit() {
        let mut flags = 0b0000_0000;
        flags = cp_8(flags, 0x80, 0x80);

        assert_eq!(flags, 0b11_00_00_00);

        flags = cp_8(flags, 0x80, 0xFF);

        assert_eq!(flags, 0b01_11_00_00);

        flags = cp_8(flags, 0x79, 0x20);

        assert_eq!(flags, 0b01_00_00_00);
    }

    #[test]
    fn test_toggle_carry() {
        let mut flags = 0b00_01_00_00;
        flags = toggle_carry(flags, false);

        assert_eq!(flags, 0b00_00_00_00);

        flags = toggle_carry(flags, true);
        assert_eq!(flags, 0b00_01_00_00);

        flags = 0b11_01_00_00;

        flags = toggle_carry(flags, false);
        assert_eq!(flags, 0b11_00_00_00);

        flags = toggle_carry(flags, true);
        assert_eq!(flags, 0b11_01_00_00);
    }

    #[test]
    fn test_toggle_zero() {
        let mut flags = 0b10_00_00_00;

        flags = toggle_zero(flags, false);
        assert_eq!(flags, 0b00_00_00_00);

        flags = toggle_zero(flags, true);
        assert_eq!(flags, 0b10_00_00_00);

        flags = 0b11_01_00_00;

        flags = toggle_zero(flags, false);
        assert_eq!(flags, 0b01_01_00_00);
        flags = toggle_zero(flags, true);
        assert_eq!(flags, 0b11_01_00_00);
    }

    #[test]
    fn test_check_flags() {
        let mut flags = 0b11_11_00_00;

        assert_eq!(check_zero_flag(flags), true);
        assert_eq!(check_negative_flag(flags), true);
        assert_eq!(check_half_carry_flag(flags), true);
        assert_eq!(check_carry_flag(flags), true);

        flags = 0b01_01_00_00;

        assert_eq!(check_zero_flag(flags), false);
        assert_eq!(check_negative_flag(flags), true);
        assert_eq!(check_half_carry_flag(flags), false);
        assert_eq!(check_carry_flag(flags), true);

        flags = 0b010_10_00_00;

        assert_eq!(check_zero_flag(flags), true);
        assert_eq!(check_negative_flag(flags), false);
        assert_eq!(check_half_carry_flag(flags), true);
        assert_eq!(check_carry_flag(flags), false);
    }
}
