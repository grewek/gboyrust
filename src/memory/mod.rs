use crate::cpu::timer::RaiseInterruptFlag;
use crate::cpu::timer::TimerController;

pub const DIV_CLOCK_REGISTER: u16 = 0xFF04;
pub const TIMA_CLOCK_REGISTER: u16 = 0xFF05;
pub const TMA_CLOCK_REGISTER: u16 = 0xFF06;
pub const TAC_CLOCK_REGISTER: u16 = 0xFF07;

pub struct Memory {
    bytes: [u8; 0x10000],
    timer_controller: TimerController,
}

impl Default for Memory {
    fn default() -> Self {
        let mut default = Self {
            bytes: [0x00; 0x10000],
            timer_controller: TimerController::new(),
        };

        //TODO: This is just for testing need to fix this to the right values at a later point !
        default.bytes[TAC_CLOCK_REGISTER as usize] = 0xF8;
        default.bytes[0xFF44] = 0xFF;
        default.bytes[0xFF02] = 0xFF;

        default
    }
}

impl Memory {
    pub fn get_mem_slice(&self) -> &[u8] {
        &self.bytes[0..]
    }

    pub fn update_timer(&mut self, m_cycles: usize) {
        let timer_overflow = self.timer_controller.update(m_cycles);

        match timer_overflow {
            RaiseInterruptFlag::CustomTimerOverflowed => self.set_timer_ie(),
            _ => (),
        }
    }

    fn set_timer_ie(&mut self) {
        let if_reg = self.read(0xFF0F);
        let if_reg = if_reg | (0x01 << 2);
        self.write(0xFF0F, if_reg);
    }

    pub fn load_cartridge(&mut self, bytes: &[u8]) {
        for (i, byte) in bytes.iter().enumerate() {
            self.bytes[i] = *byte;
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04..=0xFF07 => self.timer_controller.read(addr),
            _ => self.read_generic(addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF02 => self.serial_write_debug(value),
            0xFF04 => self.timer_controller.timer_reset(),
            0xFF05 | 0xFF06 | 0xFF07 => self.timer_controller.write(addr, value),
            _ => self.write_generic(addr, value),
        };
        /*if addr == 0xFF02 && value == 0x81 {
            let byte = char::from(self.bytes[0xFF01]);
            print!("{}", byte);
        }

        self.bytes[addr as usize] = value;*/
    }

    fn read_generic(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }
    fn write_generic(&mut self, addr: u16, value: u8) {
        self.bytes[addr as usize] = value;
    }

    fn serial_write_debug(&mut self, value: u8) {
        if value == 0x81 {
            let byte = char::from(self.bytes[0xFF01]);
            print!("{}", byte);
        } else {
            self.bytes[0xFF02] = value;
        }
    }
}
