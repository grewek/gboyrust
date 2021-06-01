pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

#[derive(Copy, Clone)]
pub enum RegWord {
    Af,
    Bc,
    De,
    Hl,
}

#[derive(Copy, Clone)]
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
    pub fn read_value16_from(&mut self, dest: RegWord) -> u16 {
        //NOTE: This transforms our 16bit register into a tuple of a byte register pair
        let pair: RegisterPair = dest.into();

        let hi_byte = self.read_value8_from(pair.0);
        let lo_byte = self.read_value8_from(pair.1);

        u16::from_le_bytes([lo_byte, hi_byte])
    }

    pub fn read_value8_from(&mut self, dest: RegByte) -> u8 {
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
