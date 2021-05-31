use std::fmt::Display;

pub struct AssemblyDesc {
    pub offset: u16,
    pub opcode: Opcode,
    pub dest: Argument,
    pub src: Argument,
    pub size: u8,
}

impl AssemblyDesc {
    fn inc_register(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Inc,
            dest: Argument::R(dest),
            src: Argument::Unused,
            size: 1,
        }
    }

    fn inc_memory(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Inc,
            dest: Argument::IndexedBy(dest),
            src: Argument::Unused,
            size: 1,
        }
    }

    fn dec_register(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Dec,
            dest: Argument::R(dest),
            src: Argument::Unused,
            size: 1,
        }
    }

    fn dec_memory(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Dec,
            dest: Argument::IndexedBy(dest),
            src: Argument::Unused,
            size: 1,
        }
    }

    fn jump_relative(offset: u16, flag: Flag, dest: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Jr,
            dest: if flag == Flag::None {
                Argument::Unused
            } else {
                Argument::Condition(flag)
            },
            src: Argument::Data8(dest),
            size: 2,
        }
    }

    fn jump(offset: u16, flag: Flag, dest: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Jp,
            //TODO: Can we simplify this logic so we don't even need to use a if here?
            dest: if flag == Flag::None {
                Argument::Unused
            } else {
                Argument::Condition(flag)
            },
            src: Argument::Data16(dest),
            size: 3,
        }
    }

    fn jump_register(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Jp,
            dest: Argument::Unused,
            src: Argument::IndexedBy(dest),
            size: 1,
        }
    }

    fn call(offset: u16, flag: Flag, dest: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Call,
            dest: if flag == Flag::None {
                Argument::Unused
            } else {
                Argument::Condition(flag)
            },
            src: Argument::Data16(dest),
            size: 3,
        }
    }

    fn ret(offset: u16, flag: Flag) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Ret,
            dest: if flag == Flag::None {
                Argument::Unused
            } else {
                Argument::Condition(flag)
            },
            src: Argument::Unused,
            size: 1,
        }
    }

    fn load_word_to_register(offset: u16, dest: Register, src: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::R(dest),
            src: Argument::Data16(src),
            size: 3,
        }
    }
    fn load_byte_to_register(offset: u16, dest: Register, src: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::R(dest),
            src: Argument::Data8(src),
            size: 2,
        }
    }

    fn load_byte_to_memory(offset: u16, dest: Register, src: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::IndexedBy(dest),
            src: Argument::Data8(src),
            size: 2,
        }
    }

    fn load_byte_to_address(offset: u16, dest: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::Address(dest),
            src: Argument::R(src),
            size: 3,
        }
    }

    fn load_memory_to_register(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::R(dest),
            src: Argument::IndexedBy(src),
            size: 1,
        }
    }

    fn load_register_to_memory_addr(offset: u16, dest: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::Address(dest),
            src: Argument::R(src),
            size: 3,
        }
    }

    fn load_register_to_memory_inc(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::IncRegister(dest),
            src: Argument::R(src),
            size: 1,
        }
    }

    fn load_register_to_memory_dec(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::DecRegister(dest),
            src: Argument::R(src),
            size: 1,
        }
    }

    fn load_memory_to_register_inc(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::R(dest),
            src: Argument::IncRegister(src),
            size: 1,
        }
    }

    fn load_memory_to_register_dec(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::R(dest),
            src: Argument::DecRegister(src),
            size: 1,
        }
    }

    fn load_register_to_memory(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::IndexedBy(dest),
            src: Argument::R(src),
            size: 1,
        }
    }

    fn load_register_to_register(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::R(dest),
            src: Argument::R(src),
            size: 1,
        }
    }

    fn load_register_to_offset(offset: u16, dest: u8, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::Offset(dest),
            src: Argument::R(src),
            size: 2,
        }
    }

    fn load_offset_to_register(offset: u16, dest: Register, src: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::R(dest),
            src: Argument::Address(src),
            size: 2,
        }
    }

    fn load_register_p_offset_to_register(offset: u16, dest: Register, disp: i8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::R(dest),
            src: Argument::SOffset(disp),
            size: 2,
        }
    }

    fn load_address_to_register(offset: u16, dest: Register, src: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Load,
            dest: Argument::R(dest),
            src: Argument::Address(src),
            size: 3,
        }
    }

    fn and_register(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::And,
            dest: Argument::Unused,
            src: Argument::R(src),
            size: 1,
        }
    }

    fn and_memory(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::And,
            dest: Argument::Unused,
            src: Argument::IndexedBy(src),
            size: 1,
        }
    }

    fn and_byte(offset: u16, src: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::And,
            dest: Argument::Unused,
            src: Argument::Data8(src),
            size: 2,
        }
    }

    fn xor_register(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Xor,
            dest: Argument::Unused,
            src: Argument::R(src),
            size: 1,
        }
    }

    fn xor_memory(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Xor,
            dest: Argument::Unused,
            src: Argument::IndexedBy(src),
            size: 1,
        }
    }

    fn xor_byte(offset: u16, src: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Xor,
            dest: Argument::Unused,
            src: Argument::Data8(src),
            size: 2,
        }
    }

    fn or_register(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Or,
            dest: Argument::Unused,
            src: Argument::R(src),
            size: 1,
        }
    }

    fn or_memory(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Or,
            dest: Argument::Unused,
            src: Argument::IndexedBy(src),
            size: 1,
        }
    }

    fn or_byte(offset: u16, src: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Or,
            dest: Argument::Unused,
            src: Argument::Data8(src),
            size: 2,
        }
    }

    fn cp_register(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Cp,
            dest: Argument::Unused,
            src: Argument::R(src),
            size: 1,
        }
    }

    fn cp_memory(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Cp,
            dest: Argument::Unused,
            src: Argument::IndexedBy(src),
            size: 1,
        }
    }

    fn cp_byte(offset: u16, src: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Cp,
            dest: Argument::Unused,
            src: Argument::Data8(src),
            size: 2,
        }
    }

    fn add_register_to_register(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Add,
            dest: Argument::R(dest),
            src: Argument::R(src),
            size: 1,
        }
    }

    fn add_memory_to_register(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Add,
            dest: Argument::R(dest),
            src: Argument::IndexedBy(src),
            size: 1,
        }
    }

    fn add_byte_to_register(offset: u16, dest: Register, src: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Add,
            dest: Argument::R(dest),
            src: Argument::Data8(src),
            size: 2,
        }
    }

    fn adc_memory_to_register(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Adc,
            dest: Argument::R(dest),
            src: Argument::IndexedBy(src),
            size: 1,
        }
    }

    fn adc_register_to_register(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Adc,
            dest: Argument::R(dest),
            src: Argument::R(src),
            size: 1,
        }
    }

    fn adc_byte_to_register(offset: u16, dest: Register, src: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Adc,
            dest: Argument::R(dest),
            src: Argument::Data8(src),
            size: 2,
        }
    }

    fn sub_register(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Sub,
            dest: Argument::Unused,
            src: Argument::R(src),
            size: 1,
        }
    }

    fn sub_memory(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Sub,
            dest: Argument::Unused,
            src: Argument::IndexedBy(src),
            size: 1,
        }
    }

    fn sub_byte(offset: u16, src: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Sub,
            dest: Argument::Unused,
            src: Argument::Data8(src),
            size: 2,
        }
    }

    fn sbc_register_to_register(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Sbc,
            dest: Argument::R(dest),
            src: Argument::R(src),
            size: 1,
        }
    }

    fn sbc_memory_to_register(offset: u16, dest: Register, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Sbc,
            dest: Argument::R(dest),
            src: Argument::IndexedBy(src),
            size: 1,
        }
    }

    fn sbc_byte_to_register(offset: u16, dest: Register, src: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Sbc,
            dest: Argument::R(dest),
            src: Argument::Data8(src),
            size: 2,
        }
    }

    fn push_register(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Push,
            dest: Argument::Unused,
            src: Argument::R(src),
            size: 1,
        }
    }

    fn pop_register(offset: u16, src: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Pop,
            dest: Argument::Unused,
            src: Argument::R(src),
            size: 1,
        }
    }

    fn rst_byte(offset: u16, idx: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rst,
            dest: Argument::Unused,
            src: Argument::Data8(idx),
            size: 1,
        }
    }

    fn nop(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Nop,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn rlca(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rlca,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn rrca(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rrca,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn stop(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Stop,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn rla(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rla,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn rra(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rra,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn daa(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Daa,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn cpl(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Cpl,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn scf(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Scf,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn ccf(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Ccf,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn halt(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Halt,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn reti(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Reti,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn di(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Di,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn ei(offset: u16) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Ei,
            dest: Argument::Unused,
            src: Argument::Unused,
            size: 1,
        }
    }

    fn data_byte(offset: u16, src: u8) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Byte,
            dest: Argument::Unused,
            src: Argument::Data8(src),
            size: 1,
        }
    }

    fn rotate_register_left(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rlc,
            dest: Argument::R(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn rotate_memory_left(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rlc,
            dest: Argument::IndexedBy(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn rotate_register_right(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rrc,
            dest: Argument::R(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn rotate_memory_right(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rrc,
            dest: Argument::IndexedBy(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn rotate_left_carry_register(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rl,
            dest: Argument::R(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn rotate_left_carry_memory(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rl,
            dest: Argument::IndexedBy(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn rotate_right_carry_register(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rr,
            dest: Argument::R(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn rotate_right_carry_memory(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Rr,
            dest: Argument::IndexedBy(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn shift_left_arithmetic_register(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Sla,
            dest: Argument::R(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn shift_left_arithmetic_memory(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Sla,
            dest: Argument::IndexedBy(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn shift_right_arithmetic_register(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Sra,
            dest: Argument::R(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn shift_right_arithmetic_memory(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Sra,
            dest: Argument::IndexedBy(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn swap_register(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Swap,
            dest: Argument::R(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn swap_memory(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Swap,
            dest: Argument::IndexedBy(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn shift_right_logic_register(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Srl,
            dest: Argument::R(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    fn shift_right_logic_memory(offset: u16, dest: Register) -> AssemblyDesc {
        AssemblyDesc {
            offset,
            opcode: Opcode::Srl,
            dest: Argument::IndexedBy(dest),
            src: Argument::Unused,
            size: 2,
        }
    }

    //TODO: I could collapse the set/clear/check bit functions into one by factoring the opcode out as
    //      a argument this would shorten all these functions into two, one for memory and one for registers...
    fn bit_operation_register(
        offset: u16,
        opcode: Opcode,
        dest: Register,
        bit_index: u8,
    ) -> AssemblyDesc {
        match opcode {
            Opcode::Bit | Opcode::Res | Opcode::Set => AssemblyDesc {
                offset,
                opcode,
                dest: Argument::Bit(bit_index),
                src: Argument::R(dest),
                size: 2,
            },

            _ => panic!("The given opcode {} does not work on a bit level !", opcode),
        }
    }

    fn bit_operation_memory(
        offset: u16,
        opcode: Opcode,
        dest: Register,
        bit_index: u8,
    ) -> AssemblyDesc {
        match opcode {
            Opcode::Bit | Opcode::Res | Opcode::Set => AssemblyDesc {
                offset,
                opcode,
                dest: Argument::Bit(bit_index),
                src: Argument::IndexedBy(dest),
                size: 2,
            },

            _ => panic!("The given opcode {} does not work on a bit level !", opcode),
        }
    }

    pub fn disassemble(offset: u16, opcode: u8, arg_lo: u8, arg_hi: u8) -> AssemblyDesc {
        if opcode != 0xCB {
            AssemblyDesc::opcode_table_no_prefix(offset, opcode, arg_lo, arg_hi)
        } else {
            AssemblyDesc::opcode_table_prefix(offset, arg_lo)
        }
    }

    fn opcode_table_prefix(offset: u16, opcode: u8) -> AssemblyDesc {
        match opcode {
            0x00..=0x05 | 0x07 => AssemblyDesc::rotate_register_left(
                offset,
                Register::decode_8bit_src_register(opcode),
            ),
            0x06 => AssemblyDesc::rotate_memory_left(offset, Register::Hl),
            0x08..=0x0D | 0x0F => AssemblyDesc::rotate_register_right(
                offset,
                Register::decode_8bit_src_register(opcode),
            ),
            0x0E => AssemblyDesc::rotate_memory_right(offset, Register::Hl),
            0x10..=0x15 | 0x17 => AssemblyDesc::rotate_left_carry_register(
                offset,
                Register::decode_8bit_src_register(opcode),
            ),
            0x16 => AssemblyDesc::rotate_left_carry_memory(offset, Register::Hl),
            0x18..=0x1D | 0x1F => AssemblyDesc::rotate_right_carry_register(
                offset,
                Register::decode_8bit_src_register(opcode),
            ),
            0x1E => AssemblyDesc::rotate_right_carry_memory(offset, Register::Hl),
            0x20..=0x25 | 0x27 => AssemblyDesc::shift_left_arithmetic_register(
                offset,
                Register::decode_8bit_src_register(opcode),
            ),
            0x26 => AssemblyDesc::shift_left_arithmetic_memory(offset, Register::Hl),
            0x28..=0x2D | 0x2F => AssemblyDesc::shift_right_arithmetic_register(
                offset,
                Register::decode_8bit_src_register(opcode),
            ),
            0x2E => AssemblyDesc::shift_right_arithmetic_memory(offset, Register::Hl),
            0x30..=0x35 | 0x37 => {
                AssemblyDesc::swap_register(offset, Register::decode_8bit_src_register(opcode))
            }
            0x36 => AssemblyDesc::swap_memory(offset, Register::Hl),
            0x38..=0x3D | 0x3F => AssemblyDesc::shift_right_logic_register(
                offset,
                Register::decode_8bit_src_register(opcode),
            ),
            0x3E => AssemblyDesc::shift_right_logic_memory(offset, Register::Hl),
            0x40..=0x45
            | 0x47..=0x4D
            | 0x4F..=0x55
            | 0x57..=0x5D
            | 0x5F..=0x65
            | 0x67..=0x6D
            | 0x6F..=0x75
            | 0x77..=0x7D
            | 0x7F => AssemblyDesc::bit_operation_register(
                offset,
                Opcode::Bit,
                Register::decode_8bit_src_register(opcode),
                opcode >> 3 & 0x07,
            ),
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x76 | 0x7E => {
                AssemblyDesc::bit_operation_memory(
                    offset,
                    Opcode::Bit,
                    Register::Hl,
                    opcode >> 3 & 0x07,
                )
            }
            0x80..=0x85
            | 0x87..=0x8D
            | 0x8F..=0x95
            | 0x97..=0x9D
            | 0x9F..=0xA5
            | 0xA7..=0xAD
            | 0xAF..=0xB5
            | 0xB7..=0xBD
            | 0xBF => AssemblyDesc::bit_operation_register(
                offset,
                Opcode::Res,
                Register::decode_8bit_src_register(opcode),
                opcode >> 3 & 0x07,
            ),
            0x86 | 0x8E | 0x96 | 0x9E | 0xA6 | 0xAE | 0xB6 | 0xBE => {
                AssemblyDesc::bit_operation_memory(
                    offset,
                    Opcode::Rst,
                    Register::Hl,
                    opcode >> 3 & 0x07,
                )
            }

            0xC0..=0xC5
            | 0xC7..=0xCD
            | 0xCF..=0xD5
            | 0xD7..=0xDD
            | 0xDF..=0xE5
            | 0xE7..=0xED
            | 0xEF..=0xF5
            | 0xF7..=0xFD
            | 0xFF => AssemblyDesc::bit_operation_register(
                offset,
                Opcode::Set,
                Register::decode_8bit_src_register(opcode),
                opcode >> 3 & 0x07,
            ),
            0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE => {
                AssemblyDesc::bit_operation_memory(
                    offset,
                    Opcode::Set,
                    Register::Hl,
                    opcode >> 3 & 0x07,
                )
            }
        }
    }

    fn opcode_table_no_prefix(offset: u16, opcode: u8, arg_lo: u8, arg_hi: u8) -> AssemblyDesc {
        let a = (arg_hi as u16) << 8 | arg_lo as u16;
        match opcode {
            0x00 => AssemblyDesc::nop(offset),
            0x01 | 0x11 | 0x21 | 0x31 => AssemblyDesc::load_word_to_register(
                offset,
                Register::decode_16bit_register(opcode),
                a,
            ),
            0x02 => AssemblyDesc::load_register_to_memory(offset, Register::Bc, Register::A),
            0x03 | 0x13 | 0x23 | 0x33 => {
                AssemblyDesc::inc_register(offset, Register::decode_16bit_register(opcode))
            }
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x3C => {
                AssemblyDesc::inc_register(offset, Register::decode_8bit_dest_register(opcode))
            }
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x3D => {
                AssemblyDesc::dec_register(offset, Register::decode_8bit_dest_register(opcode))
            }
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E => AssemblyDesc::load_byte_to_register(
                offset,
                Register::decode_8bit_dest_register(opcode),
                arg_lo,
            ),
            0x07 => AssemblyDesc::rlca(offset),
            0x08 => AssemblyDesc::load_register_to_memory_addr(offset, a, Register::Sp),
            0x09 | 0x19 | 0x29 | 0x39 => AssemblyDesc::add_register_to_register(
                offset,
                Register::Hl,
                Register::decode_16bit_register(opcode),
            ),
            0x0A => AssemblyDesc::load_memory_to_register(offset, Register::A, Register::Bc),
            0x0B | 0x1B | 0x2B | 0x3B => {
                AssemblyDesc::dec_register(offset, Register::decode_16bit_register(opcode))
            }
            0x0F => AssemblyDesc::rrca(offset),
            0x10 => AssemblyDesc::stop(offset),
            0x12 => AssemblyDesc::load_register_to_memory(offset, Register::De, Register::A),
            0x17 => AssemblyDesc::rla(offset),
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 => {
                AssemblyDesc::jump_relative(offset, Flag::decode_conditional(opcode), arg_lo)
            }
            0x1A => AssemblyDesc::load_memory_to_register(offset, Register::A, Register::De),
            0x1F => AssemblyDesc::rra(offset),
            0x22 => AssemblyDesc::load_register_to_memory_inc(offset, Register::Hl, Register::A),
            0x27 => AssemblyDesc::daa(offset),
            0x2A => AssemblyDesc::load_memory_to_register_inc(offset, Register::A, Register::Hl),
            0x2F => AssemblyDesc::cpl(offset),
            0x32 => AssemblyDesc::load_register_to_memory_dec(offset, Register::Hl, Register::A),
            0x34 => AssemblyDesc::inc_memory(offset, Register::Hl),
            0x35 => AssemblyDesc::dec_memory(offset, Register::Hl),
            0x36 => AssemblyDesc::load_byte_to_memory(offset, Register::Hl, arg_lo),
            0x37 => AssemblyDesc::scf(offset),
            0x3A => AssemblyDesc::load_memory_to_register_dec(offset, Register::A, Register::Hl),
            0x3F => AssemblyDesc::ccf(offset),
            0x40..=0x45
            | 0x47..=0x4D
            | 0x4F..=0x55
            | 0x57..=0x5D
            | 0x5F..=0x65
            | 0x67..=0x6D
            | 0x6F
            | 0x78..=0x7D
            | 0x7F
            | 0xF9 => AssemblyDesc::load_register_to_register(
                offset,
                Register::decode_8bit_dest_register(opcode),
                Register::decode_8bit_src_register(opcode),
            ),
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => {
                AssemblyDesc::load_memory_to_register(
                    offset,
                    Register::decode_8bit_dest_register(opcode),
                    Register::decode_8bit_src_register(opcode),
                )
            }
            0x70..=0x75 | 0x77 => AssemblyDesc::load_register_to_memory(
                offset,
                Register::Hl,
                Register::decode_8bit_src_register(opcode),
            ),
            0x76 => AssemblyDesc::halt(offset),
            0x80..=0x85 | 0x87 => AssemblyDesc::add_register_to_register(
                offset,
                Register::A,
                Register::decode_8bit_src_register(opcode),
            ),
            0x86 => AssemblyDesc::add_memory_to_register(offset, Register::A, Register::Hl),
            0x88..=0x8D | 0x8F => AssemblyDesc::adc_register_to_register(
                offset,
                Register::A,
                Register::decode_8bit_src_register(opcode),
            ),
            0x8E => AssemblyDesc::adc_memory_to_register(offset, Register::A, Register::Hl),
            0x90..=0x95 | 0x97 => {
                AssemblyDesc::sub_register(offset, Register::decode_8bit_src_register(opcode))
            }
            0x96 => AssemblyDesc::sub_memory(offset, Register::Hl),
            0x98..=0x9D | 0x9F => AssemblyDesc::sbc_register_to_register(
                offset,
                Register::A,
                Register::decode_8bit_src_register(opcode),
            ),
            0x9E => AssemblyDesc::sbc_memory_to_register(offset, Register::A, Register::Hl),
            0xA0..=0xA5 | 0xA7 => {
                AssemblyDesc::and_register(offset, Register::decode_8bit_src_register(opcode))
            }
            0xA6 => AssemblyDesc::and_memory(offset, Register::Hl),
            0xA8..=0xAD | 0xAF => {
                AssemblyDesc::xor_register(offset, Register::decode_8bit_src_register(opcode))
            }
            0xAE => AssemblyDesc::xor_memory(offset, Register::Hl),
            0xB0..=0xB5 | 0xB7 => {
                AssemblyDesc::or_register(offset, Register::decode_8bit_src_register(opcode))
            }
            0xB6 => AssemblyDesc::or_memory(offset, Register::Hl),
            0xB8..=0xBD | 0xBF => {
                AssemblyDesc::cp_register(offset, Register::decode_8bit_src_register(opcode))
            }
            0xBE => AssemblyDesc::cp_memory(offset, Register::Hl),
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                AssemblyDesc::pop_register(offset, Register::decode_16bit_register(opcode))
            }
            0xC2 | 0xC3 | 0xCA | 0xD2 | 0xDA => {
                AssemblyDesc::jump(offset, Flag::decode_conditional(opcode), a)
            }
            0xC4 | 0xD4 | 0xCC | 0xDC | 0xCD => {
                AssemblyDesc::call(offset, Flag::decode_conditional(opcode), a)
            }
            0xC5 | 0xD5 | 0xE5 | 0xF5 => {
                AssemblyDesc::push_register(offset, Register::decode_16bit_register(opcode))
            }
            0xC6 => AssemblyDesc::add_byte_to_register(offset, Register::A, arg_lo),
            0xC7 => AssemblyDesc::rst_byte(offset, 0x00),
            0xC0 | 0xC8 | 0xC9 | 0xD0 | 0xD8 => {
                AssemblyDesc::ret(offset, Flag::decode_conditional(opcode))
            }
            0xCB => AssemblyDesc::data_byte(offset, opcode),
            0xCE => AssemblyDesc::adc_byte_to_register(offset, Register::A, arg_lo),
            0xCF => AssemblyDesc::rst_byte(offset, 0x08),
            0xD3 => AssemblyDesc::data_byte(offset, opcode),
            0xD6 => AssemblyDesc::sub_byte(offset, arg_lo),
            0xD7 => AssemblyDesc::rst_byte(offset, 0x10),
            0xD9 => AssemblyDesc::reti(offset),
            0xDB => AssemblyDesc::data_byte(offset, opcode),
            0xDD => AssemblyDesc::data_byte(offset, opcode),
            0xDE => AssemblyDesc::sbc_byte_to_register(offset, Register::A, arg_lo),
            0xDF => AssemblyDesc::rst_byte(offset, 0x18),
            0xE0 => AssemblyDesc::load_register_to_offset(offset, arg_lo as u8, Register::A),
            0xE2 => AssemblyDesc::load_register_to_memory(offset, Register::C, Register::A),
            0xE3 => AssemblyDesc::data_byte(offset, opcode),
            0xE4 => AssemblyDesc::data_byte(offset, opcode),
            0xE6 => AssemblyDesc::and_byte(offset, arg_lo),
            0xE7 => AssemblyDesc::rst_byte(offset, 0x20),
            0xE8 => AssemblyDesc::add_byte_to_register(offset, Register::Sp, arg_lo),
            0xE9 => AssemblyDesc::jump_register(offset, Register::Hl),
            0xEA => AssemblyDesc::load_byte_to_address(offset, a, Register::A),
            0xEB => AssemblyDesc::data_byte(offset, opcode),
            0xEC => AssemblyDesc::data_byte(offset, opcode),
            0xED => AssemblyDesc::data_byte(offset, opcode),
            0xEE => AssemblyDesc::xor_byte(offset, arg_lo),
            0xEF => AssemblyDesc::rst_byte(offset, 0x28),
            0xF0 => AssemblyDesc::load_offset_to_register(offset, Register::A, arg_lo as u16),
            0xF2 => AssemblyDesc::load_memory_to_register(offset, Register::A, Register::A),
            0xF3 => AssemblyDesc::di(offset),
            0xF4 => AssemblyDesc::data_byte(offset, opcode),
            0xF6 => AssemblyDesc::or_byte(offset, arg_lo),
            0xF7 => AssemblyDesc::rst_byte(offset, 0x30),
            0xF8 => {
                AssemblyDesc::load_register_p_offset_to_register(offset, Register::Hl, arg_lo as i8)
            }
            0xFA => AssemblyDesc::load_address_to_register(offset, Register::A, a),
            0xFB => AssemblyDesc::ei(offset),
            0xFC => AssemblyDesc::data_byte(offset, opcode),
            0xFD => AssemblyDesc::data_byte(offset, opcode),
            0xFE => AssemblyDesc::cp_byte(offset, arg_lo),
            0xFF => AssemblyDesc::rst_byte(offset, 0x38),
        }
    }
}

impl Display for AssemblyDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seperator = if self.dest == Argument::Unused {
            ""
        } else {
            ", "
        };

        writeln!(f, "{} {}{}{}", self.opcode, self.dest, seperator, self.src)
    }
}

#[derive(PartialEq, Eq)]
pub enum Opcode {
    Byte,
    Load,
    Add,
    Adc,
    Sub,
    Sbc,
    Inc,
    Dec,
    Push,
    Pop,
    And,
    Xor,
    Or,
    Cp,
    Rlca,
    Rrca,
    Rla,
    Rra,
    Nop,
    Stop,
    Jr,
    Jp,
    Call,
    Ret,
    Rst,
    Ei,
    Di,
    Reti,
    Daa,
    Scf,
    Cpl,
    Ccf,
    Halt,
    Rlc,
    Rrc,
    Rl,
    Rr,
    Sla,
    Sra,
    Swap,
    Srl,
    Bit,
    Res,
    Set, //TODO: And catch fire...
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Byte => write!(f, ".byte"),
            Opcode::Load => write!(f, "ld"),
            Opcode::Add => write!(f, "add"),
            Opcode::Adc => write!(f, "adc"),
            Opcode::Sub => write!(f, "sub"),
            Opcode::Sbc => write!(f, "sbc"),
            Opcode::Inc => write!(f, "inc"),
            Opcode::Dec => write!(f, "dec"),
            Opcode::Push => write!(f, "push"),
            Opcode::Pop => write!(f, "pop"),
            Opcode::And => write!(f, "and"),
            Opcode::Xor => write!(f, "xor"),
            Opcode::Or => write!(f, "or"),
            Opcode::Cp => write!(f, "cp"),
            Opcode::Rlca => write!(f, "rlca"),
            Opcode::Rrca => write!(f, "rrca"),
            Opcode::Rla => write!(f, "rla"),
            Opcode::Rra => write!(f, "rra"),
            Opcode::Nop => write!(f, "nop"),
            Opcode::Stop => write!(f, "stop"),
            Opcode::Jr => write!(f, "jr"),
            Opcode::Jp => write!(f, "jp"),
            Opcode::Call => write!(f, "call"),
            Opcode::Ret => write!(f, "ret"),
            Opcode::Rst => write!(f, "rst"),
            Opcode::Ei => write!(f, "ei"),
            Opcode::Di => write!(f, "di"),
            Opcode::Reti => write!(f, "reti"),
            Opcode::Daa => write!(f, "daa"),
            Opcode::Scf => write!(f, "scf"),
            Opcode::Cpl => write!(f, "cpl"),
            Opcode::Ccf => write!(f, "ccf"),
            Opcode::Halt => write!(f, "halt"),
            Opcode::Rlc => write!(f, "rlc"),
            Opcode::Rrc => write!(f, "rrc"),
            Opcode::Rl => write!(f, "rl"),
            Opcode::Rr => write!(f, "rr"),
            Opcode::Sla => write!(f, "sla"),
            Opcode::Sra => write!(f, "sra"),
            Opcode::Swap => write!(f, "swap"),
            Opcode::Srl => write!(f, "srl"),
            Opcode::Bit => write!(f, "bit"),
            Opcode::Res => write!(f, "res"),
            Opcode::Set => write!(f, "set"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Argument {
    Unused,

    Data16(u16),
    Data8(u8),

    Address(u16),
    Offset(u8),
    SOffset(i8),

    Bit(u8),
    R(Register),
    IndexedBy(Register),
    IncRegister(Register),
    DecRegister(Register),
    Condition(Flag),
}

impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Unused => write!(f, " "),
            Argument::Data16(v) => write!(f, "${:04X}", v),
            Argument::Data8(v) => write!(f, "${:02X}", v),
            Argument::Address(v) => write!(f, "(${:04X})", v),
            Argument::Offset(v) => write!(f, "(${:04X})", (0xFF00 + (*v as u16))),
            Argument::SOffset(v) => write!(f, "SP + {:02X}", v),
            Argument::R(r) => write!(f, "{}", r),
            Argument::IndexedBy(r) => write!(f, "({})", r),
            Argument::IncRegister(r) => write!(f, "({}+)", r),
            Argument::DecRegister(r) => write!(f, "({}-)", r),
            Argument::Condition(fl) => write!(f, "{}", fl),
            Argument::Bit(b) => write!(f, "{}", b),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Register {
    Af,
    Bc,
    De,
    Hl,
    Sp,
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Register {
    fn decode_16bit_register(opcode: u8) -> Self {
        let dest_bits = opcode >> 4 & 0x03;
        let reg_group = opcode >> 6 & 0x03;

        match dest_bits {
            0 => Self::Bc,
            1 => Self::De,
            2 => Self::Hl,
            3 if reg_group == 0x03 => Self::Af,
            3 => Self::Sp,
            _ => panic!("Dest Register Index out of range !"),
        }
    }

    fn decode_8bit_dest_register(opcode: u8) -> Self {
        let dest_bits = opcode >> 3 & 0x07;

        match dest_bits {
            0 => Self::B,
            1 => Self::C,
            2 => Self::D,
            3 => Self::E,
            4 => Self::H,
            5 => Self::L,
            6 => Self::Hl,
            7 => Self::A,
            _ => panic!("Dest Register Index out of range !"),
        }
    }

    fn decode_8bit_src_register(opcode: u8) -> Self {
        let src_bits = opcode & 0x07;

        match src_bits {
            0 => Self::B,
            1 => Self::C,
            2 => Self::D,
            3 => Self::E,
            4 => Self::H,
            5 => Self::L,
            6 => Self::Hl,
            7 => Self::A,
            _ => panic!("Src Register Index out of range !"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Af => write!(f, "af"),
            Register::Bc => write!(f, "bc"),
            Register::De => write!(f, "de"),
            Register::Hl => write!(f, "hl"),
            Register::Sp => write!(f, "sp"),
            Register::A => write!(f, "a"),
            Register::F => write!(f, "f"),
            Register::B => write!(f, "b"),
            Register::C => write!(f, "c"),
            Register::D => write!(f, "d"),
            Register::E => write!(f, "e"),
            Register::H => write!(f, "h"),
            Register::L => write!(f, "l"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Flag {
    None,
    NotZero,
    Zero,
    NotCarry,
    Carry,
}

impl Flag {
    fn decode_conditional(opcode: u8) -> Self {
        let mut y = (opcode >> 3) & 0x07;
        let z = opcode & 0x07;
        let x = (opcode >> 6) & 0x03;
        let p = (opcode >> 4) & 0x03;

        //TODO: Improve this check it looks pretty dumb
        if z == 0x05
            || z == 0x03
            || (x == 0x00 && z == 0x00 && y == 0x03)
            || (x == 0x03 && z == 0x01 && p == 0x00)
        {
            return Self::None;
        }

        if x == 0x00 && (y >= 4 && y <= 7) {
            y -= 4;
        }

        match y {
            0 => Self::NotZero,
            1 => Self::Zero,
            2 => Self::NotCarry,
            3 => Self::Carry,
            _ => panic!("Flag opcode value out of range !"),
        }
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flag::None => Ok(()),
            Flag::NotZero => write!(f, "nz"),
            Flag::Zero => write!(f, "z"),
            Flag::NotCarry => write!(f, "nc"),
            Flag::Carry => write!(f, "c"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_conditional_flags_decoding() {
        let result_jp_nz = Flag::decode_conditional(0xC2);
        let result_jp_nc = Flag::decode_conditional(0xD2);
        let result_jp_z = Flag::decode_conditional(0xCC);
        let result_jp_c = Flag::decode_conditional(0xDC);
        let result_jp = Flag::decode_conditional(0xC3);

        assert_eq!(result_jp_nz, Flag::NotZero);
        assert_eq!(result_jp_nc, Flag::NotCarry);
        assert_eq!(result_jp_z, Flag::Zero);
        assert_eq!(result_jp_c, Flag::Carry);
        assert_eq!(result_jp, Flag::None);

        let result_jr_nz = Flag::decode_conditional(0x20);
        let result_jr_nc = Flag::decode_conditional(0x30);
        let result_jr_z = Flag::decode_conditional(0x28);
        let result_jr_c = Flag::decode_conditional(0x38);
        let result_jr = Flag::decode_conditional(0x18);

        assert_eq!(result_jr_nz, Flag::NotZero);
        assert_eq!(result_jr_nc, Flag::NotCarry);
        assert_eq!(result_jr_z, Flag::Zero);
        assert_eq!(result_jr_c, Flag::Carry);
        assert_eq!(result_jr, Flag::None);

        let result_ret_nz = Flag::decode_conditional(0xC0);
        let result_ret_nc = Flag::decode_conditional(0xD0);
        let result_ret_z = Flag::decode_conditional(0xC8);
        let result_ret_c = Flag::decode_conditional(0xD8);
        let result_ret = Flag::decode_conditional(0xC9);

        assert_eq!(result_ret_nz, Flag::NotZero);
        assert_eq!(result_ret_nc, Flag::NotCarry);
        assert_eq!(result_ret_z, Flag::Zero);
        assert_eq!(result_ret_c, Flag::Carry);
        assert_eq!(result_ret, Flag::None);

        let result_call_nz = Flag::decode_conditional(0xC4);
        let result_call_nc = Flag::decode_conditional(0xD4);
        let result_call_z = Flag::decode_conditional(0xCC);
        let result_call_c = Flag::decode_conditional(0xDC);
        let result_call = Flag::decode_conditional(0xCD);

        assert_eq!(result_call_nz, Flag::NotZero);
        assert_eq!(result_call_nc, Flag::NotCarry);
        assert_eq!(result_call_z, Flag::Zero);
        assert_eq!(result_call_c, Flag::Carry);
        assert_eq!(result_call, Flag::None);
    }
}
