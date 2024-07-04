use crate::visitor::Visitor;

use super::tokens::{Token, TokenEnum};

pub enum Expression {
  BinaryExpression(BinaryExpression),
  UnaryExpression(UnaryExpression),
  GroupingExpression(GroupingExpression),
  Literal(Literal),
}

impl Expression {
  pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
    match self {
      Expression::BinaryExpression(binary_expression) => visitor.visit_binary_expression(binary_expression),
      Expression::UnaryExpression(unary_expression) => visitor.visit_unary_expression(unary_expression),
      Expression::GroupingExpression(grouping_expression) => visitor.visit_grouping_expression(grouping_expression),
      Expression::Literal(literal) => visitor.visit_literal(literal),
    }
  }
}

pub struct BinaryExpression {
  pub operator: Box<Token>,
  pub left: Box<Expression>,
  pub right: Box<Expression>,
}

impl BinaryExpression {
  pub fn new(operator: Box<Token>, left: Expression, right: Expression) -> BinaryExpression {
    BinaryExpression { operator, left: Box::new(left), right: Box::new(right) }
  }

  pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
    visitor.visit_binary_expression(self)
  }
}

pub struct UnaryExpression {
  pub operator: Box<Token>,
  pub right: Box<Expression>,
}

impl UnaryExpression {
  pub fn new(operator: Box<Token>, right: Expression) -> UnaryExpression {
    UnaryExpression { operator, right: Box::new(right) }
  }

  pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
    visitor.visit_unary_expression(self)
  }
}

pub struct GroupingExpression {
  pub expression: Box<Expression>,
}

impl GroupingExpression {
  pub fn new(expression: Expression) -> GroupingExpression {
    GroupingExpression { expression: Box::new(expression) }
  }

  pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
    visitor.visit_grouping_expression(self)
  }
}

pub enum Literal {
  StringLiteral(String),
  NumberLiteral(f64),
  BooleanLiteral(bool),
  NullLiteral,
}

impl Literal {
  pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
    visitor.visit_literal(self)
  }
}
