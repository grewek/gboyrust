pub const DIV_CLOCK_REGISTER: u16  = 0xFF04;
pub const TIMA_CLOCK_REGISTER: u16 = 0xFF05;
pub const TMA_CLOCK_REGISTER: u16  = 0xFF06;
pub const TAC_CLOCK_REGISTER: u16  = 0xFF07;

pub enum MemoryEvent {
    WriteDivRegister,
    WriteMemory,
}
pub struct Memory {
    bytes: [u8; 0x10000],
}

impl Default for Memory {
    fn default() -> Self {
        let mut default = Self {
            bytes: [0x00; 0x10000],
        };

        default.bytes[TAC_CLOCK_REGISTER as usize] = 0xF8; 
        //NOTE: This for the mooneye testsuit
        default.bytes[0xFF44] = 0xFF;
        default.bytes[0xFF02] = 0xFF;

        default
    }
}

impl Memory {
    pub fn get_mem_slice(&self) -> &[u8] {
        &self.bytes[0..]
    }
    pub fn load_cartridge(&mut self, bytes: &[u8]) {
        for (i, byte) in bytes.iter().enumerate() {
            self.bytes[i] = *byte;
        }
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) -> MemoryEvent {
        match addr {
            0xFF02 => self.serial_write_debug(value),
            0xFF04 => self.div_register_reset(),
            _ => self.write_generic(addr, value),

        }
        /*if addr == 0xFF02 && value == 0x81 {
            let byte = char::from(self.bytes[0xFF01]);
            print!("{}", byte);
        }

        self.bytes[addr as usize] = value;*/
    }

    pub fn update_div_register(&mut self, value: u8) {
        self.bytes[DIV_CLOCK_REGISTER as usize] = value;
    }

    pub fn update_tima_register(&mut self, value: u8) {
        self.bytes[TIMA_CLOCK_REGISTER as usize] = value;
    }

    fn write_generic(&mut self, addr: u16, value: u8) -> MemoryEvent {
        self.bytes[addr as usize] = value;

        MemoryEvent::WriteMemory
    }

    fn serial_write_debug(&mut self, value: u8) -> MemoryEvent {
        if value == 0x81 {
            let byte = char::from(self.bytes[0xFF01]);
            print!("{}", byte);
        } else {
            self.bytes[0xFF02] = value;
        }

        MemoryEvent::WriteMemory
    }

    fn div_register_reset(&mut self) -> MemoryEvent {
        self.bytes[0xFF04] = 0x00;
        MemoryEvent::WriteDivRegister
    }
}
