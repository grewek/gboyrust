use std::{collections::HashMap, collections::HashSet, fs::File, io::Read};

use egui::Color32;

use crate::{
    cpu::{
        register::{RegByte, RegWord},
        Cpu,
    },
    disassembler::AssemblyDesc,
    memory::Memory,
};

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
    breakpoints: HashSet<u16>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::default(),
            memory: Memory::default(),
            breakpoints: HashSet::new(),
        }
    }

    pub fn load_cartridge(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).unwrap();
        self.memory.load_cartridge(&buffer);
    }

    fn generate_hexdump(instr: &AssemblyDesc, memory: &[u8]) -> [Option<u8>; 3] {
        let offset = instr.offset as usize;
        match instr.size {
            1 => [Some(memory[offset]), None, None],
            2 => [Some(memory[offset]), Some(memory[offset + 1]), None],
            3 => [
                Some(memory[offset]),
                Some(memory[offset + 1]),
                Some(memory[offset + 2]),
            ],
            _ => panic!("Instruction size out of range!"),
        }
    }
    pub fn disassemble(
        &self,
        disassembly_cache: &mut Vec<([Option<u8>; 3], AssemblyDesc)>,
        disassembly_map: &mut HashMap<u16, usize>,
    ) {
        disassembly_cache.clear();
        let mut start = 0x0000;
        let memory = self.memory.get_mem_slice();

        while start < 0xFFFF {
            let instruction = AssemblyDesc::disassemble(start, memory);

            start += instruction.size as u16;

            disassembly_cache.push((Self::generate_hexdump(&instruction, memory), instruction));
            disassembly_map.insert(instruction.offset, disassembly_cache.len() - 1);
        }
    }

    pub fn stack_data(&self) -> Vec<(bool, u16, u16)> {
        let mut result = Vec::new();

        let lower_stack_bound = self.cpu.sp.saturating_sub(8);
        let upper_stack_bound = self.cpu.sp.saturating_add(8);

        for stackptr in (lower_stack_bound..upper_stack_bound).step_by(2) {
            let low_byte = self.memory.read(stackptr as u16);
            let high_byte = self.memory.read(stackptr + 1 as u16);

            let word = (high_byte as u16) << 8 | low_byte as u16;
            let top = self.cpu.sp == stackptr;

            result.push((top, stackptr, word));
        }

        result
    }

    pub fn toggle_breakpoint(&mut self, offset: u16) {
        if self.is_registered_breakpoint(offset) {
            self.breakpoints.remove(&offset);
        } else {
            self.breakpoints.insert(offset);
        }
    }

    pub fn is_registered_breakpoint(&self, offset: u16) -> bool {
        if self.breakpoints.contains(&offset) {
            true
        } else {
            false
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

    pub fn get_div_timer(&self) -> String {
        format!("{:03}", self.memory.read(0xFF04))
    }

    pub fn get_custom_timer(&self) -> String {
        format!("{:03}", self.memory.read(0xFF05))
    }

    pub fn get_custom_timer_tick_rate(&self) -> String {
        let tac_value = self.memory.read(0xFF07);
        let div_index = (tac_value & 0x03) as usize;

        let current_tick_rate = match div_index {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => panic!("ERROR: Tickrate index was out of range !"),
        };

        format!("{:04}", current_tick_rate)
    }

    pub fn get_timer_reset(&self) -> String {
        format!("{:03}", self.memory.read(0xFF06))
    }
    pub fn run(&mut self) {
        //FIXME: If there is no breakpoint this loop is a infinite one and will
        //       not return to the caller.
        loop {
            if self
                .breakpoints
                .contains(&(self.get_program_counter() as u16))
            {
                break;
            }

            self.cpu.cycle(&mut self.memory);
        }
    }

    pub fn step(&mut self) {
        self.cpu.cycle(&mut self.memory);
    }

    pub fn get_program_counter(&self) -> usize {
        self.cpu.pc as usize
    }

    pub fn get_machine_cycles(&self) -> usize {
        self.cpu.machine_cycles
    }
}
