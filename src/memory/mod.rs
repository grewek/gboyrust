pub struct Memory {
    bytes: [u8; 0x10000],
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            bytes: [0x00; 0x10000],
        }
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

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr == 0xFF02 && value == 0x81 {
            let byte = char::from(self.bytes[0xFF01]);
            print!("{}", byte);
        }

        self.bytes[addr as usize] = value;
    }
}
