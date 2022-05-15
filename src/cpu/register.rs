pub struct Registers {
    a: u8,
    pub f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RegWord {
    Af,
    Bc,
    De,
    Hl,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RegByte {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

type RegisterPair = (RegByte, RegByte);
impl From<RegWord> for RegisterPair {
    fn from(dest: RegWord) -> Self {
        match dest {
            RegWord::Af => (RegByte::A, RegByte::F),
            RegWord::Bc => (RegByte::B, RegByte::C),
            RegWord::De => (RegByte::D, RegByte::E),
            RegWord::Hl => (RegByte::H, RegByte::L),
        }
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            //NOTE: Need to find out all the start up values of the cpu currently i only know that the
            //      a register is 1 at startup...
            a: 0x01,
            f: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
        }
    }
    pub fn read_value16_from(&self, dest: RegWord) -> u16 {
        //NOTE: This transforms our 16bit register into a tuple of a byte register pair
        let pair: RegisterPair = dest.into();

        let hi_byte = self.read_value8_from(pair.0);
        let lo_byte = self.read_value8_from(pair.1);

        u16::from_le_bytes([lo_byte, hi_byte])
    }

    pub fn read_value8_from(&self, dest: RegByte) -> u8 {
        match dest {
            RegByte::A => self.a,
            RegByte::F => self.f,
            RegByte::B => self.b,
            RegByte::C => self.c,
            RegByte::D => self.d,
            RegByte::E => self.e,
            RegByte::H => self.h,
            RegByte::L => self.l,
        }
    }

    pub fn write_value16_to(&mut self, dest: RegWord, value: u16) {
        let bytes = value.to_le_bytes();

        //NOTE: This transforms our 16bit register into a tuple of a byte register pair
        let pair: RegisterPair = dest.into();

        self.write_value8_to(pair.0, bytes[1]);
        self.write_value8_to(pair.1, bytes[0]);
    }

    pub fn write_value8_to(&mut self, dest: RegByte, value: u8) {
        match dest {
            RegByte::A => self.a = value,
            RegByte::F => self.f = value,
            RegByte::B => self.b = value,
            RegByte::C => self.c = value,
            RegByte::D => self.d = value,
            RegByte::E => self.e = value,
            RegByte::H => self.h = value,
            RegByte::L => self.l = value,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_word_register_to_pair() {
        let register_af = RegWord::Af;

        let pair: RegisterPair = register_af.into();

        assert_eq!(pair.0, RegByte::A);
        assert_eq!(pair.1, RegByte::F);

        let register_bc = RegWord::Bc;

        let pair: RegisterPair = register_bc.into();

        assert_eq!(pair.0, RegByte::B);
        assert_eq!(pair.1, RegByte::C);

        let register_de = RegWord::De;

        let pair: RegisterPair = register_de.into();

        assert_eq!(pair.0, RegByte::D);
        assert_eq!(pair.1, RegByte::E);

        let register_hl = RegWord::Hl;

        let pair: RegisterPair = register_hl.into();

        assert_eq!(pair.0, RegByte::H);
        assert_eq!(pair.1, RegByte::L);
    }

    #[test]
    fn test_write_word_value_into_register() {
        let mut regs = Registers::new();
        let register_af = RegWord::Af;

        regs.write_value16_to(register_af, 0xFF80);

        assert_eq!(regs.a, 0xFF);
        assert_eq!(regs.f, 0x80);

        let register_bc = RegWord::Bc;

        regs.write_value16_to(register_bc, 0xAABB);

        assert_eq!(regs.b, 0xAA);
        assert_eq!(regs.c, 0xBB);

        let register_de = RegWord::De;

        regs.write_value16_to(register_de, 0xCCDD);

        assert_eq!(regs.d, 0xCC);
        assert_eq!(regs.e, 0xDD);

        let register_hl = RegWord::Hl;

        regs.write_value16_to(register_hl, 0xEE11);

        assert_eq!(regs.h, 0xEE);
        assert_eq!(regs.l, 0x11);
    }

    #[test]
    fn test_write_byte_into_register() {
        let mut regs = Registers::new();

        let reg_a = RegByte::A;
        regs.write_value8_to(reg_a, 0x11);
        assert_eq!(regs.a, 0x11);

        let reg_f = RegByte::F;
        regs.write_value8_to(reg_f, 0x22);
        assert_eq!(regs.f, 0x22);

        let reg_b = RegByte::B;
        regs.write_value8_to(reg_b, 0x33);
        assert_eq!(regs.b, 0x33);

        let reg_c = RegByte::C;
        regs.write_value8_to(reg_c, 0x44);
        assert_eq!(regs.c, 0x44);

        let reg_d = RegByte::D;
        regs.write_value8_to(reg_d, 0x55);
        assert_eq!(regs.d, 0x55);

        let reg_e = RegByte::E;
        regs.write_value8_to(reg_e, 0x66);
        assert_eq!(regs.e, 0x66);

        let reg_h = RegByte::H;
        regs.write_value8_to(reg_h, 0x77);
        assert_eq!(regs.h, 0x77);

        let reg_l = RegByte::L;
        regs.write_value8_to(reg_l, 0x88);
        assert_eq!(regs.l, 0x88);
    }

    #[test]
    fn read_word_from_register() {
        let mut regs = Registers {
            a: 0x11,
            f: 0x22,
            b: 0x33,
            c: 0x44,
            d: 0x55,
            e: 0x66,
            h: 0x77,
            l: 0x88,
        };

        let register_af = RegWord::Af;
        let result = regs.read_value16_from(register_af);
        assert_eq!(result, 0x1122);

        let register_bc = RegWord::Bc;
        let result = regs.read_value16_from(register_bc);
        assert_eq!(result, 0x3344);

        let register_de = RegWord::De;
        let result = regs.read_value16_from(register_de);
        assert_eq!(result, 0x5566);

        let register_hl = RegWord::Hl;
        let result = regs.read_value16_from(register_hl);
        assert_eq!(result, 0x7788);
    }

    #[test]
    fn read_byte_from_register() {
        let mut regs = Registers {
            a: 0x11,
            f: 0x22,
            b: 0x33,
            c: 0x44,
            d: 0x55,
            e: 0x66,
            h: 0x77,
            l: 0x88,
        };

        let reg_a = RegByte::A;
        let result = regs.read_value8_from(reg_a);
        assert_eq!(result, 0x11);

        let reg_f = RegByte::F;
        let result = regs.read_value8_from(reg_f);
        assert_eq!(result, 0x22);

        let reg_b = RegByte::B;
        let result = regs.read_value8_from(reg_b);
        assert_eq!(result, 0x33);

        let reg_c = RegByte::C;
        let result = regs.read_value8_from(reg_c);
        assert_eq!(result, 0x44);

        let reg_d = RegByte::D;
        let result = regs.read_value8_from(reg_d);
        assert_eq!(result, 0x55);

        let reg_e = RegByte::E;
        let result = regs.read_value8_from(reg_e);
        assert_eq!(result, 0x66);

        let reg_h = RegByte::H;
        let result = regs.read_value8_from(reg_h);
        assert_eq!(result, 0x77);

        let reg_l = RegByte::L;
        let result = regs.read_value8_from(reg_l);
        assert_eq!(result, 0x88);
    }
}
