mod alu;
mod register;
//use register::Register;

use crate::{
    cpu::register::{RegByte, RegWord},
    memory::Memory,
};

use self::{alu::Alu, register::Registers};

pub struct Cpu {
    alu: Alu,
    regs: Registers,
    pub sp: u16,
    pub pc: u16,

    interrupt_flag: bool,
    enable_interrupt: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            alu: Alu::default(),
            regs: Registers::new(),
            sp: 0xFFFE,
            pc: 0x0100,

            interrupt_flag: false,
            enable_interrupt: false,
        }
    }
}

impl Cpu {
    pub fn cycle(&mut self, memory: &mut Memory) {
        let opcode = self.fetch_byte(memory);

        if self.enable_interrupt && !self.interrupt_flag {
            self.interrupt_flag = true;
            self.enable_interrupt = false;
        }

        if opcode == 0xCB {
            let opcode = self.fetch_byte(memory);

            match opcode {
                0x19 => self.opcode_rr(RegByte::C),
                0x1A => self.opcode_rr(RegByte::D),
                0x1B => self.opcode_rr(RegByte::E),
                0x37 => self.opcode_swap_register(RegByte::A),
                0x38 => self.opcode_srl_register(RegByte::B),
                _ => panic!("Error Unknown opcode {:X} with prefix", opcode),
            }
        } else {
            match opcode {
                0x00 => self.opcode_nop(),
                0x01 => self.opcode_ld_word_to_register(memory, RegWord::Bc),
                0x02 => self.opcode_ld_register_to_memory(memory, RegWord::Bc, RegByte::A),
                0x03 => self.opcode_inc16(RegWord::Bc),
                0x04 => self.opcode_inc8(RegByte::B),
                0x05 => self.opcode_dec8(RegByte::B),
                0x06 => self.opcode_ld_byte_to_register(memory, RegByte::B),
                0x07 => self.opcode_rlca(RegByte::A),
                0x08 => self.opcode_ld_sp_to_address(memory),
                0x09 => self.opcode_add_register_word(RegWord::Hl, RegWord::Bc),
                0x10 => self.opcode_stop(),
                0x0A => self.opcode_ld_memory_to_register(memory, RegByte::A, RegWord::Bc),
                0x0B => self.opcode_dec16(RegWord::Bc),
                0x0C => self.opcode_inc8(RegByte::C),
                0x0D => self.opcode_dec8(RegByte::C),
                0x0E => self.opcode_ld_byte_to_register(memory, RegByte::C),
                0x11 => self.opcode_ld_word_to_register(memory, RegWord::De),
                0x12 => self.opcode_ld_register_to_memory(memory, RegWord::De, RegByte::A),
                0x13 => self.opcode_inc16(RegWord::De),
                0x14 => self.opcode_inc8(RegByte::D),
                0x15 => self.opcode_dec8(RegByte::D),
                0x19 => self.opcode_add_register_word(RegWord::Hl, RegWord::De),
                0x18 => self.opcode_jr(memory),
                0x16 => self.opcode_ld_byte_to_register(memory, RegByte::D),
                0x1A => self.opcode_ld_memory_to_register(memory, RegByte::A, RegWord::De),
                0x1B => self.opcode_dec16(RegWord::De),
                0x1C => self.opcode_inc8(RegByte::E),
                0x1D => self.opcode_dec8(RegByte::E),
                0x1E => self.opcode_ld_byte_to_register(memory, RegByte::E),
                0x1F => self.opcode_rr(RegByte::A),
                0x20 => self.opcode_jr_nz(memory),
                0x21 => self.opcode_ld_word_to_register(memory, RegWord::Hl),
                0x22 => self.opcode_ldi_register_to_memory(memory, RegWord::Hl, RegByte::A),
                0x23 => self.opcode_inc16(RegWord::Hl),
                0x24 => self.opcode_inc8(RegByte::H),
                0x25 => self.opcode_dec8(RegByte::H),
                0x26 => self.opcode_ld_byte_to_register(memory, RegByte::H),
                0x28 => self.opcode_jr_z(memory),
                0x29 => self.opcode_add_register_word(RegWord::Hl, RegWord::Hl),
                0x2A => self.opcode_ldi_memory_to_register(memory, RegByte::A, RegWord::Hl),
                0x2B => self.opcode_dec16(RegWord::Hl),
                0x2C => self.opcode_inc8(RegByte::L),
                0x2D => self.opcode_dec8(RegByte::L),
                0x2E => self.opcode_ld_byte_to_register(memory, RegByte::L),
                0x30 => self.opcode_jr_nc(memory),
                0x31 => self.opcode_ld_memory_to_stackptr(memory),
                0x32 => self.opcode_ldd_register_to_memory(memory, RegWord::Hl, RegByte::A),
                0x33 => self.opcode_inc_sp(),
                0x35 => self.opcode_dec_memory(memory, RegWord::Hl),
                0x36 => self.opcode_ld_byte_to_memory(memory, RegWord::Hl),
                0x38 => self.opcode_jr_c(memory),
                0x3A => self.opcode_ldd_memory_to_register(memory, RegByte::A, RegWord::Hl),
                0x3E => self.opcode_ld_byte_to_register(memory, RegByte::A),
                0x3C => self.opcode_inc8(RegByte::A),
                0x3D => self.opcode_dec8(RegByte::A),
                0x40 => self.opcode_ld_register_to_register(RegByte::B, RegByte::B),
                0x41 => self.opcode_ld_register_to_register(RegByte::B, RegByte::C),
                0x42 => self.opcode_ld_register_to_register(RegByte::B, RegByte::D),
                0x43 => self.opcode_ld_register_to_register(RegByte::B, RegByte::E),
                0x44 => self.opcode_ld_register_to_register(RegByte::B, RegByte::H),
                0x45 => self.opcode_ld_register_to_register(RegByte::B, RegByte::L),
                0x46 => self.opcode_ld_memory_to_register(memory, RegByte::B, RegWord::Hl),
                0x47 => self.opcode_ld_register_to_register(RegByte::B, RegByte::A),
                0x48 => self.opcode_ld_register_to_register(RegByte::C, RegByte::B),
                0x49 => self.opcode_ld_register_to_register(RegByte::C, RegByte::C),
                0x4A => self.opcode_ld_register_to_register(RegByte::C, RegByte::D),
                0x4B => self.opcode_ld_register_to_register(RegByte::C, RegByte::E),
                0x4C => self.opcode_ld_register_to_register(RegByte::C, RegByte::H),
                0x4D => self.opcode_ld_register_to_register(RegByte::C, RegByte::L),
                0x4E => self.opcode_ld_memory_to_register(memory, RegByte::C, RegWord::Hl),
                0x4F => self.opcode_ld_register_to_register(RegByte::C, RegByte::A),
                0x50 => self.opcode_ld_register_to_register(RegByte::D, RegByte::B),
                0x51 => self.opcode_ld_register_to_register(RegByte::D, RegByte::C),
                0x52 => self.opcode_ld_register_to_register(RegByte::D, RegByte::D),
                0x53 => self.opcode_ld_register_to_register(RegByte::D, RegByte::E),
                0x54 => self.opcode_ld_register_to_register(RegByte::D, RegByte::H),
                0x55 => self.opcode_ld_register_to_register(RegByte::D, RegByte::L),
                0x56 => self.opcode_ld_memory_to_register(memory, RegByte::D, RegWord::Hl),
                0x57 => self.opcode_ld_register_to_register(RegByte::D, RegByte::A),
                0x58 => self.opcode_ld_register_to_register(RegByte::E, RegByte::B),
                0x59 => self.opcode_ld_register_to_register(RegByte::E, RegByte::C),
                0x5A => self.opcode_ld_register_to_register(RegByte::E, RegByte::D),
                0x5B => self.opcode_ld_register_to_register(RegByte::E, RegByte::E),
                0x5C => self.opcode_ld_register_to_register(RegByte::E, RegByte::H),
                0x5D => self.opcode_ld_register_to_register(RegByte::E, RegByte::L),
                0x5E => self.opcode_ld_memory_to_register(memory, RegByte::E, RegWord::Hl),
                0x5F => self.opcode_ld_register_to_register(RegByte::E, RegByte::A),
                0x60 => self.opcode_ld_register_to_register(RegByte::H, RegByte::B),
                0x61 => self.opcode_ld_register_to_register(RegByte::H, RegByte::C),
                0x62 => self.opcode_ld_register_to_register(RegByte::H, RegByte::D),
                0x63 => self.opcode_ld_register_to_register(RegByte::H, RegByte::E),
                0x64 => self.opcode_ld_register_to_register(RegByte::H, RegByte::H),
                0x65 => self.opcode_ld_register_to_register(RegByte::H, RegByte::L),
                0x66 => self.opcode_ld_memory_to_register(memory, RegByte::H, RegWord::Hl),
                0x67 => self.opcode_ld_register_to_register(RegByte::H, RegByte::A),
                0x68 => self.opcode_ld_register_to_register(RegByte::L, RegByte::B),
                0x69 => self.opcode_ld_register_to_register(RegByte::L, RegByte::C),
                0x6A => self.opcode_ld_register_to_register(RegByte::L, RegByte::D),
                0x6B => self.opcode_ld_register_to_register(RegByte::L, RegByte::E),
                0x6C => self.opcode_ld_register_to_register(RegByte::L, RegByte::H),
                0x6D => self.opcode_ld_register_to_register(RegByte::L, RegByte::L),
                0x6E => self.opcode_ld_memory_to_register(memory, RegByte::L, RegWord::Hl),
                0x6F => self.opcode_ld_register_to_register(RegByte::L, RegByte::A),
                0x70 => self.opcode_ld_register_to_memory(memory, RegWord::Hl, RegByte::B),
                0x71 => self.opcode_ld_register_to_memory(memory, RegWord::Hl, RegByte::C),
                0x72 => self.opcode_ld_register_to_memory(memory, RegWord::Hl, RegByte::D),
                0x73 => self.opcode_ld_register_to_memory(memory, RegWord::Hl, RegByte::E),
                0x74 => self.opcode_ld_register_to_memory(memory, RegWord::Hl, RegByte::H),
                0x75 => self.opcode_ld_register_to_memory(memory, RegWord::Hl, RegByte::L),
                //TODO: 0x76 => HALT
                0x77 => self.opcode_ld_register_to_memory(memory, RegWord::Hl, RegByte::A),
                0x78 => self.opcode_ld_register_to_register(RegByte::A, RegByte::B),
                0x79 => self.opcode_ld_register_to_register(RegByte::A, RegByte::C),
                0x7A => self.opcode_ld_register_to_register(RegByte::A, RegByte::D),
                0x7B => self.opcode_ld_register_to_register(RegByte::A, RegByte::E),
                0x7C => self.opcode_ld_register_to_register(RegByte::A, RegByte::H),
                0x7D => self.opcode_ld_register_to_register(RegByte::A, RegByte::L),
                0x7E => self.opcode_ld_memory_to_register(memory, RegByte::A, RegWord::Hl),
                0x7F => self.opcode_ld_register_to_register(RegByte::A, RegByte::A),
                0x80 => self.opcode_add_register_byte(RegByte::A, RegByte::B),
                0x81 => self.opcode_add_register_byte(RegByte::A, RegByte::C),
                0x82 => self.opcode_add_register_byte(RegByte::A, RegByte::D),
                0x83 => self.opcode_add_register_byte(RegByte::A, RegByte::E),
                0x84 => self.opcode_add_register_byte(RegByte::A, RegByte::H),
                0x85 => self.opcode_add_register_byte(RegByte::A, RegByte::L),
                0x86 => self.opcode_add_memory(memory, RegByte::A, RegWord::Hl),
                0x87 => self.opcode_add_register_byte(RegByte::A, RegByte::A),
                0x88 => self.opcode_adc(RegByte::A, RegByte::B),
                0x89 => self.opcode_adc(RegByte::A, RegByte::C),
                0x8A => self.opcode_adc(RegByte::A, RegByte::D),
                0x8B => self.opcode_adc(RegByte::A, RegByte::E),
                0x8C => self.opcode_adc(RegByte::A, RegByte::H),
                0x8D => self.opcode_adc(RegByte::A, RegByte::L),
                0x8F => self.opcode_adc(RegByte::A, RegByte::A),
                0x90 => self.opcode_sub(RegByte::A, RegByte::B),
                0x91 => self.opcode_sub(RegByte::A, RegByte::C),
                0x92 => self.opcode_sub(RegByte::A, RegByte::D),
                0x93 => self.opcode_sub(RegByte::A, RegByte::E),
                0x94 => self.opcode_sub(RegByte::A, RegByte::H),
                0x95 => self.opcode_sub(RegByte::A, RegByte::L),
                0x97 => self.opcode_sub(RegByte::A, RegByte::A),
                0x98 => self.opcode_sbc(RegByte::A, RegByte::B),
                0x99 => self.opcode_sbc(RegByte::A, RegByte::C),
                0x9A => self.opcode_sbc(RegByte::A, RegByte::D),
                0x9B => self.opcode_sbc(RegByte::A, RegByte::E),
                0x9C => self.opcode_sbc(RegByte::A, RegByte::H),
                0x9D => self.opcode_sbc(RegByte::A, RegByte::L),
                0x9F => self.opcode_sbc(RegByte::A, RegByte::A),
                0xA0 => self.opcode_and(RegByte::A, RegByte::B),
                0xA1 => self.opcode_and(RegByte::A, RegByte::C),
                0xA2 => self.opcode_and(RegByte::A, RegByte::D),
                0xA3 => self.opcode_and(RegByte::A, RegByte::E),
                0xA4 => self.opcode_and(RegByte::A, RegByte::H),
                0xA5 => self.opcode_and(RegByte::A, RegByte::L),
                0xA7 => self.opcode_and(RegByte::A, RegByte::A),
                0xA8 => self.opcode_xor(RegByte::A, RegByte::B),
                0xA9 => self.opcode_xor(RegByte::A, RegByte::C),
                0xAA => self.opcode_xor(RegByte::A, RegByte::D),
                0xAB => self.opcode_xor(RegByte::A, RegByte::E),
                0xAC => self.opcode_xor(RegByte::A, RegByte::H),
                0xAD => self.opcode_xor(RegByte::A, RegByte::L),
                0xAE => self.opcode_xor_memory(memory, RegByte::A, RegWord::Hl),
                0xAF => self.opcode_xor(RegByte::A, RegByte::A),
                0xB0 => self.opcode_or(RegByte::A, RegByte::B),
                0xB1 => self.opcode_or(RegByte::A, RegByte::C),
                0xB2 => self.opcode_or(RegByte::A, RegByte::D),
                0xB3 => self.opcode_or(RegByte::A, RegByte::E),
                0xB4 => self.opcode_or(RegByte::A, RegByte::H),
                0xB5 => self.opcode_or(RegByte::A, RegByte::L),
                0xB6 => self.opcode_or_memory(memory, RegByte::A, RegWord::Hl),
                0xB7 => self.opcode_or(RegByte::A, RegByte::A),
                0xB8 => self.opcode_cp(RegByte::A, RegByte::B),
                0xB9 => self.opcode_cp(RegByte::A, RegByte::C),
                0xBA => self.opcode_cp(RegByte::A, RegByte::D),
                0xBB => self.opcode_cp(RegByte::A, RegByte::E),
                0xBC => self.opcode_cp(RegByte::A, RegByte::H),
                0xBE => self.opcode_cp_memory(memory, RegByte::A, RegWord::Hl),
                0xBD => self.opcode_cp(RegByte::A, RegByte::L),
                0xBF => self.opcode_cp(RegByte::A, RegByte::A),
                0xC0 => self.opcode_ret_nz(memory),
                0xC1 => self.opcode_pop(memory, RegWord::Bc, false),
                0xC3 => self.opcode_jp(memory),
                0xC4 => self.opcode_call_nz(memory),
                0xC5 => self.opcode_push(memory, RegWord::Bc),
                0xC6 => self.opcode_add_byte(memory, RegByte::A),
                0xC8 => self.opcode_ret_z(memory),
                0xC7 => self.opcode_rst(memory, 0x0000),
                0xC9 => self.opcode_ret(memory),
                0xCC => self.opcode_call_z(memory),
                0xCD => self.opcode_call(memory),
                0xCE => self.opcode_adc_byte(memory, RegByte::A),
                0xD0 => self.opcode_ret_nc(memory),
                0xD1 => self.opcode_pop(memory, RegWord::De, false),
                0xD5 => self.opcode_push(memory, RegWord::De),
                0xD6 => self.opcode_sub_byte(memory, RegByte::A),
                0xD8 => self.opcode_ret_c(memory),
                0xDC => self.opcode_call_c(memory),
                0xE0 => self.opcode_ldh_register_to_address(memory, RegByte::A),
                0xE1 => self.opcode_pop(memory, RegWord::Hl, false),
                0xE5 => self.opcode_push(memory, RegWord::Hl),
                0xE6 => self.opcode_and_byte(memory, RegByte::A),
                0xE9 => self.opcode_jp_hl(RegWord::Hl),
                0xEA => self.opcode_ld_register_to_address(memory, RegByte::A),
                0xEE => self.opcode_xor_byte(memory, RegByte::A),
                0xF0 => self.opcode_ldh_address_to_register(memory, RegByte::A),
                0xF1 => self.opcode_pop(memory, RegWord::Af, true),
                0xF3 => self.opcode_di(),
                0xF5 => self.opcode_push(memory, RegWord::Af),
                0xF9 => self.opcode_ld_reg_to_stackptr(RegWord::Hl),
                0xFA => self.opcode_ld_address_to_register(memory, RegByte::A),
                0xFE => self.opcode_cp_byte(memory, RegByte::A),
                0xFF => self.opcode_rst(memory, 0x0038),
                _ => panic!("Please implement the opcode {:X}", opcode),
            }
        }
    }

    fn fetch_word(&mut self, mem: &Memory) -> u16 {
        let lo_byte = mem.read(self.pc);
        self.pc += 1;
        let hi_byte = mem.read(self.pc);
        self.pc += 1;

        (hi_byte as u16) << 8 | lo_byte as u16
    }

    fn fetch_byte(&mut self, mem: &Memory) -> u8 {
        let lo_byte = mem.read(self.pc);
        self.pc = self.pc.overflowing_add(1).0;

        lo_byte
    }

    fn opcode_stop(&mut self) {
        //????? Sorry cpu i have no idea what you want me todo here ... must have something todo with it's amount of cycles
        //i guess...
    }
    fn opcode_nop(&mut self) {}

    fn opcode_di(&mut self) {
        self.enable_interrupt = true;
    }

    fn opcode_inc_sp(&mut self) {
        self.sp = self.sp.overflowing_add(1).0;
    }
    fn opcode_ld_reg_to_stackptr(&mut self, src: RegWord) {
        let value = self.regs.read_value16_from(src);
        self.sp = value;
    }

    fn opcode_rlca(&mut self, dest: RegByte) {
        let a = self.regs.read_value8_from(dest);

        let result = self.alu.rotate_left_8(a);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_cp(&mut self, dest: RegByte, src: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.regs.read_value8_from(src);

        let flags = self.alu.cp_8(a, b);

        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_cp_memory(&mut self, mem: &mut Memory, dest: RegByte, src: RegWord) {
        let a = self.regs.read_value8_from(dest);
        let ptr = self.regs.read_value16_from(src);

        let b = mem.read(ptr);
        let flags = self.alu.cp_8(a, b);

        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_cp_byte(&mut self, mem: &mut Memory, dest: RegByte) {
        let a = self.regs.read_value8_from(dest);

        let b = self.fetch_byte(mem);
        let flags = self.alu.cp_8(a, b);

        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_inc8(&mut self, dest: RegByte) {
        let value = self.regs.read_value8_from(dest);

        let result = self.alu.inc_8(value);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_inc16(&mut self, dest: RegWord) {
        let value = self.regs.read_value16_from(dest);

        let result = self.alu.inc_16(value);

        self.regs.write_value16_to(dest, result);
    }

    fn opcode_dec8(&mut self, dest: RegByte) {
        let value = self.regs.read_value8_from(dest);

        let result = self.alu.dec_8(value);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_dec16(&mut self, dest: RegWord) {
        let value = self.regs.read_value16_from(dest);

        let result = self.alu.dec_16(value);

        self.regs.write_value16_to(dest, result);
    }

    fn opcode_dec_memory(&mut self, mem: &mut Memory, dest: RegWord) {
        let ptr = self.regs.read_value16_from(dest);

        let value = mem.read(ptr);

        let result = self.alu.dec_8(value);
        let flags = self.alu.flags();

        mem.write(ptr, value);

        self.regs.write_value8_to(RegByte::F, flags)
    }

    fn opcode_or(&mut self, dest: RegByte, src: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.regs.read_value8_from(src);

        let result = self.alu.or_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_or_memory(&mut self, mem: &mut Memory, dest: RegByte, src: RegWord) {
        let a = self.regs.read_value8_from(dest);
        let ptr = self.regs.read_value16_from(src);

        let b = mem.read(ptr);

        let result = self.alu.or_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_xor(&mut self, dest: RegByte, src: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.regs.read_value8_from(src);

        let result = self.alu.xor_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_xor_memory(&mut self, mem: &mut Memory, dest: RegByte, src: RegWord) {
        let a = self.regs.read_value8_from(dest);
        let ptr = self.regs.read_value16_from(src);

        let b = mem.read(ptr);

        let result = self.alu.xor_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_xor_byte(&mut self, mem: &mut Memory, dest: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.fetch_byte(mem);

        let result = self.alu.xor_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_and(&mut self, dest: RegByte, src: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.regs.read_value8_from(src);

        let result = self.alu.and_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_and_byte(&mut self, mem: &mut Memory, dest: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.fetch_byte(mem);

        let result = self.alu.and_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_adc(&mut self, dest: RegByte, src: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.regs.read_value8_from(src);

        let result = self.alu.adc_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_adc_byte(&mut self, mem: &mut Memory, dest: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.fetch_byte(mem);

        let result = self.alu.adc_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_add_register_word(&mut self, dest: RegWord, src: RegWord) {
        let a = self.regs.read_value16_from(dest);
        let b = self.regs.read_value16_from(src);

        let result = self.alu.add_16(a, b);
        let flags = self.alu.flags();

        self.regs.write_value16_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags)
    }

    fn opcode_add_register_byte(&mut self, dest: RegByte, src: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.regs.read_value8_from(src);

        let result = self.alu.add_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_add_memory(&mut self, mem: &mut Memory, dest: RegByte, src: RegWord) {
        let a = self.regs.read_value8_from(dest);
        let ptr = self.regs.read_value16_from(src);

        let b = mem.read(ptr);

        let result = self.alu.add_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_add_byte(&mut self, mem: &mut Memory, dest: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.fetch_byte(mem);

        let result = self.alu.add_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_sbc(&mut self, dest: RegByte, src: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.regs.read_value8_from(src);

        let result = self.alu.sbc_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_sub(&mut self, dest: RegByte, src: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.regs.read_value8_from(src);

        let result = self.alu.sub_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_sub_byte(&mut self, mem: &mut Memory, dest: RegByte) {
        let a = self.regs.read_value8_from(dest);
        let b = self.fetch_byte(mem);

        let result = self.alu.sub_8(a, b);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_ld_register_to_memory(&mut self, mem: &mut Memory, dest: RegWord, src: RegByte) {
        let addr = self.regs.read_value16_from(dest);

        let value = self.regs.read_value8_from(src);

        mem.write(addr, value);
    }

    fn opcode_ld_memory_to_register(&mut self, mem: &Memory, dest: RegByte, src: RegWord) {
        let addr = self.regs.read_value16_from(src);

        let value = mem.read(addr);

        self.regs.write_value8_to(dest, value);
    }

    fn opcode_ld_byte_to_register(&mut self, mem: &Memory, dest: RegByte) {
        let byte = self.fetch_byte(mem);

        self.regs.write_value8_to(dest, byte);
    }

    fn opcode_ld_word_to_register(&mut self, mem: &Memory, dest: RegWord) {
        let word = self.fetch_word(mem);

        self.regs.write_value16_to(dest, word);
    }

    fn opcode_ld_byte_to_memory(&mut self, mem: &mut Memory, dest: RegWord) {
        let addr = self.regs.read_value16_from(dest);
        let value = self.fetch_byte(mem);

        mem.write(addr, value);
    }

    fn opcode_ld_register_to_register(&mut self, dest: RegByte, src: RegByte) {
        let src = self.regs.read_value8_from(src);

        self.regs.write_value8_to(dest, src);
    }

    fn opcode_ldi_register_to_memory(&mut self, mem: &mut Memory, dest: RegWord, src: RegByte) {
        let value = self.regs.read_value8_from(src);
        let addr = self.regs.read_value16_from(dest);

        mem.write(addr, value);
        let result = self.alu.inc_16(addr);

        self.regs.write_value16_to(dest, result);
    }

    fn opcode_ldi_memory_to_register(&mut self, mem: &mut Memory, dest: RegByte, src: RegWord) {
        let addr = self.regs.read_value16_from(src);
        let value = mem.read(addr);

        self.regs.write_value8_to(dest, value);
        let result = self.alu.inc_16(addr);

        self.regs.write_value16_to(src, result);
    }

    fn opcode_ldd_register_to_memory(&mut self, mem: &mut Memory, dest: RegWord, src: RegByte) {
        let value = self.regs.read_value8_from(src);
        let addr = self.regs.read_value16_from(dest);

        mem.write(addr, value);
        let result = self.alu.dec_16(addr);

        self.regs.write_value16_to(dest, result);
    }

    fn opcode_ldd_memory_to_register(&mut self, mem: &mut Memory, dest: RegByte, src: RegWord) {
        let addr = self.regs.read_value16_from(src);
        let value = mem.read(addr);

        self.regs.write_value8_to(dest, value);

        let result = self.alu.dec_16(addr);

        self.regs.write_value16_to(src, result);
    }

    fn opcode_ld_register_to_address(&mut self, mem: &mut Memory, src: RegByte) {
        let addr = self.fetch_word(mem);
        let value = self.regs.read_value8_from(src);

        mem.write(addr, value);
    }

    fn opcode_ldh_register_to_address(&mut self, mem: &mut Memory, src: RegByte) {
        let offset = self.fetch_byte(mem);
        let addr = 0xFF00 + offset as u16;
        let value = self.regs.read_value8_from(src);

        mem.write(addr, value);
    }

    fn opcode_ldh_address_to_register(&mut self, mem: &mut Memory, dest: RegByte) {
        let offset = self.fetch_byte(mem);
        let addr = 0xFF00 + offset as u16;

        let value = mem.read(addr);

        self.regs.write_value8_to(dest, value);
    }

    fn opcode_ld_address_to_register(&mut self, mem: &mut Memory, dest: RegByte) {
        let addr = self.fetch_word(mem);

        let value = mem.read(addr);

        self.regs.write_value8_to(dest, value);
    }

    fn opcode_push(&mut self, mem: &mut Memory, src: RegWord) {
        let value: u16 = self.regs.read_value16_from(src);

        let hi_byte = (value >> 8) as u8;
        let lo_byte = value as u8;
        self.sp -= 1;
        mem.write(self.sp, hi_byte);
        self.sp -= 1;
        mem.write(self.sp, lo_byte);
    }

    fn opcode_pop(&mut self, mem: &mut Memory, dest: RegWord, is_flag_register: bool) {
        let lo_byte = mem.read(self.sp);
        self.sp += 1;
        let hi_byte = mem.read(self.sp);
        self.sp += 1;

        let value = ((hi_byte as u16) << 8) | lo_byte as u16;

        if is_flag_register {
            self.alu.restore_flags(lo_byte);
        }
        self.regs.write_value16_to(dest, value);
    }

    fn opcode_jp(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);
        self.pc = addr;
    }

    fn opcode_jp_nz(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);

        if !self.alu.check_zero_flag() {
            self.pc = addr;
        }
    }

    fn opcode_jp_z(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);

        if self.alu.check_zero_flag() {
            self.pc = addr;
        }
    }

    fn opcode_jp_nc(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);

        if !self.alu.check_carry_flag() {
            self.pc = addr;
        }
    }

    fn opcode_jp_c(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);

        if self.alu.check_carry_flag() {
            self.pc = addr;
        }
    }

    fn opcode_jr(&mut self, mem: &mut Memory) {
        let offset = self.fetch_byte(mem) as i8;

        //self.pc += offset as u16;
        self.pc = (self.pc as i16).overflowing_add(offset as i16).0 as u16
    }

    fn opcode_jr_nz(&mut self, mem: &mut Memory) {
        let offset = self.fetch_byte(mem) as i8;

        if !self.alu.check_zero_flag() {
            //self.pc = (self.pc as i16 + offset as i16) as u16;
            self.pc = (self.pc as i16).overflowing_add(offset as i16).0 as u16
        }
    }

    fn opcode_jr_z(&mut self, mem: &mut Memory) {
        let offset = self.fetch_byte(mem) as i8;

        if self.alu.check_zero_flag() {
            //FUFUFUFUFUFUFUFU !!!
            self.pc = (self.pc as i16).overflowing_add(offset as i16).0 as u16;
        }
    }
    fn opcode_jr_nc(&mut self, mem: &mut Memory) {
        let offset = self.fetch_byte(mem) as i8;

        if !self.alu.check_carry_flag() {
            self.pc = (self.pc as i16).overflowing_add(offset as i16).0 as u16;
        }
    }

    fn opcode_jr_c(&mut self, mem: &mut Memory) {
        let offset = self.fetch_byte(mem) as i8;

        if self.alu.check_carry_flag() {
            self.pc = (self.pc as i16).overflowing_add(offset as i16).0 as u16;
        }
    }

    fn opcode_jp_hl(&mut self, src: RegWord) {
        let addr: u16 = self.regs.read_value16_from(src);
        self.pc = addr;
    }

    fn opcode_call_nz(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);

        if !self.alu.check_zero_flag() {
            let lo_byte = self.pc as u8;
            let hi_byte = (self.pc >> 8) as u8;

            self.sp -= 1;
            mem.write(self.sp, hi_byte);
            self.sp -= 1;
            mem.write(self.sp, lo_byte);

            self.pc = addr;
        }
    }

    fn opcode_call_z(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);

        if self.alu.check_zero_flag() {
            let lo_byte = self.pc as u8;
            let hi_byte = (self.pc >> 8) as u8;

            self.sp -= 1;
            mem.write(self.sp, hi_byte);
            self.sp -= 1;
            mem.write(self.sp, lo_byte);

            self.pc = addr;
        }
    }

    fn opcode_call_c(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);

        if self.alu.check_carry_flag() {
            let lo_byte = self.pc as u8;
            let hi_byte = (self.pc >> 8) as u8;

            self.sp -= 1;
            mem.write(self.sp, hi_byte);
            self.sp -= 1;
            mem.write(self.sp, lo_byte);

            self.pc = addr;
        }
    }

    fn opcode_call(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);

        let lo_byte = self.pc as u8;
        let hi_byte = (self.pc >> 8) as u8;

        self.sp -= 1;
        mem.write(self.sp, hi_byte);
        self.sp -= 1;
        mem.write(self.sp, lo_byte);

        self.pc = addr;
    }

    fn opcode_rst(&mut self, mem: &mut Memory, value: u16) {
        let lo_byte = self.pc as u8;
        let hi_byte = (self.pc >> 8) as u8;

        self.sp -= 1;
        mem.write(self.sp, hi_byte);
        self.sp -= 1;
        mem.write(self.sp, lo_byte);

        self.pc = value;
    }

    fn opcode_rr(&mut self, dest: RegByte) {
        let a = self.regs.read_value8_from(dest);

        let result = self.alu.rotate_right_8(a);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }

    fn opcode_ret_nc(&mut self, mem: &mut Memory) {
        if !self.alu.check_carry_flag() {
            let lo_byte = mem.read(self.sp);
            self.sp += 1;
            let hi_byte = mem.read(self.sp);
            self.sp += 1;
            let addr = (hi_byte as u16) << 8 | lo_byte as u16;
            self.pc = addr;
        }
    }

    fn opcode_ret_c(&mut self, mem: &mut Memory) {
        if self.alu.check_carry_flag() {
            let lo_byte = mem.read(self.sp);
            self.sp += 1;
            let hi_byte = mem.read(self.sp);
            self.sp += 1;
            let addr = (hi_byte as u16) << 8 | lo_byte as u16;
            self.pc = addr;
        }
    }

    fn opcode_ret_z(&mut self, mem: &mut Memory) {
        if self.alu.check_zero_flag() {
            let lo_byte = mem.read(self.sp);
            self.sp += 1;
            let hi_byte = mem.read(self.sp);
            self.sp += 1;
            let addr = (hi_byte as u16) << 8 | lo_byte as u16;

            self.pc = addr;
        }
    }

    fn opcode_ret_nz(&mut self, mem: &mut Memory) {
        if !self.alu.check_zero_flag() {
            let lo_byte = mem.read(self.sp);
            self.sp += 1;
            let hi_byte = mem.read(self.sp);
            self.sp += 1;

            let addr = (hi_byte as u16) << 8 | lo_byte as u16;

            self.pc = addr;
        }
    }

    fn opcode_ret(&mut self, mem: &mut Memory) {
        let lo_byte = mem.read(self.sp);
        self.sp += 1;
        let hi_byte = mem.read(self.sp);
        self.sp += 1;

        let addr = (hi_byte as u16) << 8 | lo_byte as u16;

        self.pc = addr;
    }

    fn opcode_ld_memory_to_stackptr(&mut self, mem: &mut Memory) {
        let value = self.fetch_word(mem);

        self.sp = value;
    }

    fn opcode_ld_sp_to_address(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);
        let lo_byte = self.sp as u8;
        let hi_byte = (self.sp >> 8) as u8;

        mem.write(addr, lo_byte);
        mem.write(addr + 1, hi_byte);
    }

    fn opcode_swap_register(&mut self, dest: RegByte) {
        let value: u8 = self.regs.read_value8_from(dest);

        let hi_byte: u8 = value & 0xF0;
        let lo_byte: u8 = value & 0x0F;

        let result = lo_byte << 4 | hi_byte >> 4;

        self.regs.write_value8_to(dest, result);
    }

    fn opcode_srl_register(&mut self, dest: RegByte) {
        let value = self.regs.read_value8_from(dest);

        let result = self.alu.shift_right_8(value);
        let flags = self.alu.flags();

        self.regs.write_value8_to(dest, result);
        self.regs.write_value8_to(RegByte::F, flags);
    }
}
