use super::tokens::TokenEnum;

pub enum Expression {
  BinaryExpression(BinaryExpression),
  UnaryExpression(UnaryExpression),
  GroupingExpression(GroupingExpression),
  Literal(Literal),
}

pub struct BinaryExpression {
  pub operator: TokenEnum,
  pub left: Box<Expression>,
  pub right: Box<Expression>,
}

impl BinaryExpression {
  pub fn new(operator: TokenEnum, left: Expression, right: Expression) -> BinaryExpression {
    BinaryExpression { operator, left: Box::new(left), right: Box::new(right) }
  }
}

pub struct UnaryExpression {
  pub operator: TokenEnum,
  pub right: Box<Expression>,
}

impl UnaryExpression {
  pub fn new(operator: TokenEnum, right: Expression) -> UnaryExpression {
    UnaryExpression { operator, right: Box::new(right) }
  }
}

pub struct GroupingExpression {
  pub expression: Box<Expression>,
}

impl GroupingExpression {
  pub fn new(expression: Expression) -> GroupingExpression {
    GroupingExpression { expression: Box::new(expression) }
  }
}

pub enum Literal {
  StringLiteral(String),
  NumberLiteral(f64),
  BooleanLiteral(bool),
  NullLiteral,
}
