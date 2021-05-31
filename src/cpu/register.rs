use super::Cpu;

pub enum RegLable {
    Af,
    A,
    F,
    Bc,
    B,
    C,
    De,
    D,
    E,
    Hl,
    H,
    L,
}

impl RegLable {
    pub fn lable_to_register_readable<'a>(&self, cpu: &'a Cpu) -> &'a Register {
        match self {
            RegLable::Af => &cpu.af,
            RegLable::A => &cpu.af,
            RegLable::F => &cpu.af,
            RegLable::Bc => &cpu.bc,
            RegLable::B => &cpu.bc,
            RegLable::C => &cpu.bc,
            RegLable::De => &cpu.de,
            RegLable::D => &cpu.de,
            RegLable::E => &cpu.de,
            RegLable::Hl => &cpu.hl,
            RegLable::H => &cpu.hl,
            RegLable::L => &cpu.hl,
        }
    }

    pub fn lable_to_register_writeable<'a>(&self, cpu: &'a mut Cpu) -> &'a mut Register {
        match self {
            RegLable::Af => &mut cpu.af,
            RegLable::A => &mut cpu.af,
            RegLable::F => &mut cpu.af,
            RegLable::Bc => &mut cpu.bc,
            RegLable::B => &mut cpu.bc,
            RegLable::C => &mut cpu.bc,
            RegLable::De => &mut cpu.de,
            RegLable::D => &mut cpu.de,
            RegLable::E => &mut cpu.de,
            RegLable::Hl => &mut cpu.hl,
            RegLable::H => &mut cpu.hl,
            RegLable::L => &mut cpu.hl,
        }
    }
}

pub enum RegSize {
    Full,
    Half,
}

impl From<&RegLable> for RegSize {
    fn from(lbl: &RegLable) -> Self {
        match lbl {
            RegLable::Af => RegSize::Full,
            RegLable::A => RegSize::Half,
            RegLable::F => RegSize::Half,
            RegLable::Bc => RegSize::Full,
            RegLable::B => RegSize::Half,
            RegLable::C => RegSize::Half,
            RegLable::De => RegSize::Full,
            RegLable::D => RegSize::Half,
            RegLable::E => RegSize::Half,
            RegLable::Hl => RegSize::Full,
            RegLable::H => RegSize::Half,
            RegLable::L => RegSize::Half,
        }
    }
}

pub enum SizedArg {
    Size16(u16),
    Size8(u8),
}

impl From<SizedArg> for u16 {
    fn from(arg: SizedArg) -> Self {
        match arg {
            SizedArg::Size16(value) => value,
            SizedArg::Size8(_) => panic!("Cannot convert a 8bit sized argument to a 16bit one !"),
        }
    }
}

impl From<SizedArg> for u8 {
    fn from(arg: SizedArg) -> Self {
        match arg {
            SizedArg::Size16(_) => panic!("Cannot convert a 16bit sized argument to a 8bit one !"),
            SizedArg::Size8(value) => value,
        }
    }
}

pub struct Register(u16);

impl Register {
    pub fn new(value: u16) -> Self {
        Register(value)
    }
    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn get_high(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn get_low(&self) -> u8 {
        self.0 as u8
    }

    fn write_complete(&mut self, arg: SizedArg) {
        match arg {
            SizedArg::Size16(value) => self.0 = value,
            SizedArg::Size8(_) => panic!("Cannot write a 8bit value to a 16bit register !"),
        }
    }
    fn write_low(&mut self, arg: SizedArg) {
        let high_val = self.get_high();

        match arg {
            SizedArg::Size16(_) => panic!("Cannot write a 16 bit value to a 8 bit register !"),
            SizedArg::Size8(low_val) => self.0 = u16::from_le_bytes([low_val, high_val]),
        };
    }

    fn write_high(&mut self, arg: SizedArg) {
        let low_val = self.get_low();

        match arg {
            SizedArg::Size16(_) => panic!("Cannot write a 16 bit value to a 8 bit register !"),
            SizedArg::Size8(high_val) => self.0 = u16::from_le_bytes([low_val, high_val]),
        };
    }

    pub fn read_from_register(&self, lable: &RegLable) -> SizedArg {
        match lable {
            RegLable::Af => SizedArg::Size16(self.get()),
            RegLable::A => SizedArg::Size8(self.get_high()),
            RegLable::F => SizedArg::Size8(self.get_low()),
            RegLable::Bc => SizedArg::Size16(self.get()),
            RegLable::B => SizedArg::Size8(self.get_high()),
            RegLable::C => SizedArg::Size8(self.get_low()),
            RegLable::De => SizedArg::Size16(self.get()),
            RegLable::D => SizedArg::Size8(self.get_high()),
            RegLable::E => SizedArg::Size8(self.get_low()),
            RegLable::Hl => SizedArg::Size16(self.get()),
            RegLable::H => SizedArg::Size8(self.get_high()),
            RegLable::L => SizedArg::Size8(self.get_low()),
        }
    }

    pub fn write_to_register(&mut self, lable: &RegLable, arg: SizedArg) {
        match lable {
            RegLable::Af => self.write_complete(arg),
            RegLable::A => self.write_high(arg),
            RegLable::F => self.write_low(arg),
            RegLable::Bc => self.write_complete(arg),
            RegLable::B => self.write_high(arg),
            RegLable::C => self.write_low(arg),
            RegLable::De => self.write_complete(arg),
            RegLable::D => self.write_high(arg),
            RegLable::E => self.write_low(arg),
            RegLable::Hl => self.write_complete(arg),
            RegLable::H => self.write_high(arg),
            RegLable::L => self.write_low(arg),
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_rewrite_low_value() {
        let mut reg = Register(0x0000);

        reg.write_low(SizedArg::Size8(0xFF));
        assert_eq!(reg.get_low(), 0xFF);
        assert_eq!(reg.get_high(), 0x00);

        reg.write_high(SizedArg::Size8(0xAA));
        assert_eq!(reg.get_low(), 0xFF);
        assert_eq!(reg.get_high(), 0xAA);
    }
}
