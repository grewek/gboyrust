use crate::assembler::lexer;
use crate::assembler::lexer::{Token, TokenType};

struct Parser {
    source_ref: String,
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug, Eq, PartialEq)]
enum ParserRegister {
    A,
    B,
}

#[derive(Debug, Eq, PartialEq)]
enum MemorySources {
    Bc,
    De,
    Hl,
    HlPlus,
    HlMinus,
    Address16(u16),
}

#[derive(Debug, Eq, PartialEq)]
enum AddressingMode {
    RegToReg(ParserRegister, ParserRegister),
    MemToReg(ParserRegister, MemorySources),
}

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Load(AddressingMode),
}

//NOTE: This is probably the dumbest and messiest parser ever written...
//      maybe it will call some lovecraftian horror...
impl Parser {
    fn new(source_ref: &str, token_stream: Vec<Token>) -> Self {
        Self {
            source_ref: source_ref.to_string(),
            tokens: token_stream,
            position: 0,
        }
    }

    fn parse(&mut self) -> Command {
        //NOTE: Sorry to everyone who reads this horror story of a parser if you find cthullu in here
        //please let me know...i am not a professional programmer and i have really no idea how to
        //write a good parser if you have some good sources for education purposes

        let token = &self.tokens[self.position];

        match token.tokentype() {
            TokenType::Ld => Command::Load(self.parse_load_command()),
            _ => panic!("This token is not yet known and needs to be implemented!"),
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }
    fn peek_type(&self) -> TokenType {
        self.tokens[self.position].tokentype()
    }

    fn match_token_advance(&mut self, to_match: TokenType) -> Option<Token> {
        if self.peek_type() != to_match {
            return None;
        }

        let result = Some(self.tokens[self.position]);
        self.advance();

        result
    }

    fn parse_load_command(&mut self) -> AddressingMode {
        //possible cases
        // 16bit-address to Register
        // 16bit-Memory to Register
        // 16bit-Memory+1 to Register
        // 16bit-Memory-1 to Register
        // Register to Register
        self.position += 1;
        let destination = match self.peek_type() {
            TokenType::RegisterA => ParserRegister::A,
            TokenType::RegisterB => ParserRegister::B,
            _ => todo!(),
        };

        self.advance();
        self.match_token_advance(TokenType::Comma);

        match self.peek_type() {
            TokenType::OpenParen => {
                AddressingMode::MemToReg(destination, self.parse_memory_source())
            }

            TokenType::RegisterA => AddressingMode::RegToReg(destination, ParserRegister::A),
            TokenType::RegisterB => AddressingMode::RegToReg(destination, ParserRegister::B),
            _ => todo!(),
        }
    }

    fn parse_memory_source(&mut self) -> MemorySources {
        self.advance();

        let source = match self.peek_type() {
            TokenType::RegisterBC => MemorySources::Bc,
            TokenType::RegisterDE => MemorySources::De,
            TokenType::RegisterHL => MemorySources::Hl,
            TokenType::DecimalValue => MemorySources::Address16(self.parse_value(10)),
            TokenType::BinaryValue => MemorySources::Address16(self.parse_value(2)),
            TokenType::HexValue => MemorySources::Address16(self.parse_value(16)),
            _ => panic!("These tokentypes are not allowed !"),
        };

        self.advance();
        if let Some(_) = self.match_token_advance(TokenType::CloseParen) {
            return source;
        }

        if let MemorySources::Hl = source {
            match self.peek_type() {
                TokenType::Plus => {
                    self.advance();
                    return MemorySources::HlPlus;
                }
                TokenType::Minus => {
                    self.advance();
                    return MemorySources::HlMinus;
                }
                _ => panic!("Invalid symbol"),
            }
        }

        panic!("Things went south and we are here because we did not think hard and long enough...")
    }

    fn parse_value(&mut self, radix: u32) -> u16 {
        let value_token = self.tokens[self.position];
        let value = u16::from_str_radix(&self.source_ref[value_token.repr_range()], radix);

        //TODO: ERROR Handling !
        value.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assembler::lexer;
    #[test]
    fn parse_ld_reg_to_reg() {
        let source = "ld b,a";
        let token_stream = lexer::Lexer::new(&source).tokenize();

        let result: Command = Parser::new(source, token_stream).parse();
        let expected = Command::Load(AddressingMode::RegToReg(
            ParserRegister::B,
            ParserRegister::A,
        ));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ld_mem_indirect_to_reg() {
        let source = "ld a,(hl)";
        let token_stream = lexer::Lexer::new(&source).tokenize();
        let result = Parser::new(source, token_stream).parse();

        let expected = Command::Load(AddressingMode::MemToReg(
            ParserRegister::A,
            MemorySources::Hl,
        ));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ld_mem_direct_hex_to_reg() {
        let source = "ld a, ($FFFF)";
        let token_stream = lexer::Lexer::new(&source).tokenize();
        let result = Parser::new(source, token_stream).parse();

        let expected = Command::Load(AddressingMode::MemToReg(
            ParserRegister::A,
            MemorySources::Address16(0xFFFF),
        ));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ld_mem_direct_bin_to_reg() {
        let source = "ld a, (%1111111111111111)";
        let token_stream = lexer::Lexer::new(&source).tokenize();

        let result = Parser::new(source, token_stream).parse();

        let expected = Command::Load(AddressingMode::MemToReg(
            ParserRegister::A,
            MemorySources::Address16(0xFFFF),
        ));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ld_mem_direct_dec_to_reg() {
        let source = "ld a, (65535)";
        let token_stream = lexer::Lexer::new(&source).tokenize();

        let result = Parser::new(source, token_stream).parse();

        let expected = Command::Load(AddressingMode::MemToReg(
            ParserRegister::A,
            MemorySources::Address16(0xFFFF),
        ));

        assert_eq!(result, expected);
    }
}
