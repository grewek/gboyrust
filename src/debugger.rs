use std::{collections::HashMap, fs::File, io::Read};

use egui::Color32;

use crate::{cpu::{Cpu, register::{RegWord, RegByte}}, disassembler::AssemblyDesc, memory::Memory};

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
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::default(),
            memory: Memory::default(),
        }
    }

    pub fn load_cartridge(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).unwrap();
        self.memory.load_cartridge(&buffer);
    }

    pub fn disassemble(&self, disassembly_cache: &mut Vec<(usize, String)>) {
        disassembly_cache.clear();
        let mut start = 0x0101;
        let memory = self.memory.get_mem_slice();

        while start < 0xFFFF {
            let instruction = AssemblyDesc::disassemble(start, memory);
            
            let offset = instruction.offset;
            start += instruction.size as u16;

            let opcode = format!("{}", instruction);
            disassembly_cache.push((offset as usize, opcode));
        }
    }

    pub fn get_register_word(&self, reg: RegWord) -> String {
        format!("{:04X}", self.cpu.regs.read_value16_from(reg))
    }

    pub fn get_register_byte(&self, reg: RegByte) -> String {
        format!("{:02X}", self.cpu.regs.read_value8_from(reg))
    }

    pub fn get_pc_string(&self) -> String {
        format!("{:04X}", self.cpu.pc)
    }

    pub fn get_sp_string(&self) -> String {
        format!("{:04X}", self.cpu.sp)
    }

    pub fn step(&mut self) {
        self.cpu.cycle(&mut self.memory);
    }

    pub fn get_program_counter(&self) -> usize {
        self.cpu.pc as usize
    }
}