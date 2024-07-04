// use std::collections::HashMap;

use crate::ast::tokens::{Token, TokenEnum};
use crate::diagnostics::error_handler::ErrorHandler;

pub struct Scanner {
  pub tokens: Vec<Token>,
  pub error_handler: ErrorHandler,
  raw: String,   // the source code
  cursor: usize, // index of the current character
  line: usize,   // current line number
  start: usize,  // index of the start of the current token
                 // keywords: HashMap<&'static str, TokenEnum>,
}

fn check_keyword(text: &str) -> Option<TokenEnum> {
  let kind = match text {
    "and" => TokenEnum::And,
    "class" => TokenEnum::Class,
    "else" => TokenEnum::Else,
    "false" => TokenEnum::False,
    "for" => TokenEnum::For,
    "fun" => TokenEnum::Fun,
    "if" => TokenEnum::If,
    "nil" => TokenEnum::Nil,
    "or" => TokenEnum::Or,
    "print" => TokenEnum::Print,
    "return" => TokenEnum::Return,
    "super" => TokenEnum::Super,
    "this" => TokenEnum::This,
    "true" => TokenEnum::True,
    // equivalent to None (bacause is not possible to return diferent types)
    _ => TokenEnum::Identifier,
  };

  if kind == TokenEnum::Identifier {
    return None;
  }
  return Some(kind);
}

impl Scanner {
  pub fn new(raw: String, path_name: &str) -> Self {
    // let keywords = HashMap::from([
    //   ("and", TokenEnum::And),
    //   ("class", TokenEnum::Class),
    //   ("else", TokenEnum::Else),
    //   ("false", TokenEnum::False),
    //   ("for", TokenEnum::For),
    //   ("fun", TokenEnum::Fun),
    //   ("if", TokenEnum::If),
    //   ("nil", TokenEnum::Nil),
    //   ("or", TokenEnum::Or),
    //   ("print", TokenEnum::Print),
    //   ("return", TokenEnum::Return),
    //   ("super", TokenEnum::Super),
    //   ("this", TokenEnum::This),
    //   ("true", TokenEnum::True),
    // ]);
    //
    let error_handler = ErrorHandler::new(path_name);
    Scanner { tokens: vec![], raw, cursor: 0, line: 1, start: 0, error_handler }
  }
  pub fn scan_tokens(&mut self) {
    while !self.is_at_end() {
      self.start = self.cursor;
      self.scan_token();
    }
    self.add_token(TokenEnum::EndOfFile);
  }

  pub fn scan_token(&mut self) {
    let character = self.advance();
    match character {
      '(' => self.add_token(TokenEnum::LeftParen),
      ')' => self.add_token(TokenEnum::RightParen),
      '{' => self.add_token(TokenEnum::LeftBrace),
      '}' => self.add_token(TokenEnum::RightBrace),
      ',' => self.add_token(TokenEnum::Comma),
      '.' => self.add_token(TokenEnum::Dot),
      '-' => self.add_token(TokenEnum::Minus),
      '+' => self.add_token(TokenEnum::Plus),
      ';' => self.add_token(TokenEnum::Semicolon),
      '/' => self.add_token(TokenEnum::Slash),
      '*' => self.add_token(TokenEnum::Star),
      '!' => self.add_token(TokenEnum::Bang),
      // One or two character tokens.'
      '=' => {
        if self.is_match('=') {
          self.add_token(TokenEnum::EqualEqual);
        } else {
          self.add_token(TokenEnum::Equal);
        }
      }
      '>' => {
        if self.is_match('=') {
          self.add_token(TokenEnum::GreaterEqual);
        } else {
          self.add_token(TokenEnum::Greater);
        }
      }
      '<' => {
        if self.is_match('=') {
          self.add_token(TokenEnum::LessEqual);
        } else {
          self.add_token(TokenEnum::Less);
        }
      }

      // or reserved words
      'o' => {
        if self.is_match('r') {
          self.add_token(TokenEnum::Or);
        }
      }
      // string literals
      '"' => self.scan_string(),
      // reversed order

      // comments, whitespace, newlines, digits and unknown characters
      current_character => {
        if current_character == '/' {
          if !self.is_match('/') {
            self.add_token(TokenEnum::Slash);
            return;
          };
          while self.peek_char() != '\n' && !self.is_at_end() {
            self.advance();
          }
          return;
        }
        if current_character == '\n' {
          self.line += 1;
          return;
        }

        if current_character.is_whitespace() {
          return;
        }

        // digits
        if self.is_digit(current_character) {
          self.scan_number();
          return;
        }
        // identifiers
        if self.is_alphanumeric(current_character) {
          self.scan_identifier();
          return;
        }
        let message = format!("Unexpected character: {}", current_character);
        self.error_handler.error(self.line, &message);
      }
    }
  }

  pub fn scan_string(&mut self) {
    while !self.is_at_end() && self.peek_char() != '"' {
      if self.peek_char() == '\n' {
        self.line += 1;
      }
      self.advance();
    }
    if self.is_at_end() {
      self.error_handler.error(self.line, "Unterminated string.");
    }
    // the closing ".
    self.advance();

    let literal = self.raw[self.start + 1..self.cursor - 1].to_string();
    self.add_token_with_literal(TokenEnum::StringLiteral, literal);
  }

  pub fn scan_number(&mut self) {
    let mut current_character = self.peek_char();
    while self.is_digit(current_character) {
      self.advance();
      current_character = self.peek_char();
    }
    let mut next_character = self.peek_next_char();
    if self.peek_char() == '.' && self.is_digit(next_character) {
      self.advance();
      next_character = self.peek_char();
      while self.is_digit(next_character) {
        self.advance();
        next_character = self.peek_char();
      }
    }
    // if self.is_match('e') || self.is_match('E') {
    //   self.advance();
    //   if self.is_match('+') || self.is_match('-') {
    //     self.advance();
    //   }
    // }
    //
    let literal = self.raw[self.start..self.cursor].to_string();
    self.add_token_with_literal(TokenEnum::NumberLiteral, literal);
  }

  pub fn scan_identifier(&mut self) {
    let mut current_character = self.peek_char();
    while self.is_alphanumeric(current_character) {
      self.advance();
      current_character = self.peek_char();
    }
    let text = self.raw[self.start..self.cursor].to_string();
    let kind = self.get_reserved_keyword_or_return_identifier(&text);
    self.add_token(kind);
  }
  pub fn advance(&mut self) -> char {
    let character = self.peek_char();
    self.cursor += 1;
    character
  }

  pub fn peek_char(&mut self) -> char {
    if self.is_at_end() {
      return '\0';
    }
    self.raw.chars().nth(self.cursor).unwrap()
  }

  pub fn peek_next_char(&mut self) -> char {
    if self.is_at_end() {
      return '\0';
    }
    self.raw.chars().nth(self.cursor + 1).unwrap()
  }

  pub fn add_token(&mut self, kind: TokenEnum) {
    let lexeme = self.raw[self.start..self.cursor].to_string();
    let literal = String::from("");
    self.tokens.push(Token::new(kind, lexeme, literal, self.line));
  }

  pub fn add_token_with_literal(&mut self, kind: TokenEnum, literal: String) {
    let lexeme = self.raw[self.start..self.cursor].to_string();
    self.tokens.push(Token::new(kind, lexeme, literal, self.line));
  }

  pub fn get_reserved_keyword_or_return_identifier(&mut self, text: &str) -> TokenEnum {
    if let Some(keyword) = check_keyword(text) {
      return keyword;
    }
    return TokenEnum::Identifier;
  }

  // boolean's methods

  pub fn is_at_end(&mut self) -> bool {
    self.cursor >= self.raw.len()
  }

  pub fn is_match(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      return false;
    }
    let actual = self.raw.chars().nth(self.cursor).unwrap();
    if actual != expected {
      return false;
    }
    self.cursor += 1;
    true
  }

  pub fn is_digit(&mut self, character: char) -> bool {
    return "0123456789".contains(character);
  }

  pub fn is_alphabetic(&mut self, character: char) -> bool {
    return character >= 'a' && character <= 'z' || character >= 'A' && character <= 'Z' || character == '_';
  }

  pub fn is_alphanumeric(&mut self, character: char) -> bool {
    return self.is_alphabetic(character) || self.is_digit(character);
  }

  pub fn is_keyword(&mut self, text: &str) -> bool {
    return check_keyword(text).is_some();
  }
}
