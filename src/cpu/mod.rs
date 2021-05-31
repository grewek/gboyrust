mod alu;
mod register;
use register::Register;

use crate::memory::Memory;

use self::{
    alu::Alu,
    register::{RegLable, RegSize, SizedArg},
};

pub struct Cpu {
    alu: Alu,
    pub af: Register,
    pub bc: Register,
    pub de: Register,
    pub hl: Register,
    pub sp: u16,
    pub pc: u16,

    interrupt_flag: bool,
    enable_interrupt: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            alu: Alu::default(),
            af: Register::new(0x0100),
            bc: Register::new(0x0000),
            de: Register::new(0x0000),
            hl: Register::new(0x0000),
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
                0x19 => self.opcode_rr(RegLable::C),
                0x1A => self.opcode_rr(RegLable::D),
                0x1B => self.opcode_rr(RegLable::E),
                0x37 => self.opcode_swap_register(RegLable::A),
                0x38 => self.opcode_srl_register(RegLable::B),
                _ => panic!("Error Unknown opcode {:X} with prefix", opcode),
            }
        } else {
            match opcode {
                0x00 => self.opcode_nop(),
                0x01 => self.opcode_ld_value_to_register(memory, RegLable::Bc),
                0x02 => self.opcode_ld_register_to_memory(memory, RegLable::Bc, RegLable::A),
                0x03 => self.opcode_inc(RegLable::Bc),
                0x04 => self.opcode_inc(RegLable::B),
                0x05 => self.opcode_dec(RegLable::B),
                0x06 => self.opcode_ld_value_to_register(memory, RegLable::B),
                0x07 => self.opcode_rlca(RegLable::A),
                0x08 => self.opcode_ld_sp_to_address(memory),
                0x09 => self.opcode_add(RegLable::Hl, RegLable::Bc),
                0x10 => self.opcode_stop(),
                0x0A => self.opcode_ld_memory_to_register(memory, RegLable::A, RegLable::Bc),
                0x0B => self.opcode_dec(RegLable::Bc),
                0x0C => self.opcode_inc(RegLable::C),
                0x0D => self.opcode_dec(RegLable::C),
                0x0E => self.opcode_ld_value_to_register(memory, RegLable::C),
                0x11 => self.opcode_ld_value_to_register(memory, RegLable::De),
                0x12 => self.opcode_ld_register_to_memory(memory, RegLable::De, RegLable::A),
                0x13 => self.opcode_inc(RegLable::De),
                0x14 => self.opcode_inc(RegLable::D),
                0x15 => self.opcode_dec(RegLable::D),
                0x19 => self.opcode_add(RegLable::Hl, RegLable::De),
                0x18 => self.opcode_jr(memory),
                0x16 => self.opcode_ld_value_to_register(memory, RegLable::D),
                0x1A => self.opcode_ld_memory_to_register(memory, RegLable::A, RegLable::De),
                0x1B => self.opcode_dec(RegLable::De),
                0x1C => self.opcode_inc(RegLable::E),
                0x1D => self.opcode_dec(RegLable::E),
                0x1E => self.opcode_ld_value_to_register(memory, RegLable::E),
                0x1F => self.opcode_rr(RegLable::A),
                0x20 => self.opcode_jr_nz(memory),
                0x21 => self.opcode_ld_value_to_register(memory, RegLable::Hl),
                0x22 => self.opcode_ldi_register_to_memory(memory, RegLable::Hl, RegLable::A),
                0x23 => self.opcode_inc(RegLable::Hl),
                0x24 => self.opcode_inc(RegLable::H),
                0x25 => self.opcode_dec(RegLable::H),
                0x26 => self.opcode_ld_value_to_register(memory, RegLable::H),
                0x28 => self.opcode_jr_z(memory),
                0x29 => self.opcode_add(RegLable::Hl, RegLable::Hl),
                0x2A => self.opcode_ldi_memory_to_register(memory, RegLable::A, RegLable::Hl),
                0x2B => self.opcode_dec(RegLable::Hl),
                0x2C => self.opcode_inc(RegLable::L),
                0x2D => self.opcode_dec(RegLable::L),
                0x2E => self.opcode_ld_value_to_register(memory, RegLable::L),
                0x30 => self.opcode_jr_nc(memory),
                0x31 => self.opcode_ld_memory_to_stackptr(memory),
                0x32 => self.opcode_ldd_register_to_memory(memory, RegLable::Hl, RegLable::A),
                0x33 => self.opcode_inc_sp(),
                0x35 => self.opcode_dec_memory(memory, RegLable::Hl),
                0x36 => self.opcode_ld_byte_to_memory(memory, RegLable::Hl),
                0x38 => self.opcode_jr_c(memory),
                0x3A => self.opcode_ldd_memory_to_register(memory, RegLable::A, RegLable::Hl),
                0x3E => self.opcode_ld_value_to_register(memory, RegLable::A),
                0x3C => self.opcode_inc(RegLable::A),
                0x3D => self.opcode_dec(RegLable::A),
                0x40 => self.opcode_ld_register_to_register(RegLable::B, RegLable::B),
                0x41 => self.opcode_ld_register_to_register(RegLable::B, RegLable::C),
                0x42 => self.opcode_ld_register_to_register(RegLable::B, RegLable::D),
                0x43 => self.opcode_ld_register_to_register(RegLable::B, RegLable::E),
                0x44 => self.opcode_ld_register_to_register(RegLable::B, RegLable::H),
                0x45 => self.opcode_ld_register_to_register(RegLable::B, RegLable::L),
                0x46 => self.opcode_ld_memory_to_register(memory, RegLable::B, RegLable::Hl),
                0x47 => self.opcode_ld_register_to_register(RegLable::B, RegLable::A),
                0x48 => self.opcode_ld_register_to_register(RegLable::C, RegLable::B),
                0x49 => self.opcode_ld_register_to_register(RegLable::C, RegLable::C),
                0x4A => self.opcode_ld_register_to_register(RegLable::C, RegLable::D),
                0x4B => self.opcode_ld_register_to_register(RegLable::C, RegLable::E),
                0x4C => self.opcode_ld_register_to_register(RegLable::C, RegLable::H),
                0x4D => self.opcode_ld_register_to_register(RegLable::C, RegLable::L),
                0x4E => self.opcode_ld_memory_to_register(memory, RegLable::C, RegLable::Hl),
                0x4F => self.opcode_ld_register_to_register(RegLable::C, RegLable::A),
                0x50 => self.opcode_ld_register_to_register(RegLable::D, RegLable::B),
                0x51 => self.opcode_ld_register_to_register(RegLable::D, RegLable::C),
                0x52 => self.opcode_ld_register_to_register(RegLable::D, RegLable::D),
                0x53 => self.opcode_ld_register_to_register(RegLable::D, RegLable::E),
                0x54 => self.opcode_ld_register_to_register(RegLable::D, RegLable::H),
                0x55 => self.opcode_ld_register_to_register(RegLable::D, RegLable::L),
                0x56 => self.opcode_ld_memory_to_register(memory, RegLable::D, RegLable::Hl),
                0x57 => self.opcode_ld_register_to_register(RegLable::D, RegLable::A),
                0x58 => self.opcode_ld_register_to_register(RegLable::E, RegLable::B),
                0x59 => self.opcode_ld_register_to_register(RegLable::E, RegLable::C),
                0x5A => self.opcode_ld_register_to_register(RegLable::E, RegLable::D),
                0x5B => self.opcode_ld_register_to_register(RegLable::E, RegLable::E),
                0x5C => self.opcode_ld_register_to_register(RegLable::E, RegLable::H),
                0x5D => self.opcode_ld_register_to_register(RegLable::E, RegLable::L),
                0x5E => self.opcode_ld_memory_to_register(memory, RegLable::E, RegLable::Hl),
                0x5F => self.opcode_ld_register_to_register(RegLable::E, RegLable::A),
                0x60 => self.opcode_ld_register_to_register(RegLable::H, RegLable::B),
                0x61 => self.opcode_ld_register_to_register(RegLable::H, RegLable::C),
                0x62 => self.opcode_ld_register_to_register(RegLable::H, RegLable::D),
                0x63 => self.opcode_ld_register_to_register(RegLable::H, RegLable::E),
                0x64 => self.opcode_ld_register_to_register(RegLable::H, RegLable::H),
                0x65 => self.opcode_ld_register_to_register(RegLable::H, RegLable::L),
                0x66 => self.opcode_ld_memory_to_register(memory, RegLable::H, RegLable::Hl),
                0x67 => self.opcode_ld_register_to_register(RegLable::H, RegLable::A),
                0x68 => self.opcode_ld_register_to_register(RegLable::L, RegLable::B),
                0x69 => self.opcode_ld_register_to_register(RegLable::L, RegLable::C),
                0x6A => self.opcode_ld_register_to_register(RegLable::L, RegLable::D),
                0x6B => self.opcode_ld_register_to_register(RegLable::L, RegLable::E),
                0x6C => self.opcode_ld_register_to_register(RegLable::L, RegLable::H),
                0x6D => self.opcode_ld_register_to_register(RegLable::L, RegLable::L),
                0x6E => self.opcode_ld_memory_to_register(memory, RegLable::L, RegLable::Hl),
                0x6F => self.opcode_ld_register_to_register(RegLable::L, RegLable::A),
                0x70 => self.opcode_ld_register_to_memory(memory, RegLable::Hl, RegLable::B),
                0x71 => self.opcode_ld_register_to_memory(memory, RegLable::Hl, RegLable::C),
                0x72 => self.opcode_ld_register_to_memory(memory, RegLable::Hl, RegLable::D),
                0x73 => self.opcode_ld_register_to_memory(memory, RegLable::Hl, RegLable::E),
                0x74 => self.opcode_ld_register_to_memory(memory, RegLable::Hl, RegLable::H),
                0x75 => self.opcode_ld_register_to_memory(memory, RegLable::Hl, RegLable::L),
                //TODO: 0x76 => HALT
                0x77 => self.opcode_ld_register_to_memory(memory, RegLable::Hl, RegLable::A),
                0x78 => self.opcode_ld_register_to_register(RegLable::A, RegLable::B),
                0x79 => self.opcode_ld_register_to_register(RegLable::A, RegLable::C),
                0x7A => self.opcode_ld_register_to_register(RegLable::A, RegLable::D),
                0x7B => self.opcode_ld_register_to_register(RegLable::A, RegLable::E),
                0x7C => self.opcode_ld_register_to_register(RegLable::A, RegLable::H),
                0x7D => self.opcode_ld_register_to_register(RegLable::A, RegLable::L),
                0x7E => self.opcode_ld_memory_to_register(memory, RegLable::A, RegLable::Hl),
                0x7F => self.opcode_ld_register_to_register(RegLable::A, RegLable::A),
                0x80 => self.opcode_add(RegLable::A, RegLable::B),
                0x81 => self.opcode_add(RegLable::A, RegLable::C),
                0x82 => self.opcode_add(RegLable::A, RegLable::D),
                0x83 => self.opcode_add(RegLable::A, RegLable::E),
                0x84 => self.opcode_add(RegLable::A, RegLable::H),
                0x85 => self.opcode_add(RegLable::A, RegLable::L),
                0x86 => self.opcode_add_memory(memory, RegLable::A, RegLable::Hl),
                0x87 => self.opcode_add(RegLable::A, RegLable::A),
                0x88 => self.opcode_adc(RegLable::A, RegLable::B),
                0x89 => self.opcode_adc(RegLable::A, RegLable::C),
                0x8A => self.opcode_adc(RegLable::A, RegLable::D),
                0x8B => self.opcode_adc(RegLable::A, RegLable::E),
                0x8C => self.opcode_adc(RegLable::A, RegLable::H),
                0x8D => self.opcode_adc(RegLable::A, RegLable::L),
                0x8F => self.opcode_adc(RegLable::A, RegLable::A),
                0x90 => self.opcode_sub(RegLable::A, RegLable::B),
                0x91 => self.opcode_sub(RegLable::A, RegLable::C),
                0x92 => self.opcode_sub(RegLable::A, RegLable::D),
                0x93 => self.opcode_sub(RegLable::A, RegLable::E),
                0x94 => self.opcode_sub(RegLable::A, RegLable::H),
                0x95 => self.opcode_sub(RegLable::A, RegLable::L),
                0x97 => self.opcode_sub(RegLable::A, RegLable::A),
                0x98 => self.opcode_sbc(RegLable::A, RegLable::B),
                0x99 => self.opcode_sbc(RegLable::A, RegLable::C),
                0x9A => self.opcode_sbc(RegLable::A, RegLable::D),
                0x9B => self.opcode_sbc(RegLable::A, RegLable::E),
                0x9C => self.opcode_sbc(RegLable::A, RegLable::H),
                0x9D => self.opcode_sbc(RegLable::A, RegLable::L),
                0x9F => self.opcode_sbc(RegLable::A, RegLable::A),
                0xA0 => self.opcode_and(RegLable::A, RegLable::B),
                0xA1 => self.opcode_and(RegLable::A, RegLable::C),
                0xA2 => self.opcode_and(RegLable::A, RegLable::D),
                0xA3 => self.opcode_and(RegLable::A, RegLable::E),
                0xA4 => self.opcode_and(RegLable::A, RegLable::H),
                0xA5 => self.opcode_and(RegLable::A, RegLable::L),
                0xA7 => self.opcode_and(RegLable::A, RegLable::A),
                0xA8 => self.opcode_xor(RegLable::A, RegLable::B),
                0xA9 => self.opcode_xor(RegLable::A, RegLable::C),
                0xAA => self.opcode_xor(RegLable::A, RegLable::D),
                0xAB => self.opcode_xor(RegLable::A, RegLable::E),
                0xAC => self.opcode_xor(RegLable::A, RegLable::H),
                0xAD => self.opcode_xor(RegLable::A, RegLable::L),
                0xAE => self.opcode_xor_memory(memory, RegLable::A, RegLable::Hl),
                0xAF => self.opcode_xor(RegLable::A, RegLable::A),
                0xB0 => self.opcode_or(RegLable::A, RegLable::B),
                0xB1 => self.opcode_or(RegLable::A, RegLable::C),
                0xB2 => self.opcode_or(RegLable::A, RegLable::D),
                0xB3 => self.opcode_or(RegLable::A, RegLable::E),
                0xB4 => self.opcode_or(RegLable::A, RegLable::H),
                0xB5 => self.opcode_or(RegLable::A, RegLable::L),
                0xB6 => self.opcode_or_memory(memory, RegLable::A, RegLable::Hl),
                0xB7 => self.opcode_or(RegLable::A, RegLable::A),
                0xB8 => self.opcode_cp(RegLable::A, RegLable::B),
                0xB9 => self.opcode_cp(RegLable::A, RegLable::C),
                0xBA => self.opcode_cp(RegLable::A, RegLable::D),
                0xBB => self.opcode_cp(RegLable::A, RegLable::E),
                0xBC => self.opcode_cp(RegLable::A, RegLable::H),
                0xBE => self.opcode_cp_memory(memory, RegLable::A, RegLable::Hl),
                0xBD => self.opcode_cp(RegLable::A, RegLable::L),
                0xBF => self.opcode_cp(RegLable::A, RegLable::A),
                0xC0 => self.opcode_ret_nz(memory),
                0xC1 => self.opcode_pop(memory, RegLable::Bc, false),
                0xC3 => self.opcode_jp(memory),
                0xC4 => self.opcode_call_nz(memory),
                0xC5 => self.opcode_push(memory, RegLable::Bc),
                0xC6 => self.opcode_add_value(memory, RegLable::A),
                0xC8 => self.opcode_ret_z(memory),
                0xC7 => self.opcode_rst(memory, 0x0000),
                0xC9 => self.opcode_ret(memory),
                0xCC => self.opcode_call_z(memory),
                0xCD => self.opcode_call(memory),
                0xCE => self.opcode_adc_byte(memory, RegLable::A),
                0xD0 => self.opcode_ret_nc(memory),
                0xD1 => self.opcode_pop(memory, RegLable::De, false),
                0xD5 => self.opcode_push(memory, RegLable::De),
                0xD6 => self.opcode_sub_byte(memory, RegLable::A),
                0xD8 => self.opcode_ret_c(memory),
                0xDC => self.opcode_call_c(memory),
                0xE0 => self.opcode_ldh_register_to_address(memory, RegLable::A),
                0xE1 => self.opcode_pop(memory, RegLable::Hl, false),
                0xE5 => self.opcode_push(memory, RegLable::Hl),
                0xE6 => self.opcode_and_byte(memory, RegLable::A),
                0xE9 => self.opcode_jp_hl(RegLable::Hl),
                0xEA => self.opcode_ld_register_to_address(memory, RegLable::A),
                0xEE => self.opcode_xor_byte(memory, RegLable::A),
                0xF0 => self.opcode_ldh_address_to_register(memory, RegLable::A),
                0xF1 => self.opcode_pop(memory, RegLable::Af, true),
                0xF3 => self.opcode_di(),
                0xF5 => self.opcode_push(memory, RegLable::Af),
                0xF9 => self.opcode_ld_reg_to_stackptr(RegLable::Hl),
                0xFA => self.opcode_ld_address_to_register(memory, RegLable::A),
                0xFE => self.opcode_cp_byte(memory, RegLable::A),
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
    fn opcode_ld_reg_to_stackptr(&mut self, src: RegLable) {
        let value = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        self.sp = value;
    }

    fn opcode_rlca(&mut self, dest: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();

        let (result, flags) = self.alu.rotate_left_8(a);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, SizedArg::Size8(result));

        RegLable::Af
            .lable_to_register_writeable(self)
            .write_to_register(&RegLable::F, SizedArg::Size8(flags));
    }

    fn opcode_cp(&mut self, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();
        let b = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        let flags = self.alu.cp_8(a, b);

        RegLable::Af
            .lable_to_register_writeable(self)
            .write_to_register(&RegLable::F, SizedArg::Size8(flags));
    }

    fn opcode_cp_memory(&mut self, mem: &mut Memory, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();
        let ptr = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        let b = mem.read(ptr);

        let flags = self.alu.cp_8(a, b);

        RegLable::Af
            .lable_to_register_writeable(self)
            .write_to_register(&RegLable::F, SizedArg::Size8(flags));
    }

    fn opcode_cp_byte(&mut self, mem: &mut Memory, dest: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();
        let b = self.fetch_byte(mem);

        let flags = self.alu.cp_8(a, b);

        RegLable::Af
            .lable_to_register_writeable(self)
            .write_to_register(&RegLable::F, SizedArg::Size8(flags));
    }

    fn opcode_inc(&mut self, dest: RegLable) {
        let value = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let (new_value, flags) = self.alu.inc_sized_arg(value);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, new_value);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_dec(&mut self, dest: RegLable) {
        let value = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);

        let (new_value, flags) = self.alu.dec_sized_arg(value);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, new_value);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_dec_memory(&mut self, mem: &mut Memory, dest: RegLable) {
        let ptr = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();

        let value = SizedArg::Size8(mem.read(ptr));

        let (new_value, flags) = self.alu.dec_sized_arg(value);

        mem.write(ptr, new_value.into());

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_or(&mut self, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let b = src
            .lable_to_register_readable(self)
            .read_from_register(&src);

        let (result, flags) = self.alu.or_sized_arg(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_or_memory(&mut self, mem: &mut Memory, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let ptr = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        let b = SizedArg::Size8(mem.read(ptr));

        let (result, flags) = self.alu.or_sized_arg(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_xor(&mut self, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let b = src
            .lable_to_register_readable(self)
            .read_from_register(&src);

        let (result, flags) = self.alu.xor_sized_arg(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_xor_memory(&mut self, mem: &mut Memory, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let ptr = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        let b = SizedArg::Size8(mem.read(ptr));

        let (result, flags) = self.alu.xor_sized_arg(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_xor_byte(&mut self, mem: &mut Memory, dest: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let value = SizedArg::Size8(self.fetch_byte(mem));

        let (result, flags) = self.alu.xor_sized_arg(a, value);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_and(&mut self, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let b = src
            .lable_to_register_readable(self)
            .read_from_register(&src);

        let (result, flags) = self.alu.and_sized_arg(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_and_byte(&mut self, mem: &mut Memory, dest: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let b = SizedArg::Size8(self.fetch_byte(mem));

        let (result, flags) = self.alu.and_sized_arg(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_adc(&mut self, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let b = src
            .lable_to_register_readable(self)
            .read_from_register(&src);

        let (result, flags) = self.alu.adc_sized_arg(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_adc_byte(&mut self, mem: &mut Memory, dest: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let b = SizedArg::Size8(self.fetch_byte(mem));

        let (result, flags) = self.alu.adc_sized_arg(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_add(&mut self, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let b = src
            .lable_to_register_readable(self)
            .read_from_register(&src);

        let (result, flags) = self.alu.add_sized_args(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_add_memory(&mut self, mem: &mut Memory, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);

        let ptr = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        let b = mem.read(ptr);

        let (result, flags) = self.alu.add_sized_args(a, SizedArg::Size8(b));

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_add_value(&mut self, mem: &mut Memory, dest: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let b = SizedArg::Size8(self.fetch_byte(mem));

        let (result, flags) = self.alu.add_sized_args(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_sbc(&mut self, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let b = dest
            .lable_to_register_readable(self)
            .read_from_register(&src);

        let (result, flags) = self.alu.sbc_sized_arg(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_sub(&mut self, dest: RegLable, src: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);
        let b = src
            .lable_to_register_readable(self)
            .read_from_register(&src);

        let (result, flags) = self.alu.sub_sized_args(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_sub_byte(&mut self, mem: &mut Memory, dest: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest);

        let b = SizedArg::Size8(self.fetch_byte(mem));

        let (result, flags) = self.alu.sub_sized_args(a, b);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, result);

        if let Some(flags) = flags {
            RegLable::Af
                .lable_to_register_writeable(self)
                .write_to_register(&RegLable::F, SizedArg::Size8(flags));
        }
    }

    fn opcode_ld_register_to_memory(&mut self, mem: &mut Memory, dest: RegLable, src: RegLable) {
        let addr = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();

        let value = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        mem.write(addr, value);
    }

    fn opcode_ld_memory_to_register(&mut self, mem: &Memory, dest: RegLable, src: RegLable) {
        let addr = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        let value = SizedArg::Size8(mem.read(addr));

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, value);
    }

    fn opcode_ld_value_to_register(&mut self, mem: &Memory, dest: RegLable) {
        //let dest_reg = dest.lable_to_register_readable(self);

        let arg = match RegSize::from(&dest) {
            RegSize::Full => SizedArg::Size16(self.fetch_word(mem)),
            RegSize::Half => SizedArg::Size8(self.fetch_byte(mem)),
        };

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, arg)
    }

    fn opcode_ld_byte_to_memory(&mut self, mem: &mut Memory, dest: RegLable) {
        let addr = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();
        let value = self.fetch_byte(mem);

        mem.write(addr, value);
    }

    fn opcode_ld_register_to_register(&mut self, dest: RegLable, src: RegLable) {
        let argument = src
            .lable_to_register_readable(self)
            .read_from_register(&src);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, argument);
    }

    fn opcode_ldi_register_to_memory(&mut self, mem: &mut Memory, dest: RegLable, src: RegLable) {
        let value = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        let addr = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();

        mem.write(addr, value);

        let arg = dest
            .lable_to_register_writeable(self)
            .read_from_register(&dest);
        let (new_arg, _) = self.alu.inc_sized_arg(arg);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, new_arg);
    }

    fn opcode_ldi_memory_to_register(&mut self, mem: &mut Memory, dest: RegLable, src: RegLable) {
        let addr = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        let value = mem.read(addr);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, SizedArg::Size8(value));

        let arg = src
            .lable_to_register_writeable(self)
            .read_from_register(&src);
        let (new_arg, _) = self.alu.inc_sized_arg(arg);

        src.lable_to_register_writeable(self)
            .write_to_register(&src, new_arg);
    }

    fn opcode_ldd_register_to_memory(&mut self, mem: &mut Memory, dest: RegLable, src: RegLable) {
        let value = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        let addr = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();

        mem.write(addr, value);

        let arg = dest
            .lable_to_register_writeable(self)
            .read_from_register(&dest);
        let (new_arg, _) = self.alu.dec_sized_arg(arg);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, new_arg);
    }

    fn opcode_ldd_memory_to_register(&mut self, mem: &mut Memory, dest: RegLable, src: RegLable) {
        let addr = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        let value = mem.read(addr);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, SizedArg::Size8(value));

        let arg = src
            .lable_to_register_writeable(self)
            .read_from_register(&src);
        let (new_arg, _) = self.alu.dec_sized_arg(arg);

        src.lable_to_register_writeable(self)
            .write_to_register(&src, new_arg);
    }

    fn opcode_ld_register_to_address(&mut self, mem: &mut Memory, src: RegLable) {
        let addr = self.fetch_word(mem);
        let value = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        mem.write(addr, value);
    }

    fn opcode_ldh_register_to_address(&mut self, mem: &mut Memory, src: RegLable) {
        let offset = self.fetch_byte(mem);
        let addr = 0xFF00 + offset as u16;
        let value = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        mem.write(addr, value);
    }

    fn opcode_ldh_address_to_register(&mut self, mem: &mut Memory, dest: RegLable) {
        let offset = self.fetch_byte(mem);
        let addr = 0xFF00 + offset as u16;

        let value = SizedArg::Size8(mem.read(addr));

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, value);
    }

    fn opcode_ld_address_to_register(&mut self, mem: &mut Memory, dest: RegLable) {
        let addr = self.fetch_word(mem);

        let value = mem.read(addr);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, SizedArg::Size8(value));
    }

    fn opcode_push(&mut self, mem: &mut Memory, src: RegLable) {
        let value: u16 = src
            .lable_to_register_readable(self)
            .read_from_register(&src)
            .into();

        let hi_byte = (value >> 8) as u8;
        let lo_byte = value as u8;
        self.sp -= 1;
        mem.write(self.sp, hi_byte);
        self.sp -= 1;
        mem.write(self.sp, lo_byte);
    }

    fn opcode_pop(&mut self, mem: &mut Memory, dest: RegLable, is_flag_register: bool) {
        let lo_byte = mem.read(self.sp);
        self.sp += 1;
        let hi_byte = mem.read(self.sp);
        self.sp += 1;

        let arg = SizedArg::Size16(((hi_byte as u16) << 8) | lo_byte as u16);

        if is_flag_register {
            self.alu.restore_flags(lo_byte);
        }
        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, arg);
    }

    fn opcode_jp(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);
        self.pc = addr;
    }

    fn opcode_jp_nz(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if !((flags >> 7) & 0x01) == 0x00 {
            self.pc = addr;
        }
    }

    fn opcode_jp_z(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 7) & 0x01) == 0x01 {
            self.pc = addr;
        }
    }

    fn opcode_jp_nc(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if !((flags >> 4) & 0x01) == 0x00 {
            self.pc = addr;
        }
    }

    fn opcode_jp_c(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 4) & 0x01) == 0x01 {
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
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 7) & 0x01) == 0x00 {
            //self.pc = (self.pc as i16 + offset as i16) as u16;
            self.pc = (self.pc as i16).overflowing_add(offset as i16).0 as u16
        }
    }

    fn opcode_jr_z(&mut self, mem: &mut Memory) {
        let offset = self.fetch_byte(mem) as i8;
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 7) & 0x01) == 0x01 {
            //FUFUFUFUFUFUFUFU !!!
            self.pc = (self.pc as i16).overflowing_add(offset as i16).0 as u16;
        }
    }
    fn opcode_jr_nc(&mut self, mem: &mut Memory) {
        let offset = self.fetch_byte(mem) as i8;
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 4) & 0x01) == 0x00 {
            self.pc = (self.pc as i16).overflowing_add(offset as i16).0 as u16;
        }
    }

    fn opcode_jr_c(&mut self, mem: &mut Memory) {
        let offset = self.fetch_byte(mem) as i8;
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 4) & 0x01) == 0x01 {
            self.pc = (self.pc as i16).overflowing_add(offset as i16).0 as u16;
        }
    }

    fn opcode_jp_hl(&mut self, src: RegLable) {
        let addr: u16 = src
            .lable_to_register_readable(&self)
            .read_from_register(&src)
            .into();

        self.pc = addr;
    }

    fn opcode_call_nz(&mut self, mem: &mut Memory) {
        let addr = self.fetch_word(mem);

        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 7) & 0x01) == 0x00 {
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

        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 7) & 0x01) == 0x01 {
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

        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 4) & 0x01) == 0x01 {
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

    fn opcode_rr(&mut self, dest: RegLable) {
        let a = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();

        let (result, flags) = self.alu.rotate_right_8(a);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, SizedArg::Size8(result));

        RegLable::F
            .lable_to_register_writeable(self)
            .write_to_register(&RegLable::F, SizedArg::Size8(flags));
    }

    fn opcode_ret_nc(&mut self, mem: &mut Memory) {
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 4) & 0x01) == 0x00 {
            let lo_byte = mem.read(self.sp);
            self.sp += 1;
            let hi_byte = mem.read(self.sp);
            self.sp += 1;
            let addr = (hi_byte as u16) << 8 | lo_byte as u16;
            self.pc = addr;
        }
    }

    fn opcode_ret_c(&mut self, mem: &mut Memory) {
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 4) & 0x01) == 0x01 {
            let lo_byte = mem.read(self.sp);
            self.sp += 1;
            let hi_byte = mem.read(self.sp);
            self.sp += 1;
            let addr = (hi_byte as u16) << 8 | lo_byte as u16;
            self.pc = addr;
        }
    }

    fn opcode_ret_z(&mut self, mem: &mut Memory) {
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        // NOTE TO SELF: Next time when i implement conditionals returns make god damn sure to read the return
        // after(!!!!) we checked that we actually jump when we not do this the return address is "popped" from the stack
        // and the next ret in the chain jumps into god knows where...this bull**** has costs me more hours than really
        // necessary...
        if ((flags >> 7) & 0x01) == 0x01 {
            let lo_byte = mem.read(self.sp);
            self.sp += 1;
            let hi_byte = mem.read(self.sp);
            self.sp += 1;
            let addr = (hi_byte as u16) << 8 | lo_byte as u16;

            self.pc = addr;
        }
    }

    fn opcode_ret_nz(&mut self, mem: &mut Memory) {
        let flags: u8 = self.af.read_from_register(&RegLable::F).into();

        if ((flags >> 7) & 0x01) == 0x00 {
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

    fn opcode_swap_register(&mut self, dest: RegLable) {
        let value: u8 = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();

        let hi_byte: u8 = value & 0xF0;
        let lo_byte: u8 = value & 0x0F;

        let new = lo_byte << 4 | hi_byte >> 4;

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, SizedArg::Size8(new));
    }

    fn opcode_srl_register(&mut self, dest: RegLable) {
        let value = dest
            .lable_to_register_readable(self)
            .read_from_register(&dest)
            .into();

        let (result, flags) = self.alu.shift_right_8(value);

        dest.lable_to_register_writeable(self)
            .write_to_register(&dest, SizedArg::Size8(result));

        RegLable::Af
            .lable_to_register_writeable(self)
            .write_to_register(&RegLable::F, SizedArg::Size8(flags));
    }
}
