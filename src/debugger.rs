use std::collections::HashSet;

use crate::{
    cpu::Cpu,
    disassembler::{self, AssemblyDesc},
    memory::Memory,
};
pub struct Debugger {
    cpu: Cpu,
    memory: Memory,
    breakpoints: HashSet<u16>,
}

pub enum DebuggerCommand {
    ToggleBreakpoint(u16),
    StepOver,
    Step,
    Run,
    Ignore,
}

impl Default for Debugger {
    fn default() -> Self {
        Self {
            cpu: Cpu::default(),
            memory: Memory::default(),
            breakpoints: HashSet::new(),
        }
    }
}

impl Debugger {
    pub fn new(memory: Memory) -> Self {
        Self {
            cpu: Cpu::default(),
            memory,
            breakpoints: HashSet::new(),
        }
    }

    pub fn disassemble_pc(&self) -> AssemblyDesc {
        self.disassemble_addr(self.cpu.pc)
    }

    fn disassemble_addr(&self, addr: u16) -> AssemblyDesc {
        let opcode = self.memory.read(addr);
        let arg_lo = self.memory.read(addr + 1);
        let arg_hi = self.memory.read(addr + 2);

        AssemblyDesc::disassemble(addr, opcode, arg_lo, arg_hi)
    }

    pub fn create_dump(&self, addr: u16, size: u8) -> [Option<u8>; 3] {
        match size {
            1 => [Some(self.memory.read(addr)), None, None],
            2 => [
                Some(self.memory.read(addr)),
                Some(self.memory.read(addr + 1)),
                None,
            ],
            3 => [
                Some(self.memory.read(addr)),
                Some(self.memory.read(addr + 1)),
                Some(self.memory.read(addr + 2)),
            ],
            _ => panic!("Opcode size was out of range !"),
        }
    }

    pub fn disassemble_region(&self, height: u16) -> Vec<AssemblyDesc> {
        let mut result = Vec::new();
        let mut read_pos = self.cpu.pc;

        for _ in 0..height {
            let desc = self.disassemble_addr(read_pos);

            read_pos += desc.size as u16;
            result.push(desc);
        }

        result
    }

    pub fn run(&mut self) {
        loop {
            if self.breakpoints.contains(&self.cpu.pc) {
                break;
            }

            self.cpu.cycle(&mut self.memory);
        }
    }

    pub fn step(&mut self) {
        self.cpu.cycle(&mut self.memory);
    }

    pub fn step_over(&mut self) {
        let current = self.disassemble_pc();
        if current.opcode == disassembler::Opcode::Call {
            let bp = self.cpu.pc + 3;

            self.breakpoints.insert(bp);
            self.run();
            self.breakpoints.remove(&bp);
        } else {
            self.step()
        }
    }

    pub fn is_executed_next(&self, offset: u16) -> bool {
        offset == self.cpu.pc
    }

    pub fn get_offset(&self) -> u16 {
        self.cpu.pc
    }

    pub fn get_stack_view(&self, size: usize) -> Vec<String> {
        let mut result = Vec::new();
        let middle = size / 2;
        let start = self.cpu.sp - middle as u16;
        let end;
        if self.cpu.sp.overflowing_add(middle as u16).1 {
            end = self.cpu.sp + (u16::MAX - self.cpu.sp);
        } else {
            end = self.cpu.sp + middle as u16;
        }

        for i in start..=end {
            let byte = self.memory.read(i);

            if i == self.cpu.sp {
                result.push(format!("{:04X}:\t[${:02X}] TOP", i, byte));
            } else {
                result.push(format!("{:04X}:\t[${:02X}]", i, byte));
            }
        }

        result
    }

    pub fn get_cpu_state(&self) -> [String; 6] {
        [
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            /*format!(
                "AF: {:04X} [A: {:02X}, F: {:02X}]",
                self.cpu.regs.read_value16_from(RegisterComplete::Af),
                self.cpu.regs.read_value8_from(RegisterPart::A),
                self.cpu.regs.read_value8_from(RegisterPart::F),
            ),
            format!(
                "BC: {:04X} [B: {:02X}, C: {:02X}]",
                self.cpu.bc.get(),
                self.cpu.bc.get_high(),
                self.cpu.bc.get_low()
            ),
            format!(
                "DE: {:04X} [D: {:02X}, E: {:02X}]",
                self.cpu.de.get(),
                self.cpu.de.get_high(),
                self.cpu.de.get_low()
            ),
            format!(
                "HL: {:04X} [H: {:02X}, L: {:02X}]",
                self.cpu.hl.get(),
                self.cpu.hl.get_high(),
                self.cpu.hl.get_low()
            ),
            format!("SP: {:04X}", self.cpu.sp),
            format!("PC: {:04X}", self.cpu.pc),*/
        ]
    }

    pub fn get_flags(&self) -> String {
        let flags = 0x00;
        //let flags = self.cpu.af.get_low();

        let zero_state = flags >> 7 & 0x01;
        let negative_state = flags >> 6 & 0x01;
        let half_carry_state = flags >> 5 & 0x01;
        let carry_state = flags >> 4 & 0x01;

        let zi = self.get_flag_icon(zero_state);
        let ni = self.get_flag_icon(negative_state);
        let hi = self.get_flag_icon(half_carry_state);
        let ci = self.get_flag_icon(carry_state);

        format!("Z: [{}], N: [{}], H: [{}], C: [{}]", zi, ni, hi, ci)
    }

    fn get_flag_icon(&self, state: u8) -> String {
        if state == 0x01 {
            "1".to_string()
        } else {
            "0".to_string()
        }
    }

    pub fn send_command(&mut self, cmd: DebuggerCommand) {
        match cmd {
            DebuggerCommand::ToggleBreakpoint(addr) => self.set_breakpoint(addr),
            DebuggerCommand::Step => self.step(),
            DebuggerCommand::Run => self.run(),
            DebuggerCommand::StepOver => self.step_over(),
            DebuggerCommand::Ignore => (),
        }
    }

    fn set_breakpoint(&mut self, addr: u16) {
        if self.breakpoints.contains(&addr) {
            self.breakpoints.remove(&addr);
        } else {
            self.breakpoints.insert(addr);
        }
    }

    pub fn is_active_breakpoint(&self, addr: u16) -> bool {
        self.breakpoints.contains(&addr)
    }
}
