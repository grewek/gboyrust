use std::collections::HashMap;
#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum TokenType {
    //Symbols
    OpenParen,
    CloseParen,
    DollarSign,
    PoundSign,
    Comma,
    Dot,
    Colon,

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

    //Value Definitions
    MacroDefineByte,
    MacroDefineWord,
    MacroDefineDWord,
    MacroDefineByteArray,

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
    HexValue,
    DecimalValue,
    BinaryValue,
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

            "db" => TokenType::MacroDefineByte,
            "dw" => TokenType::MacroDefineWord,
            "dd" => TokenType::MacroDefineDWord,
            "dba" => TokenType::MacroDefineByteArray,

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

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Token {
    token: TokenType,
    position: TokenPosition,
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
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

struct Lexer {
    source: String,
    length: usize,
    position: usize,
    line: usize,
    line_start: usize, //NOTE: Used to determine the relative position in the current line.
}

impl Lexer {
    fn new(source: &str) -> Self {
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

    fn tokenize(&mut self) -> Vec<Token> {
        let mut result = vec![];

        while !self.reached_end() {
            self.consume_whitespace();
            let character = self.peek();

            let token = match character.to_ascii_lowercase() {
                b'$' => self.scan_hexnumber(),
                b'_' | b'A'..=b'Z' | b'a'..=b'z' => self.scan_identifier(),
                b'.' => self.scan_macro(),
                b'0'..=b'9' => self.scan_number(),
                _ => self.scan_operator(),
            };

            result.push(token);
        }

        result.push(self.end_of_file_token());

        result
    }

    fn scan_macro(&mut self) -> Token {
        self.advance();
        let start = self.position;

        while !self.reached_end() && self.peek().is_ascii_alphanumeric() || self.peek() == b'_' {
            self.advance()
        }

        let identifier = &self.source[start..self.position];

        Token {
            token: TokenType::from(identifier),
            position: TokenPosition::new(start, self.position, self.line),
        }
    }

    fn scan_identifier(&mut self) -> Token {
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

    fn scan_number(&mut self) -> Token {
        let start = self.position;

        while !self.reached_end() && self.peek().is_ascii_digit() {
            if self.peek().is_ascii_whitespace() || self.peek().is_ascii_control() {
                break;
            }

            self.advance();
        }

        Token {
            token: TokenType::DecimalValue,
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
        self.advance();
        let start = self.position;

        while !self.reached_end() && Lexer::is_hexsymbol(self.peek()) {
            self.advance()
        }

        Token {
            token: TokenType::HexValue,
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
            _ => unimplemented!(),
        };

        Token {
            token: tokentype,
            position: TokenPosition::new(start, self.position, self.line),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer_symbol() {
        let source = "symbol";

        let expected = vec![
            Token {
                token: TokenType::Identifier,
                position: TokenPosition {
                    start: 0,
                    end: 6,
                    line: 1,
                },
            },
            Token {
                token: TokenType::EOF,
                position: TokenPosition {
                    start: 6,
                    end: 6,
                    line: 1,
                },
            },
        ];

        let result = Lexer::new(source).tokenize();

        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_lexer_decimal_value() {
        let source = "255";

        let expected = vec![
            Token {
                token: TokenType::DecimalValue,
                position: TokenPosition {
                    start: 0,
                    end: 3,
                    line: 1,
                },
            },
            Token {
                token: TokenType::EOF,
                position: TokenPosition {
                    start: 3,
                    end: 3,
                    line: 1,
                },
            },
        ];

        let result = Lexer::new(source).tokenize();

        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_lexer_8bit_registers() {
        let source = "A B C D E H L";

        let expected = vec![
            Token {
                token: TokenType::RegisterA,
                position: TokenPosition {
                    start: 0,
                    end: 1,
                    line: 1,
                },
            },
            Token {
                token: TokenType::RegisterB,
                position: TokenPosition {
                    start: 2,
                    end: 3,
                    line: 1,
                },
            },
            Token {
                token: TokenType::RegisterC,
                position: TokenPosition {
                    start: 4,
                    end: 5,
                    line: 1,
                },
            },
            Token {
                token: TokenType::RegisterD,
                position: TokenPosition {
                    start: 6,
                    end: 7,
                    line: 1,
                },
            },
            Token {
                token: TokenType::RegisterE,
                position: TokenPosition {
                    start: 8,
                    end: 9,
                    line: 1,
                },
            },
            Token {
                token: TokenType::RegisterH,
                position: TokenPosition {
                    start: 10,
                    end: 11,
                    line: 1,
                },
            },
            Token {
                token: TokenType::RegisterL,
                position: TokenPosition {
                    start: 12,
                    end: 13,
                    line: 1,
                },
            },
            Token {
                token: TokenType::EOF,
                position: TokenPosition {
                    start: 13,
                    end: 13,
                    line: 1,
                },
            },
        ];

        let result = Lexer::new(source).tokenize();

        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_lexer_16bit_registers() {
        let source = "AF BC DE HL SP";
        let expected = vec![
            Token {
                token: TokenType::RegisterAF,
                position: TokenPosition {
                    start: 0,
                    end: 2,
                    line: 1,
                },
            },
            Token {
                token: TokenType::RegisterBC,
                position: TokenPosition {
                    start: 3,
                    end: 5,
                    line: 1,
                },
            },
            Token {
                token: TokenType::RegisterDE,
                position: TokenPosition {
                    start: 6,
                    end: 8,
                    line: 1,
                },
            },
            Token {
                token: TokenType::RegisterHL,
                position: TokenPosition {
                    start: 9,
                    end: 11,
                    line: 1,
                },
            },
            Token {
                token: TokenType::RegisterSP,
                position: TokenPosition {
                    start: 12,
                    end: 14,
                    line: 1,
                },
            },
            Token {
                token: TokenType::EOF,
                position: TokenPosition {
                    start: 14,
                    end: 14,
                    line: 1,
                },
            },
        ];

        let result = Lexer::new(source).tokenize();

        assert_eq!(&result[..], &expected[..]);
    }
    #[test]
    fn test_lexer_simple_line_lower_case() {
        let source = "ld B, #$FF";
        //TODO: Refactor these into their own tests !
        //let source_b = "LD b, #$ff";
        //let source_c = "lD B, #$fF";
        let expected = vec![
            Token {
                token: TokenType::Ld,
                position: TokenPosition {
                    start: 0,
                    end: 2,
                    line: 1,
                },
            },
            Token {
                token: TokenType::RegisterB,
                position: TokenPosition {
                    start: 3,
                    end: 4,
                    line: 1,
                },
            },
            Token {
                token: TokenType::Comma,
                position: TokenPosition {
                    start: 4,
                    end: 5,
                    line: 1,
                },
            },
            Token {
                token: TokenType::PoundSign,
                position: TokenPosition {
                    start: 6,
                    end: 7,
                    line: 1,
                },
            },
            Token {
                token: TokenType::HexValue,
                position: TokenPosition {
                    start: 8,
                    end: 10,
                    line: 1,
                },
            },
            Token {
                token: TokenType::EOF,
                position: TokenPosition {
                    start: 10,
                    end: 10,
                    line: 1,
                },
            },
        ];

        let result = Lexer::new(&source).tokenize();
        //let result_b = Lexer::new(&source_b).tokenize();
        //let result_c = Lexer::new(&source_c).tokenize();

        assert_eq!(result[..], expected[..]);
        //assert_eq!(result_b[..], expected[..]);
        //assert_eq!(result_c[..], expected[..]);
    }

    #[test]
    fn test_macro_definition() {
        let source = "byte_array .dba $FF,$00";

        let expected = vec![
            Token {
                token: TokenType::Identifier,
                position: TokenPosition {
                    start: 0,
                    end: "byte_array".len(),
                    line: 1,
                },
            },
            Token {
                token: TokenType::MacroDefineByteArray,
                position: TokenPosition {
                    start: "byte_array .".len(),
                    end: "byte_array .".len() + ("dba".len()),
                    line: 1,
                },
            },
            Token {
                token: TokenType::HexValue,
                position: TokenPosition {
                    start: "byte_array .dba $".len(),
                    end: "byte_array .dba $FF".len(),
                    line: 1,
                },
            },
            Token {
                token: TokenType::Comma,
                position: TokenPosition {
                    start: "byte_array .dba $FF".len(),
                    end: "byte_array.dba $FF,$".len(),
                    line: 1,
                },
            },
            Token {
                token: TokenType::HexValue,
                position: TokenPosition {
                    start: "byte_array .dba $FF,$".len(),
                    end: "byte_array .dba $FF,$FF".len(),
                    line: 1,
                },
            },
            Token {
                token: TokenType::EOF,
                position: TokenPosition {
                    start: "byte_array .dba $FF,$FF".len(),
                    end: "byte_array .dba $FF,$FF".len(),
                    line: 1,
                },
            },
        ];

        let result = Lexer::new(&source).tokenize();
        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_label_definition() {
        let source = "some_label:";
        let expected = vec![
            Token {
                token: TokenType::Identifier,
                position: TokenPosition {
                    start: "".len(),
                    end: "some_label".len(),
                    line: 1,
                },
            },
            Token {
                token: TokenType::Comma,
                position: TokenPosition {
                    start: "some_label".len(),
                    end: "some_label:".len(),
                    line: 1,
                },
            },
        ];
    }
}
