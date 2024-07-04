use crate::ast;

use super::Visitor;

pub struct PrintVisitor;

impl Visitor<String> for PrintVisitor {
  fn visit(&mut self, node: &ast::Expression) -> String {
    self.visit_expression(node)
  }
  fn visit_binary_expression(&mut self, binary_expression: &ast::BinaryExpression) -> String {
    binary_expression.left.accept(self)
  }

  fn visit_unary_expression(&mut self, unary_expression: &ast::UnaryExpression) -> String {
    let mut result = String::from("");
    let operator_lexeme = &unary_expression.operator.lexeme;
    result = format!("{}{}", operator_lexeme, unary_expression.right.accept(self));
    result
  }

  fn visit_grouping_expression(&mut self, grouping_expression: &ast::GroupingExpression) -> String {
    let mut result = String::from("");
    result = format!("({})", grouping_expression.expression.accept(self));
    return result;
  }

  fn visit_literal(&mut self, literal: &ast::Literal) -> String {
    let mut result = String::from("");
    match literal {
      ast::Literal::StringLiteral(string_literal) => {
        result = format!("{}", string_literal);
      }
      ast::Literal::NumberLiteral(number_literal) => {
        result = format!("{}", number_literal);
      }
      ast::Literal::BooleanLiteral(boolean_literal) => {
        result = format!("{}", boolean_literal);
      }
      ast::Literal::NullLiteral => {
        result = format!("null");
      }
    }
    return result;
  }

  fn visit_expression(&mut self, expression: &ast::Expression) -> String {
    match expression {
      ast::Expression::BinaryExpression(binary_expression) => self.visit_binary_expression(binary_expression),
      ast::Expression::UnaryExpression(unary_expression) => self.visit_unary_expression(unary_expression),
      ast::Expression::GroupingExpression(grouping_expression) => self.visit_grouping_expression(grouping_expression),
      ast::Expression::Literal(literal) => self.visit_literal(literal),
    }
  }
}
