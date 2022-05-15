use std::{collections::HashMap, fs::File, io::Read};

use crate::{cpu::Cpu, disassembler::AssemblyDesc, memory::Memory};

//NOTE:  Gameboy Memory Map:
//Start     End     Size    Description
//0x0000    0x3FFF  16Kb    Rom Bank 0
//0x4000    0x7FFF  16Kb    Rom Bank 1, switchable
//0x8000    0x9FFF   8Kb    Video Memory
//0xA000    0xBFFF   8Kb    External Memory
//0xC000    0xCFFF   4Kb    Working Ram
//0xD000    0xDFFF   4Kb    Working Ram switchable
//0xE000    0xFDFF   8Kb    Mirror of 0xC000 -> 0xDFFF

pub struct Debugger {
    cpu: Cpu,
    memory: Memory,
    pub disassembly_cache: DisassemblyCache,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::default(),
            memory: Memory::default(),
            disassembly_cache: DisassemblyCache::new(),
        }
    }

    pub fn load_cartridge(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).unwrap();
        self.memory.load_cartridge(&buffer);
    }

    pub fn disassemble(&mut self, region: &MemoryLayout) {
        self.disassembly_cache
            .disassemble_memory(&self.memory, *region);
    }

    pub fn get_disassembly(&mut self, region: &MemoryLayout) -> &Vec<DisassemblyText> {
        if self.disassembly_cache.cache.contains_key(region) {
            return &self.disassembly_cache.cache[region];
        } else {
            self.disassemble(region);

            &self.disassembly_cache.cache[region]
        }
    }

    pub fn get_program_counter(&self) -> usize {
        self.cpu.pc as usize
    }
}

pub struct DisassemblyCache {
    pub cache: HashMap<MemoryLayout, Vec<DisassemblyText>>,
}

impl DisassemblyCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn disassemble_memory(&mut self, memory: &Memory, region: MemoryLayout) {
        if self.cache.contains_key(&region) {
            let entry = self.cache.get_mut(&region).unwrap();

            entry.clear();
            let mem_slice = memory.get_mem_slice();
            let size = region.get_size();
            let start: usize = *size.start();

            *entry = disassemble_block(&mem_slice[size], start);
        } else {
            let mem_slice = memory.get_mem_slice();
            let size = region.get_size();
            let start: usize = *size.start();
            self.cache
                .insert(region, disassemble_block(&mem_slice[size], start));
        }
    }
}

pub struct DisassemblyText {
    pub offset: String,
    pub hexdump: String,
    pub opcode: String,
    pub argument: String,
}

impl DisassemblyText {
    fn new(bytes: &[u8], raw_disassembly: &AssemblyDesc) -> Self {
        Self {
            offset: format!("{:04X}", raw_disassembly.offset),
            hexdump: generate_hexdump(&bytes, raw_disassembly.size),
            opcode: raw_disassembly.opcode.to_string(),
            argument: format!("{} {}", raw_disassembly.dest, raw_disassembly.src),
        }
    }
}

fn generate_hexdump(bytes: &[u8], size: u8) -> String {
    match size {
        1 => format!("{:02X}              ", bytes[0]),
        2 => format!("{:02X} {:02X}       ", bytes[0], bytes[1]),
        3 => format!("{:02X} {:02X} {:02X}", bytes[0], bytes[1], bytes[2]),
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum MemoryLayout {
    RomBankZero,
    RomBankOne,
    VideoMemory,
    ExternalMemory,
    WorkingRamZero,
    WorkingRamOne,
    MirrorRam,
    SpriteAttributeTable,
    SecretMemory,
    IORegisters,
    HighRam,
    InterruptsEnable,
}

impl MemoryLayout {
    fn get_size(&self) -> std::ops::RangeInclusive<usize> {
        match self {
            MemoryLayout::RomBankZero => 0x0000..=0x3FFF,
            MemoryLayout::RomBankOne => 0x4000..=0x7FFF,
            MemoryLayout::VideoMemory => 0x8000..=0x9FFF,
            MemoryLayout::ExternalMemory => 0xA000..=0xBFFF,
            MemoryLayout::WorkingRamZero => 0xC000..=0xCFFF,
            MemoryLayout::WorkingRamOne => 0xD000..=0xDFFF,
            MemoryLayout::MirrorRam => 0xE000..=0xFDFF,
            MemoryLayout::SpriteAttributeTable => 0xFE00..=0xFE9F,
            MemoryLayout::SecretMemory => 0xFEA0..=0xFEFF,
            MemoryLayout::IORegisters => 0xFF00..=0xFF7F,
            MemoryLayout::HighRam => 0xFF80..=0xFFFE,
            MemoryLayout::InterruptsEnable => 0xFFFF..=0xFFFF,
        }
    }
    fn disassemble_area(&self, full_memory: &[u8]) -> Vec<DisassemblyText> {
        let size = self.get_size();
        let start: usize = *size.start();
        disassemble_block(&full_memory[size], start)
    }
}

fn disassemble_block(block: &[u8], start: usize) -> Vec<DisassemblyText> {
    let mut result = Vec::new();
    let end = block.len();
    let mut address: usize = 0x0000;
    while address <= end {
        let block = if address + 3 < end {
            &block[address..address + 3]
        } else if address + 2 < end {
            &block[address..address + 2]
        } else if address + 1 < end {
            &block[address..address + 1]
        } else {
            break;
        };

        let assembly = AssemblyDesc::disassemble(start as u16 + address as u16, block);
        address += assembly.size as usize;

        result.push(DisassemblyText::new(block, &assembly));
    }

    result
}

mod test {
    use super::*;

    #[test]
    fn disassemble_block_faulty_does_not_panic() {
        let block = [];

        let result = super::disassemble_block(&block, 0x0000);

        assert_eq!(result.len(), 0);
    }
    #[test]
    fn disassemble_block_midi_small_seq() {
        let block = [0xCE, 0xFF, 0x00];

        let result = super::disassemble_block(&block, 0x0000);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn disassemble_block_large_small_seq() {
        let block = [0xEA, 0xFF, 0x02, 0x00];

        let result = super::disassemble_block(&block, 0x0000);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn disassemble_block_small_large_seq() {
        let block = [0x00, 0xEA, 0xFF, 0x02];

        let result = super::disassemble_block(&block, 0x0000);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn disassemble_block_small_midi_large_midi_small_seq() {
        let block = [0x00, 0xCE, 0xFF, 0xEA, 0xFF, 0x02, 0xCE, 0xFF, 0x00];

        let result = super::disassemble_block(&block, 0x0000);
        assert_eq!(result.len(), 5);
    }
}
