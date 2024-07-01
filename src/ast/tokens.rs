#[derive(Debug, PartialEq, Clone)]
pub enum TokenEnum {
  // Single-character tokens.
  LeftParen,  // '('
  RightParen, // ')'
  LeftBrace,  // '{'
  RightBrace, // '}'
  Comma,      // ','
  Dot,        // '.'
  Minus,      // '-'
  Plus,       // '+'
  Semicolon,  // ';'
  Slash,      // '/'
  Star,       // '*'

  // One or two character tokens.
  Bang,         // '!'
  BangEqual,    // '!='
  Equal,        // '='
  EqualEqual,   // '=='
  Greater,      // '>'
  GreaterEqual, // '>='
  Less,         // '<'
  LessEqual,    // '<='

  // Literals.
  Identifier,    // 'identifier'
  StringLiteral, // '"string"'
  NumberLiteral, // '1234' // '1234.5678'

  // Keywords.
  And,    // 'and'
  Class,  // 'class'
  Else,   // 'else'
  False,  // 'false'
  Fun,    // 'fun'
  For,    // 'for'
  If,     // 'if'
  Nil,    // 'nil'
  Or,     // 'or'
  Print,  // 'print'
  Return, // 'return'
  Super,  // 'super'
  This,   // 'this'
  True,   // 'true
}

#[derive(Debug, PartialEq)]
pub struct Token {
  pub kind: TokenEnum,
  pub lexeme: String,
  pub literal: String,
  pub line: usize,
}

impl Token {
  pub fn new(kind: TokenEnum, lexeme: String, literal: String, line: usize) -> Token {
    Token { kind, lexeme, literal, line }
  }

  pub fn to_string(&self) -> String {
    format!("{:?} {} {}", self.kind, self.lexeme, self.literal)
  }
}
