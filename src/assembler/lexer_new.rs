use std::io::Write;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum TokenType {
    Identifier,
    ByteRegister,
    WordRegister,
    Number,
    String,

    KeywordNop,
    KeywordLoad,
    KeywordInc,
    KeywordDec,
    KeywordRlca,
    KeywordRrca,
    KeywordRra,
    KeywordAnd,
    KeywordXor,
    KeywordOr,
    KeywordCp,
    KeywordAdd,
    KeywordAdc,
    KeywordSub,
    KeywordSbc,
    KeywordStop,
    KeywordRla,
    KeywordJr,
    KeywordJp,
    KeywordDaa,
    KeywordCpl,
    KeywordCcf,
    KeywordHalt,
    KeywordRet,
    KeywordPush,
    KeywordPop,
    KeywordCall,
    KeywordRst,
    KeywordEi,
    KeywordDi,
    KeywordRlc,
    KeywordRrc,
    KeywordRl,
    KeywordRr,
    KeywordSla,
    KeywordSra,
    KeywordSwap,
    KeywordBit,
    KeywordRes,
    KeywordSet,

    Comma,
    Colon,
    DollarSign,
    OpenParen,
    CloseParen,
    Dot,
    Equals,
    RenamingOperator,
    Minus,
    Plus,

    Eof,
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "a" => TokenType::ByteRegister,
            "b" => TokenType::ByteRegister,
            "c" => TokenType::ByteRegister,
            "d" => TokenType::ByteRegister,
            "e" => TokenType::ByteRegister,
            "h" => TokenType::ByteRegister,
            "l" => TokenType::ByteRegister,
            "af" => TokenType::WordRegister,
            "bc" => TokenType::WordRegister,
            "de" => TokenType::WordRegister,
            "hl" => TokenType::WordRegister,

            "nop" => TokenType::KeywordNop,
            "ld" => TokenType::KeywordLoad,
            "inc" => TokenType::KeywordInc,
            "dec" => TokenType::KeywordDec,
            "rlca" => TokenType::KeywordRlca,
            "rrca" => TokenType::KeywordRrca,
            "rra" => TokenType::KeywordRra,
            "and" => TokenType::KeywordAnd,
            "xor" => TokenType::KeywordXor,
            "or" => TokenType::KeywordOr,
            "cp" => TokenType::KeywordCp,
            "add" => TokenType::KeywordAdd,
            "adc" => TokenType::KeywordAdc,
            "sub" => TokenType::KeywordSub,
            "sbc" => TokenType::KeywordSbc,
            "stop" => TokenType::KeywordStop,
            "rla" => TokenType::KeywordRla,
            "jr" => TokenType::KeywordJr,
            "jp" => TokenType::KeywordJp,
            "daa" => TokenType::KeywordDaa,
            "cpl" => TokenType::KeywordCpl,
            "ccf" => TokenType::KeywordCcf,
            "halt" => TokenType::KeywordHalt,
            "ret" => TokenType::KeywordRet,
            "push" => TokenType::KeywordPush,
            "pop" => TokenType::KeywordPop,
            "call" => TokenType::KeywordCall,
            "rst" => TokenType::KeywordRst,
            "ei" => TokenType::KeywordEi,
            "di" => TokenType::KeywordDi,
            "rlc" => TokenType::KeywordRlc,
            "rrc" => TokenType::KeywordRrc,
            "rl" => TokenType::KeywordRl,
            "rr" => TokenType::KeywordRr,
            "sla" => TokenType::KeywordSla,
            "sra" => TokenType::KeywordSra,
            "swap" => TokenType::KeywordSwap,
            "bit" => TokenType::KeywordBit,
            "res" => TokenType::KeywordRes,
            "set" => TokenType::KeywordSet,
            _ => TokenType::Identifier,
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Token {
    line: usize,
    start: usize,
    end: usize,

    tokentype: TokenType,
}

impl Token {
    fn new(start: usize, end: usize, line: usize, tokentype: TokenType) -> Self {
        Self {
            line,
            start,
            end,
            tokentype,
        }
    }
}

struct StringProcessor;
impl Processor for StringProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<ProcessorMessage, LexerErrors> {
        lexer.start = lexer.current;

        loop {
            let next = lexer.advance();

            if next.is_none() {
                lexer.change_state(&EndOfFileProcessor);
                return Err(LexerErrors::StringWithoutEnd);
            }

            match next.unwrap() {
                '"' => break,
                _ => continue,
            }
        }

        lexer.push_token(Token::new(
            lexer.start,
            lexer.current,
            lexer.line,
            TokenType::String,
        ));
        lexer.change_state(&BasicProcessor);
        Ok(ProcessorMessage::TokenAdded)
    }
}

struct OperatorProcessor;
impl Processor for OperatorProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<ProcessorMessage, LexerErrors> {
        lexer.start = lexer.current;
        let current = lexer.advance();

        if current.is_none() {
            lexer.change_state(&EndOfFileProcessor);
        }

        let next = lexer.peek();

        let tokentype = match current.unwrap() {
            ',' => TokenType::Comma,
            '(' => TokenType::OpenParen,
            ')' => TokenType::CloseParen,
            '.' => TokenType::Dot,
            '$' => TokenType::DollarSign,
            ':' => TokenType::Colon,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            '=' if next.is_some() && next.unwrap() == '>' => {
                lexer.advance();
                TokenType::RenamingOperator
            }
            '=' => TokenType::Equals,
            _ => return Err(LexerErrors::UnknownOperator),
        };

        lexer.push_token(Token::new(
            lexer.start,
            lexer.current,
            lexer.line,
            tokentype,
        ));
        lexer.change_state(&BasicProcessor);

        Ok(ProcessorMessage::TokenAdded)
    }
}

struct BinaryNumberProcessor;
impl Processor for BinaryNumberProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<ProcessorMessage, LexerErrors> {
        lexer.start = lexer.current;

        loop {
            let next = lexer.advance();

            if next.is_none() {
                lexer.change_state(&EndOfFileProcessor);
                //TODO: Error Binary value without value ? (%??????)
                break;
            }

            match next.unwrap() {
                '0'..='1' => continue,
                ' ' => break,
                _ => return Err(LexerErrors::InvalidNumberLiteral), //TODO: Different Error ?
            }
        }

        lexer.push_token(Token::new(
            lexer.start,
            lexer.current,
            lexer.line,
            TokenType::Number,
        ));
        lexer.change_state(&BasicProcessor);
        Ok(ProcessorMessage::TokenAdded)
    }
}

struct HexadecimalNumberProcessor;
impl Processor for HexadecimalNumberProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<ProcessorMessage, LexerErrors> {
        lexer.start = lexer.current;

        loop {
            let next = lexer.advance();

            if next.is_none() {
                lexer.change_state(&EndOfFileProcessor);
                //TODO: Error Hexadecimal marker without value  (0x??????)!
                break;
            }

            match next.unwrap() {
                '0'..='9' | 'a'..='f' | 'A'..='F' => continue,
                ' ' => break,
                _ => return Err(LexerErrors::InvalidNumberLiteral), //TODO: Different Error ?
            }
        }

        lexer.push_token(Token::new(
            lexer.start,
            lexer.current,
            lexer.line,
            TokenType::Number,
        ));
        lexer.change_state(&BasicProcessor);
        Ok(ProcessorMessage::TokenAdded)
    }
}

struct DecimalNumberProcessor;
impl Processor for DecimalNumberProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<ProcessorMessage, LexerErrors> {
        lexer.start = lexer.current;

        loop {
            let next = lexer.advance();

            if next.is_none() {
                lexer.change_state(&EndOfFileProcessor);
                break;
            }

            match next.unwrap() {
                'x' => {
                    lexer.change_state(&HexadecimalNumberProcessor);
                    return Ok(ProcessorMessage::Nop);
                }
                '0'..='9' => continue,
                ' ' => break,
                _ => return Err(LexerErrors::InvalidNumberLiteral),
            }
        }

        lexer.push_token(Token::new(
            lexer.start,
            lexer.current,
            lexer.line,
            TokenType::Number,
        ));
        lexer.change_state(&BasicProcessor);
        Ok(ProcessorMessage::TokenAdded)
    }
}

struct IdentifierProcessor;
impl Processor for IdentifierProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<ProcessorMessage, LexerErrors> {
        let mut result = ProcessorMessage::Nop;
        lexer.start = lexer.current - 1;
        loop {
            let next = lexer.advance();

            if next.is_none() {
                lexer.change_state(&EndOfFileProcessor);
                break;
            }

            match next.unwrap() {
                '_' | 'a'..='z' | 'A'..='Z' => continue,
                _ => break,
            }
        }

        if lexer.start != lexer.current {
            result = ProcessorMessage::TokenAdded;
            let tokentype = TokenType::from(lexer.slice_from_source());
            lexer.push_token(Token::new(
                lexer.start,
                lexer.current,
                lexer.line,
                tokentype,
            ));
        }

        lexer.change_state(&BasicProcessor);

        Ok(result)
    }
}

struct EndOfFileProcessor;
impl Processor for EndOfFileProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<ProcessorMessage, LexerErrors> {
        lexer.start = lexer.current;
        lexer.push_token(Token::new(
            lexer.start,
            lexer.current,
            lexer.line,
            TokenType::Eof,
        ));
        lexer.stop_processing();

        Ok(ProcessorMessage::TokenAdded)
    }
}

struct BasicProcessor;
impl Processor for BasicProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<ProcessorMessage, LexerErrors> {
        lexer.consume_whitespace();

        let ch = lexer.advance();

        if ch.is_none() {
            lexer.change_state(&EndOfFileProcessor); //Return some type of EOF
            return Ok(ProcessorMessage::Nop);
        }

        match ch.unwrap() {
            '%' => lexer.change_state(&BinaryNumberProcessor),
            '_' | 'a'..='z' | 'A'..='Z' => lexer.change_state(&IdentifierProcessor),
            '0'..='9' => lexer.change_state(&DecimalNumberProcessor),
            '"' => lexer.change_state(&StringProcessor),
            _ => todo!(),
        };

        Ok(ProcessorMessage::Nop)
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum ProcessorMessage {
    TokenAdded,
    Nop, //TODO: Rename this into something that makes more sense like ProcessorChanged or StateChanged!
}

#[derive(Debug)]
enum LexerErrors {
    StringWithoutEnd,
    InvalidNumberLiteral,
    UnknownOperator,
}

impl std::fmt::Display for LexerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerErrors::StringWithoutEnd => write!(
                f,
                "String literal without end found. Please add a '\"' to the end of your string"
            ),
            LexerErrors::InvalidNumberLiteral => todo!(),
            LexerErrors::UnknownOperator => todo!(),
        }
    }
}

trait Processor {
    fn process(&self, lexer: &mut Lexer) -> Result<ProcessorMessage, LexerErrors>;
}

struct Lexer<'a> {
    source: String,
    char_iter: Vec<char>,

    line: usize,
    start: usize,
    current: usize,

    state: Option<&'a dyn Processor>,

    ring_buffer: [Option<Token>; 2],

    ring_write_pos: usize,
    ring_prev_written: isize,
    ring_read_pos: usize,
}

impl<'a> Lexer<'a> {
    const RING_SIZE: usize = 2;
    fn new(source: &'a str) -> Self {
        Self {
            source: source.to_string(),
            char_iter: source.chars().collect(),
            line: 1,

            start: 0,
            current: 0,

            state: Some(&BasicProcessor),

            ring_buffer: [None; 2],
            ring_write_pos: 0,
            ring_prev_written: 0,
            ring_read_pos: 0,
        }
    }

    fn slice_from_source(&self) -> &str {
        &self.source[self.start..self.current]
    }

    fn push_token(&mut self, token: Token) {
        self.ring_prev_written = self.ring_write_pos as isize;
        self.ring_buffer[self.ring_write_pos] = Some(token);
        self.ring_write_pos = (self.ring_write_pos + 1) % Self::RING_SIZE;
    }

    fn peek_prev(&self) -> Option<Token> {
        let prev_pos = ((self.ring_prev_written - 1) % Self::RING_SIZE as isize) as usize;
        self.ring_buffer[prev_pos]
    }

    fn peek_token(&mut self) -> Option<Token> {
        self.ring_buffer[self.ring_read_pos]
    }

    fn peek_next_token(&mut self) -> Option<Token> {
        self.ring_buffer[(self.ring_read_pos + 1) % Self::RING_SIZE]
    }

    fn pop_token(&mut self) -> Option<Token> {
        let token = self.ring_buffer[self.ring_read_pos].take();
        self.ring_read_pos = (self.ring_read_pos + 1) % Self::RING_SIZE;
        token
    }

    fn consume_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            //TODO: There are probably other utf-8 characters that lead to a newline should we handle these
            //cases as well ?
            if ch == '\n' {
                self.line += 1;
            }

            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.current >= self.source.len() {
            return None;
        }

        Some(self.char_iter[self.current])
    }

    fn advance(&mut self) -> Option<char> {
        if self.current >= self.source.len() {
            return None;
        }

        let pos = self.current;

        self.current += 1;
        Some(self.char_iter[pos])
    }

    fn change_state(&mut self, processor: &'a impl Processor) {
        self.state = Some(processor);
    }

    fn stop_processing(&mut self) {
        self.state = None;
    }

    fn report_error(&self, err: LexerErrors) {
        let stderr = std::io::stderr();
        let mut handle = stderr.lock();
        let err_msg = format!("SCANNER:ERROR:{}: {}", self.current, err);
        handle.write_all(err_msg.as_bytes()).unwrap();
    }

    fn scan_next_token(&mut self) {
        while let Some(processor) = self.state {
            match processor.process(self) {
                Ok(ProcessorMessage::TokenAdded) => break,
                Ok(_) => continue,
                Err(err) => self.report_error(err),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn scan_empty() {
        let source = "";
        let expected_eof = Token {
            start: 0,
            end: 0,
            line: 1,
            tokentype: TokenType::Eof,
        };

        let mut lexer = Lexer::new(source);
        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_eof);
    }

    #[test]
    fn scan_identifier() {
        let source = "hello world \"Test String\"";
        let expected_hello = Token {
            start: "".len(),
            end: "hello ".len(),
            line: 1,
            tokentype: TokenType::Identifier,
        };

        let expected_world = Token {
            start: "hello ".len(),
            end: "hello world ".len(),
            line: 1,
            tokentype: TokenType::Identifier,
        };

        let expected_string = Token {
            start: "hello world \"".len(),
            end: "hello world \"Test String\"".len(),
            line: 1,
            tokentype: TokenType::String,
        };

        let expected_eof = Token {
            start: "hello world \"Test String\"".len(),
            end: "hello world \"Test String\"".len(),
            line: 1,
            tokentype: TokenType::Eof,
        };

        let mut lexer = Lexer::new(source);

        lexer.scan_next_token();
        assert_eq!(lexer.peek_token().unwrap(), expected_hello);

        lexer.scan_next_token();
        assert_eq!(lexer.peek_prev().unwrap(), expected_hello);

        assert_eq!(lexer.pop_token().unwrap(), expected_hello);
        assert_eq!(lexer.peek_token().unwrap(), expected_world);
        assert_eq!(lexer.pop_token().unwrap(), expected_world);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_string);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_eof);
    }

    #[test]
    fn scan_number() {
        let source = "1 10 100 1000 10000";
        let expected_one = Token {
            start: "1".len(),
            end: "1 ".len(),
            line: 1,
            tokentype: TokenType::Number,
        };

        let expected_ten = Token {
            start: "1 1".len(),
            end: "1 10 ".len(),
            line: 1,
            tokentype: TokenType::Number,
        };

        let expected_one_hundred = Token {
            start: "1 10 1".len(),
            end: "1 10 100 ".len(),
            line: 1,
            tokentype: TokenType::Number,
        };

        let expected_one_thousand = Token {
            start: "1 10 100 1".len(),
            end: "1 10 100 1000 ".len(),
            line: 1,
            tokentype: TokenType::Number,
        };

        let expected_ten_thousand = Token {
            start: "1 10 100 1000 1".len(),
            end: "1 10 100 1000 10000".len(),
            line: 1,
            tokentype: TokenType::Number,
        };

        let expected_eof = Token {
            start: "1 10 100 1000 10000".len(),
            end: "1 10 100 1000 10000".len(),
            line: 1,
            tokentype: TokenType::Eof,
        };

        let mut lexer = Lexer::new(source);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_one);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_ten);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_one_hundred);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_one_thousand);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_ten_thousand);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_eof);
    }

    #[test]
    fn scan_hex_number() {
        let source = "0x0 0xFF 0xFFAA";
        let expected_zero = Token {
            start: "0x".len(),
            end: "0x0 ".len(),
            line: 1,
            tokentype: TokenType::Number,
        };

        let expected_ff = Token {
            start: "0x0 0x".len(),
            end: "0x0 0xFF ".len(),
            line: 1,
            tokentype: TokenType::Number,
        };

        let expected_ffaa = Token {
            start: "0x0 0xFF 0x".len(),
            end: "0x0 0xFF 0xFFAA".len(),
            line: 1,
            tokentype: TokenType::Number,
        };

        let expected_eof = Token {
            start: "0x0 0xFF 0xFFAA".len(),
            end: "0x0 0xFF 0xFFAA".len(),
            line: 1,
            tokentype: TokenType::Eof,
        };

        let mut lexer = Lexer::new(source);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_zero);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_ff);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_ffaa);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_eof);
    }

    #[test]
    fn scan_binary_number() {
        let source = "%01 %10 %11";
        let expected_one = Token {
            start: "%".len(),
            end: "%01 ".len(),
            line: 1,
            tokentype: TokenType::Number,
        };

        let expected_two = Token {
            start: "%01 %".len(),
            end: "%01 %10 ".len(),
            line: 1,
            tokentype: TokenType::Number,
        };

        let expected_three = Token {
            start: "%01 %10 %".len(),
            end: "%01 %10 %11".len(),
            line: 1,
            tokentype: TokenType::Number,
        };

        let expected_eof = Token {
            start: "%01 %10 %11".len(),
            end: "%01 %10 %11".len(),
            line: 1,
            tokentype: TokenType::Eof,
        };

        let mut lexer = Lexer::new(source);
        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_one);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_two);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_three);

        lexer.scan_next_token();
        assert_eq!(lexer.pop_token().unwrap(), expected_eof);
    }
}
