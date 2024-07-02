use std::process;

use crate::ast::tokens::{Token, TokenEnum};
use crate::shared::constants::ERROR_EXIT_CODE;

pub struct ErrorHandler {
  pub had_error: bool,
}

impl Default for ErrorHandler {
  fn default() -> Self {
    Self::new()
  }
}
impl ErrorHandler {
  pub fn new() -> ErrorHandler {
    ErrorHandler { had_error: false }
  }

  pub fn report(&mut self, line: usize, _where: &str, message: &str) {
    let error = format!("[line {}] Error {}: {}", line, _where, message);
    println!("{}", error);
    self.had_error = true;
  }

  pub fn error(&mut self, line: usize, message: &str) {
    self.report(line, "<unknown>", message);
  }
}
