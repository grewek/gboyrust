use crate::assembler::lexer;
use crate::assembler::lexer::{Token, TokenType};

struct Parser {
    source_ref: String,
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug, Eq, PartialEq)]
enum ByteReg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Eq, PartialEq)]
enum WordReg {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Debug, Eq, PartialEq)]
enum Condition {
    Unconditional,
    OnZeroSet,
    OnNegativeSet,
    OnCarrySet,
    OnHalfCarrySet,
}

#[derive(Debug, Eq, PartialEq)]
enum Target {
    MemReg(WordReg),
    MemRegInc(WordReg),
    MemRegDec(WordReg),

    ByteReg(ByteReg),
    WordReg(WordReg),

    Address16(u16),

    Data8(u8),
    Data16(u16),
}

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Load(Target, Target),
    Inc(Target),
    Dec(Target),
    Push(WordReg),
    Pop(WordReg),
    JumpRel(Condition, Target),
}

//NOTE: This is probably the dumbest and messiest parser ever written...
//      maybe it will call some lovecraftian horror...
impl Parser {
    //TOKENTYPE: PUSH 16bitreg
    fn new(source_ref: &str, token_stream: Vec<Token>) -> Self {
        Self {
            source_ref: source_ref.to_string(),
            tokens: token_stream,
            position: 0,
        }
    }

    fn parse(&mut self) -> Command {
        //FIXME: The parser will parse everything even it is not possible like
        //       pushing a 8bit register to the stack i need a better way to handle
        //       these cases !
        match self.advance().unwrap().tokentype() {
            TokenType::Ld => Command::Load(self.parse_target(), self.parse_target()),
            TokenType::Inc => Command::Inc(self.parse_target()),
            TokenType::Dec => Command::Dec(self.parse_target()),
            TokenType::Push => Command::Push(self.parse_16bit_register()),
            TokenType::Pop => Command::Pop(self.parse_16bit_register()),
            TokenType::Jr => Command::JumpRel(self.parse_conditional(), self.parse_jump_target()),
            _ => panic!("This token is not yet known and needs to be implemented!"),
        }
    }

    fn parse_conditional(&mut self) -> Condition {
        match self.peek_type_top() {
            TokenType::ZeroFlag => Condition::OnZeroSet,
            TokenType::NegativeFlag => Condition::OnNegativeSet,
            TokenType::CarryFlag => Condition::OnCarrySet,
            TokenType::HalfCarryFlag => Condition::OnHalfCarrySet,
            _ => Condition::Unconditional,
        }
    }

    fn parse_jump_target(&mut self) -> Target {
        let token = self.advance().unwrap();

        match token.tokentype() {
            TokenType::BinaryValueByte => {
                Target::Data8(self.parse_byte_value(2, token.repr_range()))
            }
            TokenType::DecimalValueByte => {
                Target::Data8(self.parse_byte_value(10, token.repr_range()))
            }
            TokenType::HexValueByte => Target::Data8(self.parse_byte_value(16, token.repr_range())),
            _ => panic!("INVALID TOKEN ERROR HANDLING"),
        }
    }

    fn parse_16bit_register(&mut self) -> WordReg {
        let token = self.advance().unwrap();

        match token.tokentype() {
            TokenType::RegisterAF => WordReg::AF,
            TokenType::RegisterBC => WordReg::BC,
            TokenType::RegisterDE => WordReg::DE,
            TokenType::RegisterHL => WordReg::HL,
            _ => panic!(
                "These should be errors we expected a 16bit register but we got something else !"
            ),
        }
    }

    fn parse_target(&mut self) -> Target {
        if self.match_token(TokenType::Comma) {
            self.advance();
        }

        let token = self.advance().unwrap();

        match token.tokentype() {
            TokenType::OpenParen => self.parse_parentheses_expression(),
            TokenType::RegisterA => Target::ByteReg(ByteReg::A),
            TokenType::RegisterB => Target::ByteReg(ByteReg::B),
            TokenType::RegisterC => Target::ByteReg(ByteReg::C),
            TokenType::RegisterD => Target::ByteReg(ByteReg::D),
            TokenType::RegisterE => Target::ByteReg(ByteReg::E),
            TokenType::RegisterH => Target::ByteReg(ByteReg::H),
            TokenType::RegisterL => Target::ByteReg(ByteReg::L),
            TokenType::PoundSign => self.parse_immediate_data(),
            TokenType::RegisterAF => Target::WordReg(WordReg::AF),
            TokenType::RegisterBC => Target::WordReg(WordReg::BC),
            TokenType::RegisterDE => Target::WordReg(WordReg::DE),
            TokenType::RegisterHL => Target::WordReg(WordReg::HL),
            TokenType::HexValueByte => Target::Data8(self.parse_byte_value(16, token.repr_range())),
            _ => {
                dbg!(self.peek_type_top());
                unimplemented!()
            }
        }
    }

    fn parse_immediate_data(&mut self) -> Target {
        let token = self.advance().unwrap(); //ERROR Handling ;) top on the list ^_^

        //FIXME: Very much repetition of calling parse_*_value could we factor that out a bit ?
        match token.tokentype() {
            TokenType::BinaryValueByte => {
                Target::Data8(self.parse_byte_value(2, token.repr_range()))
            }
            TokenType::BinaryValueWord => {
                Target::Data16(self.parse_word_value(2, token.repr_range()))
            }
            TokenType::DecimalValueByte => {
                Target::Data8(self.parse_byte_value(10, token.repr_range()))
            }
            TokenType::DecimalValueWord => {
                Target::Data16(self.parse_word_value(10, token.repr_range()))
            }
            TokenType::HexValueByte => Target::Data8(self.parse_byte_value(16, token.repr_range())),
            TokenType::HexValueWord => {
                Target::Data16(self.parse_word_value(16, token.repr_range()))
            }
            _ => panic!("Not valid in the current context AKA WE NEED ERROR HANDLING..."),
        }
    }

    fn parse_parentheses_expression(&mut self) -> Target {
        let token = self.advance().unwrap();
        let addr_mode = match token.tokentype() {
            TokenType::Identifier => todo!(),
            TokenType::BinaryValueWord => {
                Target::Address16(self.parse_word_value(2, token.repr_range()))
            }
            TokenType::DecimalValueWord => {
                Target::Address16(self.parse_word_value(10, token.repr_range()))
            }
            TokenType::HexValueWord => {
                Target::Address16(self.parse_word_value(16, token.repr_range()))
            }
            TokenType::RegisterBC => Target::WordReg(WordReg::BC),
            TokenType::RegisterDE => Target::WordReg(WordReg::DE),
            TokenType::RegisterHL => Target::WordReg(WordReg::HL),
            _ => unimplemented!(),
        };

        self.advance(); //Remove the closingparen token
        addr_mode
    }

    fn advance_by(&mut self, amount: usize) {
        let mut i = amount;

        while i > 0 {
            self.advance();
            i -= 1;
        }
    }

    fn match_token(&mut self, to_match: TokenType) -> bool {
        if !(self.peek_type_top() == to_match) {
            return false;
        }

        true
    }

    fn parse_register(&mut self) -> ByteReg {
        let register = match self.peek_type_top() {
            TokenType::RegisterA => ByteReg::A,
            TokenType::RegisterB => ByteReg::B,
            _ => unimplemented!(),
        };

        self.advance();

        register
    }

    fn advance(&mut self) -> Option<Token> {
        //TODO: Make sure we never try to read behind the token buffer...
        if self.position >= self.tokens.len() {
            return None;
        }

        let token = self.tokens[self.position];
        self.position += 1;

        Some(token)
    }

    fn peek_type_top(&self) -> TokenType {
        //TODO: Check for overflow!
        self.tokens[self.position].tokentype()
    }

    fn peek_type_fwd(&self) -> TokenType {
        //TODO: Check for overflow!
        self.tokens[self.position + 1].tokentype()
    }

    fn match_token_advance(&mut self, to_match: TokenType) -> Option<Token> {
        if self.peek_type_top() != to_match {
            return None;
        }

        let result = Some(self.tokens[self.position]);
        self.advance();

        result
    }

    fn parse_byte_value(&mut self, radix: u32, range: std::ops::Range<usize>) -> u8 {
        let value = u8::from_str_radix(&self.source_ref[range], radix);

        //TODO: ERROR Handling !
        value.unwrap()
    }

    fn parse_word_value(&mut self, radix: u32, range: std::ops::Range<usize>) -> u16 {
        let value = u16::from_str_radix(&self.source_ref[range], radix);

        //TODO: ERROR Handling !
        value.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assembler::lexer;
    #[test]
    fn parse_inc_reg() {
        let source = "inc a";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::Inc(Target::ByteReg(ByteReg::A));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_inc_mem() {
        let source = "inc (hl)";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::Inc(Target::WordReg(WordReg::HL));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_dec_reg() {
        let source = "dec a";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::Dec(Target::ByteReg(ByteReg::A));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ld_reg_to_reg() {
        let source = "ld b,a";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result: Command = Parser::new(source, token_stream).parse();
        let expected = Command::Load(Target::ByteReg(ByteReg::B), Target::ByteReg(ByteReg::A));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ld_mem_indirect_to_reg() {
        let source = "ld a,(hl)";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();
        let result = Parser::new(source, token_stream).parse();

        let expected = Command::Load(Target::ByteReg(ByteReg::A), Target::WordReg(WordReg::HL));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ld_mem_direct_hex_to_reg() {
        let source = "ld a, ($FFFF)";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();
        let result = Parser::new(source, token_stream).parse();

        let expected = Command::Load(Target::ByteReg(ByteReg::A), Target::Address16(0xFFFF));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ld_mem_direct_bin_to_reg() {
        let source = "ld a, (%1111111111111111)";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();

        let expected = Command::Load(Target::ByteReg(ByteReg::A), Target::Address16(0xFFFF));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ld_mem_direct_dec_to_reg() {
        let source = "ld a, (65535)";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::Load(Target::ByteReg(ByteReg::A), Target::Address16(0xFFFF));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ld_reg_to_mem() {
        let source = "ld ($FFFF), a";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::Load(Target::Address16(0xFFFF), Target::ByteReg(ByteReg::A));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ld_reg_immediate_8bit() {
        let source = "ld a, #120";

        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::Load(Target::ByteReg(ByteReg::A), Target::Data8(120));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_push_reg() {
        let source = "push af";

        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::Push(WordReg::AF);

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_pop_reg() {
        let source = "pop bc";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::Pop(WordReg::BC);

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_unconditional_relative_jump() {
        let source = "jr $FF";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::JumpRel(Condition::Unconditional, Target::Data8(0xFF));

        assert_eq!(result, expected);
    }
}
