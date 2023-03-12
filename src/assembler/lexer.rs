#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub enum TokenType {
    //Symbols
    OpenParen,
    CloseParen,
    DollarSign,
    PoundSign,
    Comma,
    Dot,
    Colon,
    Plus,
    Minus,

    //Keywords
    Nop,
    Ld,
    Inc,
    Dec,
    Rlca,
    Rrca,
    Rra,
    And,
    Xor,
    Or,
    Cp,
    Add,
    Adc,
    Sub,
    Sbc,
    Stop,
    Rla,
    Jr,
    Jp,
    Daa,
    Cpl,
    Ccf,
    Halt,
    Ret,
    Push,
    Pop,
    Call,
    Rst,
    Ei,
    Di,

    //Flags
    ZeroFlag,
    NegativeFlag,
    CarryFlag,
    HalfCarryFlag, //TODO: this should be removed you cannot act on halfcarry flag changes ?

    //Value Definitions
    MacroRenameRegister,
    MacroDefineFunctionStart,
    MacroDefineFunctionEnd,
    MacroDefineByte,
    MacroDefineWord,
    MacroDefineDWord,
    MacroDefineByteArray,
    String,

    //Symbols
    RegisterA,
    RegisterB,
    RegisterC,
    RegisterD,
    RegisterE,
    RegisterH,
    RegisterL,
    RegisterAF,
    RegisterBC,
    RegisterDE,
    RegisterHL,
    RegisterSP,

    //Values
    HexValueByte,
    HexValueWord,
    DecimalValueByte,
    DecimalValueWord,
    BinaryValueByte,
    BinaryValueWord,
    Identifier,

    //ControlSymbols
    EOF,
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "nop" => TokenType::Nop,
            "ld" => TokenType::Ld,
            "inc" => TokenType::Inc,
            "dec" => TokenType::Dec,
            "rlca" => TokenType::Rlca,
            "rrca" => TokenType::Rrca,
            "rra" => TokenType::Rra,
            "and" => TokenType::And,
            "xor" => TokenType::Xor,
            "or" => TokenType::Or,
            "cp" => TokenType::Cp,
            "add" => TokenType::Add,
            "adc" => TokenType::Adc,
            "sub" => TokenType::Sub,
            "sbc" => TokenType::Sbc,
            "stop" => TokenType::Stop,
            "rla" => TokenType::Rla,
            "jr" => TokenType::Jr,
            "jp" => TokenType::Jp,
            "daa" => TokenType::Daa,
            "cpl" => TokenType::Cpl,
            "ccf" => TokenType::Ccf,
            "halt" => TokenType::Halt,
            "ret" => TokenType::Ret,
            "push" => TokenType::Push,
            "pop" => TokenType::Pop,
            "call" => TokenType::Call,
            "rst" => TokenType::Rst,
            "ei" => TokenType::Ei,
            "di" => TokenType::Di,

            "fun" => TokenType::MacroDefineFunctionStart,
            "end" => TokenType::MacroDefineFunctionEnd,
            "db" => TokenType::MacroDefineByte,
            "dw" => TokenType::MacroDefineWord,
            "dd" => TokenType::MacroDefineDWord,
            "dba" => TokenType::MacroDefineByteArray,

            //TODO: These are renamed to not collide with the register keywords...
            //      may be we should handle these and the registers as symbols and let the
            //      parser handle the fine details...
            "zf" => TokenType::ZeroFlag,
            "nf" => TokenType::NegativeFlag,
            "cf" => TokenType::CarryFlag,
            "hf" => TokenType::HalfCarryFlag,

            //Register Symbols
            "a" => TokenType::RegisterA,
            "b" => TokenType::RegisterB,
            "c" => TokenType::RegisterC,
            "d" => TokenType::RegisterD,
            "e" => TokenType::RegisterE,
            "h" => TokenType::RegisterH,
            "l" => TokenType::RegisterL,
            "af" => TokenType::RegisterAF,
            "bc" => TokenType::RegisterBC,
            "de" => TokenType::RegisterDE,
            "hl" => TokenType::RegisterHL,
            "sp" => TokenType::RegisterSP,

            _ => TokenType::Identifier,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
enum TokenCategory {
    Macro,
    Keyword,
    Register,
    Flag,
    Identifier,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
pub struct Token {
    token: TokenType,
    position: TokenPosition,
}

impl Token {
    pub fn tokentype(&self) -> TokenType {
        self.token
    }

    pub fn repr_range(&self) -> std::ops::Range<usize> {
        self.position.start..self.position.end
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
struct TokenPosition {
    start: usize,
    end: usize,
    line: usize,
}

impl TokenPosition {
    fn new(start: usize, end: usize, line: usize) -> Self {
        Self { start, end, line }
    }
}

pub struct Lexer {
    source: String,
    length: usize,
    position: usize,
    line: usize,
    line_start: usize, //NOTE: Used to determine the relative position in the current line.
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.to_string(),
            length: source.len(),
            position: 0,
            line: 1,
            line_start: 0,
        }
    }

    fn reached_end(&mut self) -> bool {
        self.position >= self.length
    }
    fn advance(&mut self) {
        if self.reached_end() {
            return;
        }

        if self.peek() == b'\n' {
            self.line += 1;
            self.line_start = self.position + 1;
        }

        self.position += 1;
    }

    fn end_of_file_token(&self) -> Token {
        Token {
            token: TokenType::EOF,
            position: TokenPosition::new(self.position, self.position, self.line),
        }
    }

    fn peek(&mut self) -> u8 {
        self.source.as_bytes()[self.position]
    }

    fn consume_whitespace(&mut self) {
        while !self.reached_end() && self.peek().is_ascii_whitespace() {
            self.advance();
        }
    }

    fn scan_macro(&mut self) -> Token {
        self.advance(); // Consume the dot symbol
        let start = self.position;

        while !self.reached_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == b'_') {
            self.advance()
        }

        let identifier = &self.source[start..self.position];

        Token {
            token: TokenType::from(identifier),
            position: TokenPosition::new(start, self.position, self.line),
        }
    }

    fn scan_string(&mut self) -> Token {
        //TODO: Currently these are multiline strings, maybe we should rather use singleline
        //strings ?
        self.advance(); //Consume the first quote symbol
        let start = self.position;

        while !self.reached_end() {
            if self.peek() == b'"' {
                self.advance();
                break;
            }

            self.advance();
        }

        if self.reached_end() && self.source.as_bytes()[self.position - 1] != b'"' {
            //TODO: This needs to be replaced with real error handling !
            panic!(
                "ERROR ({}:{}) String without valid end.",
                self.line,
                self.position.overflowing_sub(self.line_start).0,
            );
        }

        Token {
            token: TokenType::String,
            position: TokenPosition::new(start, self.position, self.line),
        }
    }

    fn scan_identifier(&mut self) -> Token {
        let start = self.position;
        while !self.reached_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == b'_') {
            self.advance()
        }

        let identifier = &self.source[start..self.position];
        dbg!(identifier);

        Token {
            token: TokenType::from(identifier),
            position: TokenPosition::new(start, self.position, self.line),
        }
    }

    fn is_binarydigit(byte: u8) -> bool {
        match byte.to_ascii_lowercase() {
            b'0' | b'1' => true,
            _ => false,
        }
    }
    fn scan_binary_number(&mut self) -> Token {
        self.advance(); //Consume the % symbol
        let start = self.position;

        while !self.reached_end() && Lexer::is_binarydigit(self.peek()) {
            self.advance();
        }

        let length = self.position - start;

        let token = match length {
            8 => TokenType::BinaryValueByte,
            16 => TokenType::BinaryValueWord,
            _ => todo!(),
        };

        Token {
            token,
            position: TokenPosition::new(start, self.position, self.line),
        }
    }
    fn scan_number(&mut self) -> Token {
        let start = self.position;

        while !self.reached_end() && self.peek().is_ascii_digit() {
            self.advance();
        }

        let length = self.position - start;

        let token = match length {
            3 => TokenType::DecimalValueByte,
            5 => TokenType::DecimalValueWord,
            _ => todo!(),
        };

        Token {
            token,
            position: TokenPosition::new(start, self.position, self.line),
        }
    }

    fn is_hexsymbol(byte: u8) -> bool {
        match byte.to_ascii_lowercase() {
            b'a'..=b'f' | b'0'..=b'9' => true,
            _ => false,
        }
    }

    fn scan_hexnumber(&mut self) -> Token {
        self.advance(); //Consume the $ symbol
        let start = self.position;

        while !self.reached_end() && Lexer::is_hexsymbol(self.peek()) {
            self.advance()
        }

        let length = self.position - start;

        let token = match length {
            2 => TokenType::HexValueByte,
            4 => TokenType::HexValueWord,
            _ => todo!(),
        };

        Token {
            token,
            position: TokenPosition::new(start, self.position, self.line),
        }
    }

    fn scan_operator(&mut self) -> Token {
        let start = self.position;
        let char = self.peek();
        self.advance();

        let tokentype = match char {
            b'(' => TokenType::OpenParen,
            b')' => TokenType::CloseParen,
            b'#' => TokenType::PoundSign,
            b',' => TokenType::Comma,
            b'.' => TokenType::Dot,
            b':' => TokenType::Colon,
            b'+' => TokenType::Plus,
            b'-' => TokenType::Minus,
            _ => unimplemented!(),
        };

        Token {
            token: tokentype,
            position: TokenPosition::new(start, self.position, self.line),
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.reached_end() {
            self.consume_whitespace();

            if self.reached_end() {
                break;
            }

            let character = self.peek();

            match character.to_ascii_lowercase() {
                b'"' => return Some(self.scan_string()),
                b'%' => return Some(self.scan_binary_number()),
                b'$' => return Some(self.scan_hexnumber()),
                b'_' | b'A'..=b'Z' | b'a'..=b'z' => return Some(self.scan_identifier()),
                b'.' => return Some(self.scan_macro()),
                b'0'..=b'9' => return Some(self.scan_number()),
                _ => return Some(self.scan_operator()),
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn expected_token(t: TokenType, start: &str, end: &str, line: usize) -> Token {
        Token {
            token: t,
            position: TokenPosition {
                start: start.len(),
                end: end.len(),
                line,
            },
        }
    }

    #[test]
    #[should_panic]
    fn test_lexer_invalid_string() {
        let source = "\"hello, world!";
        let result: Vec<Token> = Lexer::new(source).collect();
    }

    #[test]
    fn test_lexer_parentheses() {
        let source = "(some_identifier)";
        let expected = vec![
            expected_token(TokenType::OpenParen, "", "(", 1),
            expected_token(TokenType::Identifier, "(", "(some_identifier", 1),
            expected_token(
                TokenType::CloseParen,
                "(some_identifier",
                "(some_identifier)",
                1,
            ),
            //expected_token(TokenType::EOF, "(some_identifier)", "(some_identifier)", 1),
        ];

        let result: Vec<Token> = Lexer::new(source).collect();
        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_lexer_string() {
        let source = "\"hello, world!\"";

        let expected = vec![
            expected_token(TokenType::String, "\"", "\"hello, world!\"", 1),
            //expected_token(TokenType::EOF, "\"hello, world!\"", "\"hello, world!\"", 1),
        ];

        let result: Vec<Token> = Lexer::new(source).collect();
        assert_eq!(result[..], expected[..]);
    }
    #[test]
    fn test_lexer_symbol() {
        let source = "symbol";

        let expected = vec![
            expected_token(TokenType::Identifier, "", "symbol", 1),
            //expected_token(TokenType::EOF, "symbol", "symbol", 1),
        ];

        let result: Vec<Token> = Lexer::new(source).collect();
        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_lexer_decimal_value() {
        let source = "255";

        let expected = vec![
            expected_token(TokenType::DecimalValueByte, "", "255", 1),
            //expected_token(TokenType::EOF, "255", "255", 1),
        ];

        let result: Vec<Token> = Lexer::new(source).collect();
        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_lexer_binary_value() {
        let source = "%01011101";

        let expected = vec![
            expected_token(TokenType::BinaryValueByte, "%", "%01011101", 1),
            //expected_token(TokenType::EOF, "%01011101", "%01011101", 1),
        ];

        let result: Vec<Token> = Lexer::new(source).collect();
        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_lexer_flags() {
        let source = "zf nf cf hf";

        let expected = vec![
            expected_token(TokenType::ZeroFlag, "", "zf", 1),
            expected_token(TokenType::NegativeFlag, "zf ", "zf nf", 1),
            expected_token(TokenType::CarryFlag, "zf nf ", "zf nf cf", 1),
            expected_token(TokenType::HalfCarryFlag, "zf nf cf ", "zf nf cf hf", 1),
            //expected_token(TokenType::EOF, "zf nf cf hf", "zf nf cf hf", 1),
        ];

        let result: Vec<Token> = Lexer::new(source).collect();
        assert_eq!(result[..], expected[..])
    }
    #[test]
    fn test_lexer_8bit_registers() {
        let source = "A B C D E H L";

        let expected = vec![
            expected_token(TokenType::RegisterA, "", "A", 1),
            expected_token(TokenType::RegisterB, "A ", "A B", 1),
            expected_token(TokenType::RegisterC, "A B ", "A B C", 1),
            expected_token(TokenType::RegisterD, "A B C ", "A B C D", 1),
            expected_token(TokenType::RegisterE, "A B C D ", "A B C D E", 1),
            expected_token(TokenType::RegisterH, "A B C D E ", "A B C D E H", 1),
            expected_token(TokenType::RegisterL, "A B C D E H ", "A B C D E H L", 1),
            //expected_token(TokenType::EOF, "A B C D E H L", "A B C D E H L", 1),
        ];

        let result: Vec<Token> = Lexer::new(source).collect();
        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_lexer_16bit_registers() {
        let source = "AF BC DE HL SP";
        let expected = vec![
            expected_token(TokenType::RegisterAF, "", "AF", 1),
            expected_token(TokenType::RegisterBC, "AF ", "AF BC", 1),
            expected_token(TokenType::RegisterDE, "AF BC ", "AF BC DE", 1),
            expected_token(TokenType::RegisterHL, "AF BC DE ", "AF BC DE HL", 1),
            expected_token(TokenType::RegisterSP, "AF BC DE HL ", "AF BC DE HL SP", 1),
            //expected_token(TokenType::EOF, "AF BC DE HL SP", "AF BC DE HL SP", 1),
        ];

        let result: Vec<Token> = Lexer::new(source).collect();
        assert_eq!(&result[..], &expected[..]);
    }
    #[test]
    fn test_lexer_simple_line_lower_case() {
        let source = "ld B, #$FF";
        //TODO: Refactor these into their own tests !
        //let source_b = "LD b, #$ff";
        //let source_c = "lD B, #$fF";
        let expected = vec![
            expected_token(TokenType::Ld, "", "ld", 1),
            expected_token(TokenType::RegisterB, "ld ", "ld B", 1),
            expected_token(TokenType::Comma, "ld B", "ld B,", 1),
            expected_token(TokenType::PoundSign, "ld B, ", "ld B, #", 1),
            expected_token(TokenType::HexValueByte, "ld B, #$", "ld B, #$FF", 1),
            //expected_token(TokenType::EOF, "ld B, #$FF", "ld B, #$FF", 1),
        ];

        let result: Vec<Token> = Lexer::new(&source).collect();
        //let result_b = Lexer::new(&source_b).tokenize();
        //let result_c = Lexer::new(&source_c).tokenize();

        assert_eq!(result[..], expected[..]);
        //assert_eq!(result_b[..], expected[..]);
        //assert_eq!(result_c[..], expected[..]);
    }

    #[test]
    fn test_lexer_multiline() {
        let source = "main: ld A, #$02\nloop:\ndec A\njr zf,$FE\nhalt";
        let expected = vec![
            expected_token(TokenType::Identifier, "", "main", 1),
            expected_token(TokenType::Colon, "main", "main:", 1),
            expected_token(TokenType::Ld, "main: ", "main: ld", 1),
            expected_token(TokenType::RegisterA, "main: ld ", "main: ld A", 1),
            expected_token(TokenType::Comma, "main: ld A", "main: ld A,", 1),
            expected_token(TokenType::PoundSign, "main: ld A, ", "main: ld A, #", 1),
            expected_token(
                TokenType::HexValueByte,
                "main: ld A, #$",
                "main: ld A, #$02",
                1,
            ),
            expected_token(
                TokenType::Identifier,
                "main: ld A, #$02\n",
                "main: ld A, #$02\nloop",
                2,
            ),
            expected_token(
                TokenType::Colon,
                "main: ld A, #$02\nloop",
                "main: ld A, #$02\nloop:",
                2,
            ),
            expected_token(
                TokenType::Dec,
                "main: ld A, #$02\nloop:\n",
                "main: ld A, #$02\nloop:\ndec",
                3,
            ),
            expected_token(
                TokenType::RegisterA,
                "main: ld A, #$02\nloop:\ndec ",
                "main: ld A, #$02\nloop:\ndec A",
                3,
            ),
            expected_token(
                TokenType::Jr,
                "main: ld A, #$02\nloop:\ndec A\n",
                "main: ld A, #$02\nloop:\ndec A\njr",
                4,
            ),
            expected_token(
                TokenType::ZeroFlag,
                "main: ld A, #$02\nloop:\ndec A\njr ",
                "main: ld A, #$02\nloop:\ndec A\njr zf",
                4,
            ),
            expected_token(
                TokenType::Comma,
                "main: ld A, #$02\nloop:\ndec A\njr zf",
                "main: ld A, #$02\nloop:\ndec A\njr zf,",
                4,
            ),
            expected_token(
                TokenType::HexValueByte,
                "main: ld A, #$02\nloop:\ndec A\njr zf,$",
                "main: ld A, #$02\nloop:\ndec A\njr zf,$FE",
                4,
            ),
            expected_token(
                TokenType::Halt,
                "main: ld A, #$02\nloop:\ndec A\njr zf,$FE\n",
                "main: ld A, #$02\nloop:\ndec A\njr zf,$FE\nhalt",
                5,
            ),
            /*expected_token(
                TokenType::EOF,
                "main: ld A, #$02\nloop:dec A\njr zf,$FE\nhalt",
                "main: ld A, #$02\nloop:dec A\njr zf,$FE\nhalt",
                4,
            ),*/
        ];

        let result: Vec<Token> = Lexer::new(source).collect();
        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_macro_definition() {
        let source = "byte_array .dba $FF,$00";

        let expected = vec![
            expected_token(TokenType::Identifier, "", "byte_array", 1),
            expected_token(
                TokenType::MacroDefineByteArray,
                "byte_array .",
                "byte_array .dba",
                1,
            ),
            expected_token(
                TokenType::HexValueByte,
                "byte_array .dba $",
                "byte_array .dba $FF",
                1,
            ),
            expected_token(
                TokenType::Comma,
                "byte_array .dba $FF",
                "byte_array .dba $FF,",
                1,
            ),
            expected_token(
                TokenType::HexValueByte,
                "byte_array .dba $FF,$",
                "byte_array .dba $FF,$FF",
                1,
            ),
            /*expected_token(
                TokenType::EOF,
                "byte_array .dba $FF,$FF",
                "byte_array .dba $FF,$FF",
                1,
            ),*/
        ];

        let result: Vec<Token> = Lexer::new(&source).collect();
        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_label_definition() {
        let source = "some_label:";
        let expected = vec![
            expected_token(TokenType::Identifier, "", "some_label", 1),
            expected_token(TokenType::Colon, "some_label", "some_label:", 1),
            //expected_token(TokenType::EOF, "some_label:", "some_label:", 1),
        ];

        let result: Vec<Token> = Lexer::new(&source).collect();

        assert_eq!(result[..], expected[..]);
    }
}
