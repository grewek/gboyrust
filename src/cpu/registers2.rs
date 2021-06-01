struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

enum RegisterComplete {
    Af,
    Bc,
    De,
    Hl,
}

enum RegisterPart {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

type RegisterPair = (RegisterPart, RegisterPart);

impl From<RegisterComplete> for RegisterPair {
    fn from(dest: RegisterComplete) -> Self {
        match dest {
            RegisterComplete::Af => (RegisterPart::A, RegisterPart::F),
            RegisterComplete::Bc => (RegisterPart::B, RegisterPart::C),
            RegisterComplete::De => (RegisterPart::D, RegisterPart::E),
            RegisterComplete::Hl => (RegisterPart::H, RegisterPart::L),
        }
    }
}

impl Registers {
    fn read_value16_from(&mut self, dest: RegisterComplete) -> u16 {
        let pair: RegisterPair = dest.into();

        let hi_byte = self.read_value8_from(pair.0) as u16;
        let lo_byte = self.read_value8_from(pair.1) as u16;

        hi_byte << 8 | lo_byte
    }

    fn read_value8_from(&mut self, dest: RegisterPart) -> u8 {
        match dest {
            RegisterPart::A => self.a,
            RegisterPart::F => self.f,
            RegisterPart::B => self.b,
            RegisterPart::C => self.c,
            RegisterPart::D => self.d,
            RegisterPart::E => self.e,
            RegisterPart::H => self.h,
            RegisterPart::L => self.l,
        }
    }

    fn write_value16_to(&mut self, dest: RegisterComplete, value: u16) {
        let hi_byte = (value >> 8) as u8;
        let lo_byte = value as u8;

        let pair: RegisterPair = dest.into();

        self.write_value8_to(pair.0, hi_byte);
        self.write_value8_to(pair.1, lo_byte);
    }

    fn write_value8_to(&mut self, dest: RegisterPart, value: u8) {
        match dest {
            RegisterPart::A => self.a = value,
            RegisterPart::F => self.f = value,
            RegisterPart::B => self.b = value,
            RegisterPart::C => self.c = value,
            RegisterPart::D => self.d = value,
            RegisterPart::E => self.e = value,
            RegisterPart::H => self.h = value,
            RegisterPart::L => self.l = value,
        }
    }

    fn copy_word_to_register(&mut self, dest: RegisterComplete, src: u16) {
        self.write_value16_to(dest, src);
    }

    fn copy_byte_to_register(&mut self, dest: RegisterPart, src: u8) {
        self.write_value8_to(dest, src);
    }

    fn copy_register_to_register(&mut self, dest: RegisterPart, src: RegisterPart) {
        let src_value = self.read_value8_from(src);
        self.write_value8_to(dest, src_value);
    }
}
