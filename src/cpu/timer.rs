use crate::memory::Memory;

trait TimerTick {
    fn tick(&mut self, t_cycles: usize, reset: usize) -> RaiseInterruptFlag;
}

trait DmaRead {
    fn dma_read(&self) -> u8;
}

trait DmaWrite {
    fn dma_write(&mut self, addr: u16, value: u8);
}

pub enum RaiseInterruptFlag {
    CustomTimerOverflowed,
    NoChanges,
}

#[derive(Debug)]
pub struct TimerController {
    div: DivRegister,
    tima: TimaRegister,

    t_cycles: usize,
    last_update: usize,
}

impl TimerController {
    pub fn new() -> Self {
        Self {
            div: DivRegister::new(),
            tima: TimaRegister::new(),

            t_cycles: 0x00,
            last_update: 0x00,
        }
    }

    pub fn timer_reset(&mut self) {
        self.div.internal_timer = 0;
        self.tima.last_tima_update = 0x00;
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => (self.div.internal_timer >> 8) as u8,
            0xFF05 => self.tima.tima_reg,
            0xFF06 => self.tima.tma_reg,
            0xFF07 => self.tima.tac_reg,
            _ => panic!("I/O Address out of timer range !"),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF05 => self.tima.tima_reg = value,
            0xFF06 => self.tima.tma_reg = value,
            0xFF07 => self.tima.tac_reg = value,
            _ => panic!("Invalid address for the timer register"),
        }
    }

    pub fn update(&mut self, m_cycles: usize) -> RaiseInterruptFlag {
        self.t_cycles = m_cycles * 4;
        let _ = self.div.tick(self.t_cycles - self.last_update, 0x00);
        let timer_overflow = self.tima.tick(self.div.internal_timer, 0x00);
        self.last_update = self.t_cycles;

        timer_overflow
    }
}

#[derive(Debug)]
struct DivRegister {
    internal_timer: usize,
}

impl DivRegister {
    fn new() -> Self {
        Self { internal_timer: 0 }
    }
}

impl TimerTick for DivRegister {
    fn tick(&mut self, t_cycles: usize, reset: usize) -> RaiseInterruptFlag {
        self.internal_timer += t_cycles;
        RaiseInterruptFlag::NoChanges
    }
}

#[derive(Debug)]
pub struct TimaRegister {
    tima_reg: u8,
    last_tima_update: usize,

    tac_reg: u8,
    tma_reg: u8,
}

impl TimaRegister {
    const TAC_DIVIDER_TABLE: [usize; 4] = [1024, 16, 64, 256];
    const TAC_REGISTER_ADDRESS: usize = 0xFF07;
    const TIMA_REGISTER_ADDRESS: usize = 0xFF05;

    fn new() -> Self {
        Self {
            tima_reg: 0x00,
            last_tima_update: 0x00,

            tac_reg: 0x00,
            tma_reg: 0x00,
        }
    }
}

impl TimerTick for TimaRegister {
    fn tick(&mut self, timer: usize, reset: usize) -> RaiseInterruptFlag {
        let enabled = (self.tac_reg >> 2) & 0x01 == 0x01;

        let mut interrupt_requested = false;
        if enabled {
            let div_index = (self.tac_reg & 0x03) as usize;
            let timer_resolution = Self::TAC_DIVIDER_TABLE[div_index];

            let mut elapsed = timer - self.last_tima_update;

            while elapsed >= timer_resolution {
                let (new_value, overflowed) = self.tima_reg.overflowing_add(1);
                self.tima_reg = new_value;

                elapsed -= timer_resolution;

                if overflowed {
                    //TODO: We need to set the if flags here ! in order to fire any
                    //interrupt
                    self.tima_reg = self.tma_reg;
                    //NOTE: Not sure if we should handle the request before we starting to increase
                    //the timer again but let's try it this way first...
                    interrupt_requested = true;
                }

                self.last_tima_update = timer - elapsed;
            }
        }

        if !interrupt_requested {
            return RaiseInterruptFlag::NoChanges;
        }

        RaiseInterruptFlag::CustomTimerOverflowed
    }
}
