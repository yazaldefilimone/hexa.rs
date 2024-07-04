use crate::ast::tokens::Token;
use error_handler::ErrorHandler;

pub mod error_handler;

pub enum DiagnosticKind {
  Error,
  Warning,
  Info,
}

pub struct Diagnostics {
  pub error_handler: ErrorHandler,
}

impl Diagnostics {
  pub fn new(path_name: &str) -> Self {
    Diagnostics { error_handler: ErrorHandler::new(path_name) }
  }

  pub fn report_token(&mut self, token: &Token, message: &str) {
    self.error_handler.report_rich(token, message);
  }

  pub fn report(&mut self, line: usize, _where: &str, message: &str) {
    self.error_handler.report(line, _where, message);
  }
  pub fn error(&mut self, line: usize, message: &str) {
    self.error_handler.error(line, message);
  }
}
