use crate::ast;
use crate::ast::tokens::{Token, TokenEnum};
use crate::diagnostics::error_handler::ErrorHandler;
use crate::values::Value;
use crate::visitor::Visitor;

pub struct Evaluator {
  pub error_handler: ErrorHandler,
}

impl Visitor<Value> for Evaluator {
  fn visit(&mut self, ast: &ast::Expression) -> Value {
    match ast {
      ast::Expression::BinaryExpression(binary_expression) => self.visit_binary_expression(binary_expression),
      ast::Expression::UnaryExpression(unary_expression) => self.visit_unary_expression(unary_expression),
      ast::Expression::GroupingExpression(grouping_expression) => self.visit_grouping_expression(grouping_expression),
      ast::Expression::Literal(literal) => self.visit_literal(literal),
    }
  }

  fn visit_binary_expression(&mut self, binary_expression: &ast::BinaryExpression) -> Value {
    let left = self.evaluate(&binary_expression.left);
    let right = self.evaluate(&binary_expression.right);
    if !self.check_suport_binary_operator(&binary_expression.operator.kind, &left) {
      self.runtime_error(
        &*binary_expression.operator,
        "Unsupported operator for left expression, expected a number or a string.",
      );
    }
    if !self.check_suport_binary_operator(&binary_expression.operator.kind, &right) {
      self.runtime_error(
        &*binary_expression.operator,
        "Unsupported operator, for right expression, expected a number or a string.",
      );
    }

    match (&binary_expression.operator.kind, &left, &right) {
      (TokenEnum::Plus, Value::Number(left), Value::Number(right)) => {
        return Value::create_number(*left + *right);
      }
      (TokenEnum::Plus, Value::String(left), Value::String(right)) => {
        return Value::create_string(left.to_owned() + right.as_str());
      }
      (TokenEnum::Minus, Value::Number(left), Value::Number(right)) => {
        return Value::create_number(*left - *right);
      }
      (TokenEnum::Star, Value::Number(left), Value::Number(right)) => {
        return Value::create_number(*left * *right);
      }
      (TokenEnum::Slash, Value::Number(left), Value::Number(right)) => {
        return Value::create_number(*left / *right);
      }
      (TokenEnum::Greater, Value::Number(left), Value::Number(right)) => {
        return Value::create_boolean(*left > *right);
      }
      (TokenEnum::Less, Value::Number(left), Value::Number(right)) => {
        return Value::create_boolean(*left < *right);
      }
      (TokenEnum::GreaterEqual, Value::Number(left), Value::Number(right)) => {
        return Value::create_boolean(*left >= *right);
      }
      (TokenEnum::LessEqual, Value::Number(left), Value::Number(right)) => {
        return Value::create_boolean(*left <= *right);
      }
      _ => unreachable!(),
    }
  }

  fn visit_unary_expression(&mut self, unary_expression: &ast::UnaryExpression) -> Value {
    let right = self.evaluate(&unary_expression.right);

    if unary_expression.operator.kind == TokenEnum::Minus && !right.is_number() {
      self.runtime_error(&*unary_expression.operator, "Unsupported operator, expected a number.");
    }
    match &unary_expression.operator.kind {
      TokenEnum::Bang => Value::create_boolean(!self.is_truthy(&right)),
      TokenEnum::Minus => Value::create_number(-right.as_number()),
      _ => unreachable!(),
    }
  }
  fn visit_grouping_expression(&mut self, grouping_expression: &ast::GroupingExpression) -> Value {
    self.evaluate(&grouping_expression.expression)
  }

  fn visit_literal(&mut self, literal: &ast::Literal) -> Value {
    match literal {
      ast::Literal::StringLiteral(string_literal) => Value::create_string(string_literal.clone()),
      ast::Literal::NumberLiteral(number_literal) => Value::create_number(*number_literal),
      ast::Literal::BooleanLiteral(boolean_literal) => Value::create_boolean(*boolean_literal),
      ast::Literal::NullLiteral => Value::create_nil(),
    }
  }

  fn visit_expression(&mut self, expression: &ast::Expression) -> Value {
    self.evaluate(expression)
  }
}

impl Evaluator {
  pub fn new(name: &str) -> Self {
    Evaluator { error_handler: ErrorHandler::new(name) }
  }
  pub fn evaluate(&mut self, ast: &ast::Expression) -> Value {
    ast.accept(self)
  }

  pub fn is_suport_operator(&self, operator: &TokenEnum, value: &Value) -> bool {
    match (operator, value) {
      (TokenEnum::Plus, Value::Number(_)) => true,
      (TokenEnum::Plus, Value::String(_)) => true,
      (TokenEnum::Minus, Value::Number(_)) => true,
      (TokenEnum::Star, Value::Number(_)) => true,
      (TokenEnum::Slash, Value::Number(_)) => true,
      (TokenEnum::Greater, Value::Number(_)) => true,
      (TokenEnum::Less, Value::Number(_)) => true,
      (TokenEnum::GreaterEqual, Value::Number(_)) => true,
      (TokenEnum::LessEqual, Value::Number(_)) => true,
      _ => false,
    }
  }

  pub fn is_equal(&self, left: &Value, right: &Value) -> bool {
    match (left, right) {
      (Value::Number(left), Value::Number(right)) => left == right,
      (Value::String(left), Value::String(right)) => left == right,
      (Value::Boolean(left), Value::Boolean(right)) => left == right,
      (Value::Nil, Value::Nil) => true,
      _ => false,
    }
  }

  pub fn check_suport_binary_operator(&self, operator: &TokenEnum, value: &Value) -> bool {
    match (operator, value) {
      (TokenEnum::Plus, Value::Number(_)) => true,
      (TokenEnum::Plus, Value::String(_)) => true,
      (TokenEnum::Minus, Value::Number(_)) => true,
      (TokenEnum::Star, Value::Number(_)) => true,
      (TokenEnum::Slash, Value::Number(_)) => true,
      (TokenEnum::Greater, Value::Number(_)) => true,
      (TokenEnum::Less, Value::Number(_)) => true,
      (TokenEnum::GreaterEqual, Value::Number(_)) => true,
      (TokenEnum::LessEqual, Value::Number(_)) => true,
      _ => false,
    }
  }

  pub fn check_suport_unary_operator(&self, operator: &TokenEnum, value: &Value) -> bool {
    match (operator, value) {
      (TokenEnum::Bang, Value::Boolean(_)) => true,
      (TokenEnum::Minus, Value::Number(_)) => true,
      (TokenEnum::Bang, Value::Number(_)) => true,
      _ => false,
    }
  }

  pub fn is_number(&self, value: &Value) -> bool {
    match value {
      Value::Number(_) => true,
      _ => false,
    }
  }
  pub fn is_string(&self, value: &Value) -> bool {
    match value {
      Value::String(_) => true,
      _ => false,
    }
  }

  fn is_truthy(&self, value: &Value) -> bool {
    match value {
      Value::Nil => false,
      Value::Boolean(boolean) => *boolean,
      Value::Number(number) => {
        if *number == 0.0 {
          return false;
        }
        true
      }
      Value::String(string) => string.len() > 0,
    }
  }
  pub fn runtime_error(&mut self, token: &Token, message: &str) {
    self.error_handler.error(token.line, message);
  }
}
