pub struct Memory([u8; 0x10000]);

impl Default for Memory {
    fn default() -> Self {
        Memory([0x00; 0x10000])
    }
}

impl Memory {
    pub fn load_cartridge(&mut self, bytes: &[u8]) {
        for (i, byte) in bytes.iter().enumerate() {
            self.0[i] = *byte;
        }
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr == 0xFF02 && value == 0x81 {
            let byte = char::from(self.0[0xFF01]);
            print!("{}", byte);
        }

        self.0[addr as usize] = value;
    }
}
