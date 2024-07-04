use std::fmt::{Display, Formatter};

pub enum Value {
  Nil,
  Boolean(bool),
  Number(f64),
  String(String),
}

impl Value {
  pub fn is_nil(&self) -> bool {
    match self {
      Value::Nil => true,
      _ => false,
    }
  }

  pub fn is_boolean(&self) -> bool {
    match self {
      Value::Boolean(_) => true,
      _ => false,
    }
  }
  pub fn is_number(&self) -> bool {
    match self {
      Value::Number(_) => true,
      _ => false,
    }
  }
  pub fn is_string(&self) -> bool {
    match self {
      Value::String(_) => true,
      _ => false,
    }
  }
  // create a new value

  // boolean
  pub fn create_boolean(boolean: bool) -> Value {
    Value::Boolean(boolean)
  }
  // number
  pub fn create_number(number: f64) -> Value {
    Value::Number(number)
  }
  // string
  pub fn create_string(string: String) -> Value {
    Value::String(string)
  }
  // nil
  pub fn create_nil() -> Value {
    Value::Nil
  }

  // helper methods
  pub fn is_truthy(&self) -> bool {
    match self {
      Value::Nil => false,
      Value::Boolean(boolean) => *boolean,
      Value::Number(number) => *number != 0.0,
      Value::String(string) => string.len() > 0,
    }
  }

  // getters
  pub fn as_number(&self) -> f64 {
    match self {
      Value::Number(number) => *number,
      _ => unreachable!(),
    }
  }
  pub fn as_string(&self) -> &str {
    match self {
      Value::String(string) => string.as_str(),
      _ => unreachable!(),
    }
  }
  pub fn as_boolean(&self) -> bool {
    match self {
      Value::Boolean(boolean) => *boolean,
      _ => unreachable!(),
    }
  }
}

// formant value

impl Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Nil => write!(f, "nil"),
      Value::Boolean(boolean) => write!(f, "{}", boolean),
      Value::Number(number) => {
        // if ends with .0 then remove it
        let mut number_string = format!("{}", number);
        if number_string.ends_with(".0") {
          number_string = number_string.trim_end_matches(".0").to_string();
        }
        write!(f, "{}", number_string)
      }
      Value::String(string) => write!(f, "\"{}\"", string),
    }
  }
}
