use crate::memory::Memory;

#[derive(Debug)]
pub struct Timer {
    div_reg: u8,
    div_reset: usize,

    tima_reg: u8,
    last_tima_update: usize,

    tma_reg: u8,
}

impl Timer {
    const GAMEBOY_INTERNAL_TIMER_UPDATE_RATE: usize = 256; //NOTE: Update the internal clock every 256
                                                           //cycles (t-cycles not machine cycles !)

    const TAC_DIVIDER_TABLE: [usize; 4] = [1024, 16, 64, 256];
    const TAC_REGISTER_ADDRESS: usize = 0xFF07;
    const TIMA_REGISTER_ADDRESS: usize = 0xFF05;

    pub fn new() -> Self {
        Self {
            div_reg: 0x00,
            div_reset: 0x00,

            tima_reg: 0x00,
            last_tima_update: 0x00,

            tma_reg: 0x00,
            //last_update: 0x00,
        }
    }

    pub fn timer_tick(&mut self, m_cycles: usize, memory: &mut Memory) {
        let t_cycles = (m_cycles * 4).saturating_sub(self.div_reset);

        let ticks = t_cycles / Self::GAMEBOY_INTERNAL_TIMER_UPDATE_RATE;
        self.div_reg = ticks as u8;

        memory.update_div_register(self.div_reg);
    }

    pub fn tima_tick(&mut self, m_cycles: usize, memory: &mut Memory) {
        //TODO: The interrupt needs to fire if we overflow the value.
        //TODO: And to make that happen we need working interrupts !
        let tac_value = memory.read(Self::TAC_REGISTER_ADDRESS as u16);
        let enabled = (tac_value >> 2) & 0x01 == 0x01;

        if enabled {
            self.tima_reg = memory.read(Self::TIMA_REGISTER_ADDRESS as u16);

            let div_index = (tac_value & 0x03) as usize;
            let timer_resolution = Self::TAC_DIVIDER_TABLE[div_index];

            let t_cycles = m_cycles * 4;
            let t_cycles = t_cycles.saturating_sub(self.div_reset);

            if t_cycles.saturating_sub(self.last_tima_update) >= timer_resolution {
                let (new_value, overflowed) = self.tima_reg.overflowing_add(1);
                self.tima_reg = new_value;

                if overflowed {
                    let if_reg = memory.read(0xFF0F);
                    let if_reg = if_reg | (0x01 << 2);
                    memory.write(0xFF0F, if_reg);

                    self.tima_reg = memory.read(0xFF06);
                }

                self.last_tima_update = t_cycles;
                memory.write(Self::TIMA_REGISTER_ADDRESS as u16, self.tima_reg);
            }
        }
    }

    pub fn timer_reset(&mut self, m_cycles: usize) {
        self.div_reg = 0x00;
        self.last_tima_update = 0x00;
        self.div_reset = m_cycles * 4;
    }

    pub fn set_tma_register(&mut self, value: u8) {
        self.tma_reg = value;
    }
}
