use crate::assembler::lexer;
use crate::assembler::lexer::{Token, TokenType};

struct Parser {
    source_ref: String,
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum ByteReg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl From<TokenType> for ByteReg {
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::RegisterA => ByteReg::A,
            TokenType::RegisterB => ByteReg::B,
            TokenType::RegisterC => ByteReg::C,
            TokenType::RegisterD => ByteReg::D,
            TokenType::RegisterE => ByteReg::E,
            TokenType::RegisterH => ByteReg::H,
            TokenType::RegisterL => ByteReg::L,
            _ => panic!("This tokentype cannot be converted into a 8bit register"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum WordReg {
    AF,
    BC,
    DE,
    HL,
}

impl From<TokenType> for WordReg {
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::RegisterAF => WordReg::AF,
            TokenType::RegisterBC => WordReg::BC,
            TokenType::RegisterDE => WordReg::DE,
            TokenType::RegisterHL => WordReg::HL,
            _ => panic!("Tokentype not valid in this context !"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Condition {
    Unconditional,
    OnZeroSet,
    OnNegativeSet,
    OnCarrySet,
    OnHalfCarrySet,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Target {
    Identifier(String),
    MemReg(WordReg),
    MemRegInc(WordReg),
    MemRegDec(WordReg),

    ByteReg(ByteReg),
    WordReg(WordReg),

    Address16(u16),

    Data8(u8),
    Data16(u16),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Macro {
    RenameByteReg(String, ByteReg),
    RenameWordReg(String, WordReg),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Function {
    Function(String, Option<Vec<Macro>>, Vec<Command>),
    FunEnd,
}

struct RenamingByteReg {
    alias: String,
    register: ByteReg,
}

struct Program {
    macros: Vec<Macro>,
    source: Vec<Command>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Command {
    Lable(String),
    Load(Target, Target),
    Add(Target, Target),
    Adc(Target, Target),
    Inc(Target),
    Dec(Target),
    Push(WordReg),
    Pop(WordReg),
    JumpRel(Condition, Target),
    Jump(Condition, Target),
    Call(Condition, Target),
    And(Target),
    Or(Target),
    Xor(Target),
    Sbc(Target),
    Cp(Target),
    Ret(Condition),
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct AssemblyTree {
    macros: Option<Vec<Macro>>,
    functions: Option<Vec<Function>>,
    assembly: Option<Vec<Command>>,
}

//TODO: We need to restart the whole design process this mess is getting out of hand...
//NOTE: This is probably the dumbest and messiest parser ever written...
//      maybe it will call some lovecraftian horror...
//NOTE: We need to restart and __rethink__ things from scratch this mess is not a shipping
//candidate...
impl Parser {
    //TOKENTYPE: PUSH 16bitreg
    fn new(source_ref: &str, token_stream: Vec<Token>) -> Self {
        Self {
            source_ref: source_ref.to_string(),
            tokens: token_stream,
            position: 0,
        }
    }

    fn parse_source(&mut self) -> AssemblyTree {
        let mut macros = vec![];
        let mut functions = vec![];
        let mut commands = vec![];
        match self.peek_type_top().unwrap() {
            TokenType::MacroDefineByte => macros.push(self.parse_macro()),
            TokenType::MacroDefineWord => macros.push(self.parse_macro()),
            TokenType::MacroDefineFunctionStart => functions.push(self.parse_function()),
            _ => commands.push(self.parse_assembly()),
        }

        AssemblyTree {
            macros: if macros.is_empty() {
                None
            } else {
                Some(macros)
            },
            functions: if functions.is_empty() {
                None
            } else {
                Some(functions)
            },
            assembly: if commands.is_empty() {
                None
            } else {
                Some(commands)
            },
        }
    }

    fn parse_macro(&mut self) -> Macro {
        todo!()
    }

    fn parse_assembly(&mut self) -> Command {
        todo!()
    }

    fn parse_function(&mut self) -> Function {
        self.advance();
        let function_name_token = self.advance().unwrap();
        let function_name = self.source_ref[function_name_token.repr_range()].to_string();
        dbg!(&function_name);

        let reg_renamings = self.parse_renamed_registers();

        let function_body = self.parse_macro_function_body();

        Function::Function(function_name, reg_renamings, function_body)
    }

    fn parse_macro_function_body(&mut self) -> Vec<Command> {
        let mut commands = vec![];
        loop {
            match self.peek_type_top().unwrap() {
                TokenType::MacroDefineFunctionStart => todo!(), //This should be an error ?
                TokenType::MacroDefineFunctionEnd => {
                    commands.push(Command::Ret(Condition::Unconditional));
                    break;
                }
                _ => commands.push(self.parse()),
            };
        }

        commands
    }

    fn parse_renamed_registers(&mut self) -> Option<Vec<Macro>> {
        let mut renamings = vec![];
        if self.match_token(TokenType::OpenParen) {
            self.advance();

            loop {
                match self.peek_type_top().unwrap() {
                    TokenType::RegisterA
                    | TokenType::RegisterB
                    | TokenType::RegisterC
                    | TokenType::RegisterD
                    | TokenType::RegisterE
                    | TokenType::RegisterH
                    | TokenType::RegisterL
                    | TokenType::RegisterAF
                    | TokenType::RegisterBC
                    | TokenType::RegisterDE
                    | TokenType::RegisterHL => renamings.push(self.parse_renaming()),
                    TokenType::Comma => {
                        self.advance();
                        continue;
                    }
                    TokenType::CloseParen => break,
                    _ => {
                        dbg!(self.peek_type_top().unwrap());
                        break;
                    }
                };
            }
        }

        self.advance();

        if renamings.len() == 0 {
            return None;
        }

        Some(renamings)
    }

    fn parse_renaming(&mut self) -> Macro {
        match self.peek_type_top().unwrap() {
            TokenType::RegisterA
            | TokenType::RegisterB
            | TokenType::RegisterC
            | TokenType::RegisterD
            | TokenType::RegisterE
            | TokenType::RegisterH
            | TokenType::RegisterL => {
                let target_register: ByteReg = self.advance().unwrap().tokentype().into();
                if self.match_token(TokenType::Colon) {
                    self.advance();
                }

                let renamed = self.advance().unwrap().repr_range();
                return Macro::RenameByteReg(self.source_ref[renamed].to_string(), target_register);
            }
            TokenType::RegisterAF
            | TokenType::RegisterBC
            | TokenType::RegisterDE
            | TokenType::RegisterHL => {
                let target_register: WordReg = self.advance().unwrap().tokentype().into();
                if self.match_token(TokenType::Colon) {
                    self.advance();
                }

                let renamed = self.advance().unwrap().repr_range();
                return Macro::RenameWordReg(self.source_ref[renamed].to_string(), target_register);
            }
            _ => panic!("Whhooops"),
        }
    }

    fn match_any_token(&mut self, matchable: Vec<TokenType>) -> Option<Token> {
        for tt in matchable {
            if self.match_token(tt) {
                return self.advance();
            }
        }

        None
    }

    fn parse(&mut self) -> Command {
        //FIXME: The parser will parse everything even it is not possible like
        //       pushing a 8bit register to the stack i need a better way to handle
        //       these cases !
        dbg!(self.source_ref[self.tokens[self.position].repr_range()].to_string());
        if self.match_token(TokenType::Identifier) {
            return self.parse_lable();
        }
        match self.advance().unwrap().tokentype() {
            TokenType::Ld => Command::Load(self.parse_target(), self.parse_target()),
            TokenType::Add => Command::Add(self.parse_target(), self.parse_target()),
            TokenType::Adc => Command::Adc(self.parse_target(), self.parse_target()),
            TokenType::Inc => Command::Inc(self.parse_target()),

            TokenType::Dec => Command::Dec(self.parse_target()),
            TokenType::And => Command::And(self.parse_target()),
            TokenType::Or => Command::Or(self.parse_target()),
            TokenType::Xor => Command::Xor(self.parse_target()),
            TokenType::Push => Command::Push(self.parse_16bit_register()),
            TokenType::Pop => Command::Pop(self.parse_16bit_register()),
            TokenType::Sbc => Command::Sbc(self.parse_target()),
            TokenType::Cp => Command::Cp(self.parse_target()),
            TokenType::Ret => Command::Ret(self.parse_conditional()),
            TokenType::Jr => Command::JumpRel(self.parse_conditional(), self.parse_jump_target()),
            TokenType::Jp => Command::Jump(self.parse_conditional(), self.parse_jump_target()),
            TokenType::Call => Command::Call(self.parse_conditional(), self.parse_jump_target()),
            _ => panic!("This token is not yet known and needs to be implemented!"),
        }
    }

    fn parse_lable(&mut self) -> Command {
        let lable_name = self.advance().unwrap();

        if self.match_token(TokenType::Colon) {
            self.advance();
        }

        //TODO: This is __not__ a lable definition...

        Command::Lable(self.source_ref[lable_name.repr_range()].to_string())
    }

    fn parse_conditional(&mut self) -> Condition {
        dbg!(self.peek_type_top());
        match self.peek_type_top().unwrap() {
            TokenType::ZeroFlag => {
                self.advance();
                Condition::OnZeroSet
            }
            TokenType::NegativeFlag => {
                self.advance();
                Condition::OnNegativeSet
            }
            TokenType::CarryFlag => {
                self.advance();
                Condition::OnCarrySet
            }
            TokenType::HalfCarryFlag => {
                self.advance();
                Condition::OnHalfCarrySet
            }
            _ => Condition::Unconditional,
        }
    }

    fn parse_jump_target(&mut self) -> Target {
        if self.match_token(TokenType::Comma) {
            self.advance();
        }

        let token = self.advance().unwrap();

        match token.tokentype() {
            TokenType::Identifier => {
                Target::Identifier(self.source_ref[token.repr_range()].to_string())
            }
            TokenType::BinaryValueByte => {
                Target::Data8(self.parse_byte_value(2, token.repr_range()))
            }
            TokenType::DecimalValueByte => {
                Target::Data8(self.parse_byte_value(10, token.repr_range()))
            }
            TokenType::HexValueByte => Target::Data8(self.parse_byte_value(16, token.repr_range())),
            _ => {
                dbg!(token.tokentype());
                dbg!(self.source_ref[token.repr_range()].to_string());
                panic!("ERROR: Invalid token for current context !");
            }
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
        dbg!(self.position);
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
            TokenType::Identifier => {
                Target::Identifier(self.source_ref[token.repr_range()].to_string())
            }
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
        dbg!(token);
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
            TokenType::RegisterBC => Target::MemReg(WordReg::BC),
            TokenType::RegisterDE => Target::MemReg(WordReg::DE),
            TokenType::RegisterHL => Target::MemReg(WordReg::HL),
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
        if !(self.peek_type_top().unwrap() == to_match) {
            return false;
        }

        true
    }

    fn parse_register(&mut self) -> ByteReg {
        let register = match self.peek_type_top().unwrap() {
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

    fn peek_type_top(&self) -> Option<TokenType> {
        //TODO: Check for overflow!
        Some(self.tokens[self.position].tokentype())
    }

    fn peek_type_fwd(&self) -> Option<TokenType> {
        if self.position >= self.tokens.len() - 1 {
            return None;
        }

        Some(self.tokens[self.position + 1].tokentype())
    }

    fn match_token_advance(&mut self, to_match: TokenType) -> Option<Token> {
        if self.peek_type_top().unwrap() != to_match {
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
        let expected = Command::Inc(Target::MemReg(WordReg::HL));

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

        let expected = Command::Load(Target::ByteReg(ByteReg::A), Target::MemReg(WordReg::HL));

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

    #[test]
    fn parse_conditional_relative_jump() {
        let source = "jr zf, $FF";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::JumpRel(Condition::OnZeroSet, Target::Data8(0xFF));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_and_opcode() {
        let source = "and (hl)";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::And(Target::MemReg(WordReg::HL));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_or_opcode() {
        let source = "or (hl)";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::Or(Target::MemReg(WordReg::HL));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_xor_opcode() {
        let source = "xor (hl)";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::Xor(Target::MemReg(WordReg::HL));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_add_opcode() {
        let source = "add a, b";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse();
        let expected = Command::Add(Target::ByteReg(ByteReg::A), Target::ByteReg(ByteReg::B));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_macro_function() {
        let source = ".fun example_func (a: sum, b: index, hl: base_ptr)\ninc index\n.end";
        let token_stream: Vec<Token> = lexer::Lexer::new(&source).collect();

        let result = Parser::new(source, token_stream).parse_source();
        let expected_macros = None;
        let expected_assembly = None;
        let expected_functions = Some(vec![Function::Function(
            "example_func".to_string(),
            Some(vec![
                Macro::RenameByteReg("sum".to_string(), ByteReg::A),
                Macro::RenameByteReg("index".to_string(), ByteReg::B),
                Macro::RenameWordReg("base_ptr".to_string(), WordReg::HL),
            ]),
            vec![
                Command::Inc(Target::Identifier("index".to_string())),
                Command::Ret(Condition::Unconditional),
            ],
        )]);

        assert_eq!(result.functions, expected_functions);
        assert_eq!(result.macros, expected_macros);
        assert_eq!(result.assembly, expected_assembly);
    }
}
