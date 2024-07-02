use std::process;

use crate::ast;
use crate::ast::tokens::{Token, TokenEnum};
use crate::scanner::Scanner;
use crate::shared::constants::ERROR_EXIT_CODE;

pub struct Parser<'a> {
  current_token_pointer: usize,
  scanner: &'a mut Scanner,
}

impl<'a> Parser<'a> {
  pub fn new(scanner: &mut Scanner) -> Parser {
    scanner.scan_tokens();
    Parser { current_token_pointer: 0, scanner }
  }

  pub fn parse(&mut self) {}

  pub fn parse_expression(&mut self) -> ast::Expression {
    return self.parse_equality();
  }

  // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
  pub fn parse_equality(&mut self) -> ast::Expression {
    let mut left_expression = self.parse_comparison();

    while self.is_match(TokenEnum::BangEqual) || self.is_match(TokenEnum::EqualEqual) {
      let kind_operator = self.get_previous_token().kind.clone().clone();
      let right_expression = self.parse_comparison();
      let binary_expression = ast::BinaryExpression::new(kind_operator, left_expression, right_expression);
      left_expression = ast::Expression::BinaryExpression(binary_expression);
    }
    return left_expression;
  }

  // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
  pub fn parse_comparison(&mut self) -> ast::Expression {
    let mut left_expression = self.parse_term();
    let matches = &[
      TokenEnum::Greater,
      TokenEnum::Less,
      TokenEnum::GreaterEqual,
      TokenEnum::LessEqual,
    ];

    while self.is_match_many(matches) {
      let kind_operator = self.get_previous_token().kind.clone();
      let right_expression = self.parse_term();
      let binary_expression = ast::BinaryExpression::new(kind_operator, left_expression, right_expression);
      left_expression = ast::Expression::BinaryExpression(binary_expression);
    }
    return left_expression;
  }

  pub fn parse_term(&mut self) -> ast::Expression {
    let mut left_expression = self.parse_factor();
    let matches = &[TokenEnum::Plus, TokenEnum::Minus];
    while self.is_match_many(matches) {
      let kind_operator = self.get_previous_token().kind.clone();
      let right_expression = self.parse_factor();
      let binary_expression = ast::BinaryExpression::new(kind_operator, left_expression, right_expression);
      left_expression = ast::Expression::BinaryExpression(binary_expression);
    }

    return left_expression;
  }

  pub fn parse_factor(&mut self) -> ast::Expression {
    let mut left_expression = self.parse_unary();
    let matches = &[TokenEnum::Slash, TokenEnum::Star];
    while self.is_match_many(matches) {
      let kind_operator = self.get_previous_token().kind.clone();
      let right_expression = self.parse_unary();
      let binary_expression = ast::BinaryExpression::new(kind_operator, left_expression, right_expression);
      left_expression = ast::Expression::BinaryExpression(binary_expression);
    }

    return left_expression;
  }
  /*
  unary          → ( "!" | "-" ) unary
                 | primary ;
  */

  pub fn parse_unary(&mut self) -> ast::Expression {
    if self.is_match(TokenEnum::Bang) {
      self.consume_expected(TokenEnum::Bang, "Expected '!' after expression.");
      let right_expression = self.parse_unary();
      let unary_expression = ast::UnaryExpression::new(TokenEnum::Bang, right_expression);
      return ast::Expression::UnaryExpression(unary_expression);
    }

    if self.is_match(TokenEnum::Minus) {
      self.consume_expected(TokenEnum::Minus, "Expected '-' after expression.");
      let right_expression = self.parse_unary();
      let unary_expression = ast::UnaryExpression::new(TokenEnum::Minus, right_expression);
      return ast::Expression::UnaryExpression(unary_expression);
    }
    return self.parse_primary();
  }

  /*
  primary        → NUMBER | STRING | "true" | "false" | "nil"
                 | "(" expression ")" ;
  */
  pub fn parse_primary(&mut self) -> ast::Expression {
    // boolean's
    if self.is_match(TokenEnum::False) {
      let literal = ast::Literal::BooleanLiteral(true);
      return ast::Expression::Literal(literal);
    }
    if self.is_match(TokenEnum::True) {
      let literal = ast::Literal::BooleanLiteral(false);
      return ast::Expression::Literal(literal);
    }

    if self.is_match(TokenEnum::Nil) {
      let literal = ast::Literal::NullLiteral;
      return ast::Expression::Literal(literal);
    }

    if self.is_match_many(&vec![TokenEnum::StringLiteral, TokenEnum::NumberLiteral]) {
      let previous_token = self.get_previous_token();
      if previous_token.kind == TokenEnum::StringLiteral {
        let literal = ast::Literal::StringLiteral(previous_token.literal.clone());
        return ast::Expression::Literal(literal);
      }
      let literal = ast::Literal::NumberLiteral(previous_token.literal.parse::<f64>().unwrap());
      return ast::Expression::Literal(literal);
    }

    if self.is_match(TokenEnum::LeftParen) {
      let expression = self.parse_expression();
      self.consume_expected(TokenEnum::RightParen, "Expected ')' after expression.");
      let expression = ast::GroupingExpression::new(expression);
      return ast::Expression::GroupingExpression(expression);
    }
    let token = self.peek().clone();
    self.error(token, "Expected expression.");
    process::exit(ERROR_EXIT_CODE);
  }

  // helper methods
  fn advance(&mut self) -> &Token {
    if !self.is_at_end() {
      self.current_token_pointer += 1;
    }
    return self.get_previous_token();
  }

  fn get_previous_token(&mut self) -> &Token {
    let token = &self.scanner.tokens[self.current_token_pointer - 1];
    return token;
  }

  fn peek(&self) -> &Token {
    &self.scanner.tokens[self.current_token_pointer]
  }

  fn peek_next(&self) -> &Token {
    &self.scanner.tokens[self.current_token_pointer + 1]
  }

  fn is_at_end(&self) -> bool {
    self.current_token_pointer >= self.scanner.tokens.len()
  }

  fn is_match(&self, expected: TokenEnum) -> bool {
    if self.is_at_end() {
      return false;
    }
    return self.peek().kind.clone() == expected;
  }

  fn is_match_many(&self, expecteds: &[TokenEnum]) -> bool {
    if self.is_at_end() {
      return false;
    }
    return expecteds.iter().any(|expected| self.peek().kind.clone() == *expected);
  }

  fn is_match_next(&self, expected: TokenEnum) -> bool {
    if self.is_at_end() {
      return false;
    }
    return self.peek_next().kind.clone() == expected;
  }

  fn consume_expected(&mut self, expected: TokenEnum, message: &str) {
    if !self.is_match(expected) {
      let token = self.peek().clone();
      self.error(token, message);
      process::exit(ERROR_EXIT_CODE);
    }
    self.advance();
  }

  // error handling
  fn error(&mut self, token: Token, message: &str) {
    if token.kind == TokenEnum::EndOfFile {
      self.scanner.error_handler.report(token.line, " at end", message);
      // todo: exit with error code
      process::exit(ERROR_EXIT_CODE);
    }
    let lexeme_formated = &format!(" at '{}'", token.lexeme);
    self.scanner.error_handler.report(token.line, lexeme_formated, message);
    // todo: exit with error code
    process::exit(ERROR_EXIT_CODE);
  }
}
