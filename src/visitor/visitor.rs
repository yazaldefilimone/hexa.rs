use crate::ast;

pub trait Visitor<T> {
  fn visit(&mut self, ast: &ast::Expression) -> T;

  fn visit_binary_expression(&mut self, binary_expression: &ast::BinaryExpression) -> T;

  fn visit_unary_expression(&mut self, unary_expression: &ast::UnaryExpression) -> T;

  fn visit_grouping_expression(&mut self, grouping_expression: &ast::GroupingExpression) -> T;

  fn visit_literal(&mut self, literal: &ast::Literal) -> T;

  fn visit_expression(&mut self, expression: &ast::Expression) -> T;
}
