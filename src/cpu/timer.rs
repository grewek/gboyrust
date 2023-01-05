use crate::memory::Memory;

#[derive(Debug)]
pub struct Timer {
    div_reg: u8,
    tima_reg: u8,
    reset: Option<usize>,
}

impl Timer {

    const GAMEBOY_BASE_CLOCK:  usize = 4194304; //NOTE: The baseclock of a gameboy is 4.19Mhz
    const GAMEBOY_TIMER_CLOCK: usize = 16384;  //NOTE: The clock of the DIV Register updates

    const TAC_DIVIDER_TABLE: [usize; 4] = [ 1024, 16, 64, 256 ];
    const TAC_REGISTER_ADDRESS: usize = 0xFF04;

    pub fn new() -> Self {
        Self {
            div_reg: 0x00,
            tima_reg: 0x00,
            reset: None,
        }
    }

    pub fn timer_tick(&mut self, m_cycles: usize, memory: &mut Memory) {
        let t_cycles = if let Some(old) = self.reset { (m_cycles - old) * 4 } else { m_cycles * 4 };

        let ticks = t_cycles / (Self::GAMEBOY_BASE_CLOCK/Self::GAMEBOY_TIMER_CLOCK);
        self.div_reg = ticks as u8;
        memory.update_div_register(self.div_reg);

        dbg!(&self);
    }

    pub fn tima_tick(&mut self, m_cycles: usize, memory: &mut Memory) {
        let tac_value = memory.read(Self::TAC_REGISTER_ADDRESS as u16);
        let enabled = (tac_value >> 3) & 0x01 == 0x01;

        if enabled {
            let div_index = (tac_value & 0x03) as usize;
            assert!(div_index < 3); //NOTE: Make sure we are reading the right amount of bits !
            let selected_resolution = Self::TAC_DIVIDER_TABLE[div_index];
            let timer_resolution = Self::GAMEBOY_BASE_CLOCK / selected_resolution;
            let t_cycles = m_cycles * 4;
            let ticks = t_cycles / timer_resolution;

            if ticks > u8::MAX as usize {
                //TODO: Trigger an interrupt
                //TODO: Reset the value of the register to the value in the TMA Register
            } else {
                self.tima_reg = ticks as u8;
                memory.update_tima_register(self.tima_reg);
            }
        }
    }

    pub fn timer_reset(&mut self, m_cycles: usize) {
        self.reset = Some(m_cycles);
        self.div_reg = 0x00;
    }
}
